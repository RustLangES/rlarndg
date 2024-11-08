use actix_web::{App, HttpServer, Scope};
use flexi_logger::{Logger, FlexiLoggerError};
use helpers::logging::format_colored_log;
use routes::values::{random_bool, random_color, random_signed, random_unsigned};
use tokio::main;
use thiserror::Error;
use std::io::Error as IoError;

mod helpers;
mod routes;

// TODO: implement API keys, a MASTER_PASSWORD environment variable
// should exist, the header should be distinct between password and keys
// the value format could be "token XXXXX" or "password XXXXXX"
// if the password or token attempted is invalid a 60 second
// ratelimit should be applied, otherwise, requests without
// an api key should be able to do 1 request every 10 seconds,
// requests with api key should be able to do non ratelimited
// requests. (if many api keys are bought a server extension should be considered)

#[derive(Debug, Error)]
enum AppError {
    #[error("{0:#}")]
    Io(#[from] IoError),

    #[error("{0:#}")]
    Logger(#[from] FlexiLoggerError)
}

#[main]
async fn main() -> Result<(), AppError> {
    Logger::try_with_str("info")?
        .format_for_stdout(format_colored_log)
        .log_to_stdout()
        .start()?;

    HttpServer::new(|| {
        App::new()
            .service(
                Scope::new("/random")
                    .service(random_unsigned)
                    .service(random_signed)
                    .service(random_bool)
                    .service(random_color)
            )
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await?;

    Ok(())
}
