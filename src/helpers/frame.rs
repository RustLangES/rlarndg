use std::{collections::{HashMap, HashSet}, env::args, fs::read_to_string, io::Error as IoError, str::FromStr, sync::{LazyLock, OnceLock}};
use actix_web::web::Bytes;
use log::{info, warn};
use reqwest::{header::{HeaderMap, HeaderName, HeaderValue, InvalidHeaderValue, InvalidHeaderName}, Client, Error as RequestError};
use serde::Deserialize;
use serde_json::{from_str, Error as JsonError};
use thiserror::Error;
use tokio::sync::Mutex;

#[macro_export]
macro_rules! frame_bytes {
    () => {
        match $crate::helpers::frame::frame_bytes_from_source().await {
            Ok(bytes) => bytes,
            Err(err) => {
                return actix_web::HttpResponse::InternalServerError()
                    .body(format!("{err:#}"));
            }
        }
    };
}

#[derive(Debug, Error)]
pub enum CaptureError<'s> {
    #[error("{0:#}")]
    Request(#[from] RequestError),

    #[error("An empty playlist or invalid response was received from the server.")]
    InvalidResponse,

    #[error("{0:#}")]
    SourceError(&'s SourceError),

    #[error("{0:#}")]
    InvalidHeaderName(#[from] InvalidHeaderName),

    #[error("{0:#}")]
    InvalidHeaderValue(#[from] InvalidHeaderValue)
}

#[derive(Debug, Error)]
pub enum SourceError {
    #[error("{0:#}")]
    Parse(#[from] JsonError),

    #[error("{0:#}")]
    Io(#[from] IoError)
}

#[derive(Deserialize)]
struct Source {
    source: String,
    headers: Option<HashMap<String, String>>
}

#[derive(Default)]
struct SourceState {
    source_index: usize,
    values: Vec<u32>
}

static SOURCES: OnceLock<Result<Vec<Source>, SourceError>> = OnceLock::new();
static STATE: LazyLock<Mutex<SourceState>> = LazyLock::new(|| Mutex::new(SourceState::default()));

pub async fn frame_bytes_from_source<'r>() -> Result<Bytes, CaptureError<'r>> {
    let sources = SOURCES.get_or_init(|| {
        let args = args()
            .zip(args().skip(1))
            .filter(|(arg, _)| arg == "--source")
            .map(|(_, val)| val)
            .collect::<Vec<String>>();

        let mut sources = Vec::new();

        for source in args {
            let parsed_sources = read_to_string(&source)
                .and_then(|source| Ok(from_str::<Vec<Source>>(&source)?));

            match parsed_sources {
                Ok(parsed) => {
                    info!("Loaded {source} successfully.");

                    sources.extend(parsed);
                },
                Err(error) => {
                    warn!("Couldn't load source at {source}, an error occurred: {error:#}");
                }
            }
        }

        Ok(sources)
    })
        .as_ref()
        .map_err(CaptureError::SourceError)?;

    let mut state = STATE
        .lock()
        .await;

    if
        state.values.len() >= 5
        && state.values.iter().collect::<HashSet<_>>().len() != state.values.len() {

        state.source_index += 1;
    }

    if sources.len() >= state.source_index {
        state.source_index = 0;
    }

    let source = &sources[state.source_index];

    frame_bytes(&source.source, &source.headers).await
}

pub async fn frame_bytes<'p, 'r>(source: &'p String, source_headers: &'p Option<HashMap<String, String>>)
    -> Result<Bytes, CaptureError<'r>> {

    let http_client = Client::new();
    let mut headers = HeaderMap::new();

    if let Some(source_headers) = source_headers {
        for (name, value) in source_headers {
            headers.append(
                HeaderName::from_str(name)?,
                HeaderValue::from_str(value)?
            );
        }
    }

    let result = http_client
        .get(source)
        .headers(headers.clone())
        .send()
        .await?
        .text()
        .await?;

    let mut url = result
        .split('\n')
        .find(|line| !line.starts_with('#'))
        .ok_or(CaptureError::InvalidResponse)?
        .to_string();

    if !url.starts_with("http") {
        let mut source = source
            .split('/')
            .collect::<Vec<&str>>();

        source.pop();
        source.push(&url);

        url = source.join("/");
    }

    Ok(
        http_client
            .get(url)
            .headers(headers)
            .send()
            .await?
            .bytes()
            .await?
    )
}
