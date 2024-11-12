use actix_web::{App, HttpServer, Scope};
use flexi_logger::{Logger, FlexiLoggerError};
use helpers::logging::format_colored_log;
use routes::{auth::{get_user, login, signup}, keys::get_key_ids, values::{random_bool, random_color, random_signed, random_unsigned}};
use tokio::main;
use thiserror::Error;
use std::io::Error as IoError;

mod helpers;
mod routes;
mod models;

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
            .service(
                Scope::new("/auth")
                    .service(login)
                    .service(signup)
                    .service(get_user)
            )
            .service(
                Scope::new("/keys")
                    .service(get_key_ids)
            )
    })
        .bind(("127.0.0.1", 5174))?
        .run()
        .await?;

    Ok(())
}
