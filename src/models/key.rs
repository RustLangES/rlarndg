use std::{collections::HashMap, future::{ready, Future}, pin::Pin, sync::{Mutex, OnceLock}};
use actix_web::{dev::Payload, error::{ErrorBadRequest, ErrorInternalServerError, ErrorTooManyRequests, ErrorUnauthorized}, http::header::ToStrError, Error as ActixWebError, FromRequest, HttpRequest};
use bcrypt::{hash, BcryptError, DEFAULT_COST};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use serde::Serialize;
use serde_json::Error as JsonError;
use sqlx::{query, query_as, Error as SqlxError};
use time::{ext::NumericalDuration, OffsetDateTime};
use tokio::runtime::Runtime;
use crate::{db, helpers::database::connection::DbConnectionError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiKeyError {
    #[error("Query: {0:#}")]
    Query(#[from] SqlxError),

    #[error("Connection: {0:#}")]
    Connection(#[from] DbConnectionError),

    #[error("Json: {0:#}")]
    Json(#[from] JsonError),

    #[error("Bcrypt {0:#}")]
    Bcrypt(#[from] BcryptError),

    #[error("ToStr {0:#}")]
    ToString(ToStrError)
}

pub enum MaybeApiKey {
    Authorized(ApiKey),
    Unauthorized
}

#[derive(Serialize)]
pub struct ApiKey {
    pub id: i32,
    pub user_id: i32,
    pub token: String,
    pub paid: f64,
    pub stripe_session_id: String,
    pub created_at: OffsetDateTime
}

impl ApiKey {
    fn generate() -> String {
        thread_rng()
            .sample_iter(&Alphanumeric)
            .take(100)
            .map(char::from)
            .collect::<String>()
    }

    pub async fn new(user_id: i32, session_id: &String, paid: f64) -> Result<Self, ApiKeyError> {
        let key = query_as!(
            Self,
            r#"
                INSERT INTO keys (user_id, token, paid, stripe_session_id)
                VALUES ($1, $2, $3, $4)
                RETURNING *
            "#,
            user_id,
            hash(Self::generate(), DEFAULT_COST)?,
            paid,
            session_id
        )
            .fetch_one(db!())
            .await?;

        Ok(key)
    }

    pub async fn from_key(key: String) -> Result<Option<Self>, ApiKeyError> {
        let key = hash(key, DEFAULT_COST)?;

        Ok(query_as!(
            Self,
            r#"
                SELECT *
                FROM keys
                WHERE token = $1
            "#,
            key
        )
            .fetch_optional(db!())
            .await?)
    }

    pub async fn reset_key(user_id: i32, key_id: i32) -> Result<String, ApiKeyError> {
        let key = Self::generate();

        query!(
            r#"
                UPDATE keys
                SET token = $3
                WHERE user_id = $1 AND id = $2
            "#,
            user_id,
            key_id,
            hash(&key, DEFAULT_COST)?
        )
            .execute(db!())
            .await?;

        Ok(key)
    }

    pub async fn user_keys(user_id: i32) -> Result<Vec<Self>, ApiKeyError> {
        let keys = query_as!(
            Self,
            r#"
                SELECT *
                FROM keys
                WHERE user_id = $1
            "#,
            user_id
        )
            .fetch_all(db!())
            .await?;

        Ok(keys)
    }

    pub async fn fetch_exists(stripe_session_id: &String) -> Result<bool, ApiKeyError> {
        query!(
            r#"
                SELECT EXISTS(
                    SELECT true
                    FROM keys
                    WHERE stripe_session_id = $1
                )
            "#,
            stripe_session_id
        )
            .fetch_one(db!())
            .await?
            .exists
            .map_or(Ok(false), |v| Ok(v))
    }
}

impl Into<Option<ApiKey>> for MaybeApiKey {
    fn into(self) -> Option<ApiKey> {
        match self {
            Self::Authorized(key) => Some(key),
            Self::Unauthorized => None
        }
    }
}

static RATE_LIMIT_DICT: OnceLock<Mutex<HashMap<String, OffsetDateTime>>>
    = OnceLock::new();

impl FromRequest for MaybeApiKey {
    type Error = ActixWebError;

    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let runtime = Runtime::new()
            .unwrap();

        if let Some(value) = req.headers().get("Authorization") {
            let value = value
                .to_str()
                .map(|value| runtime.block_on(
                    async move { ApiKey::from_key(value.to_string()).await }
                ))
                .map_err(|err| ApiKeyError::ToString(err));

            let value = match value {
                Ok(Ok(Some(v))) => Ok(MaybeApiKey::Authorized(v)),
                Ok(Ok(None)) => Err(ErrorUnauthorized("The provided key is not valid.")),
                Ok(Err(e)) | Err(e) => Err(ErrorInternalServerError(format!("{e:#}")))
            };

            return Box::pin(ready(value));
        };

        let ip = match req.peer_addr() {
            Some(addr) => addr.to_string(),
            None => {
                return Box::pin(
                    ready(Err(
                        ErrorBadRequest("Could not retrieve peer address.")
                    ))
                );
            }
        };

        let mut rl_dict = RATE_LIMIT_DICT.get_or_init(|| Mutex::new(HashMap::new()))
            .lock()
            .unwrap(); // should never ever panic!.

        if let Some(time) = rl_dict.get(&ip) {
            let now = OffsetDateTime::now_utc();

            if time <= &now {
                return Box::pin(
                    ready(Err(
                        ErrorTooManyRequests(format!(
                            "
                            Too many requests, you will be able to make
                            a request again in {} seconds, unless you provide an api key.
                            ",
                            (*time - now).whole_seconds()
                        ))
                    ))
                );
            }
        }

        rl_dict
            .insert(ip, OffsetDateTime::now_utc() + 30i64.seconds());

        Box::pin(ready(Ok(Self::Unauthorized)))
    }
}
