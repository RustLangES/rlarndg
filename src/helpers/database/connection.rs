use std::sync::OnceLock;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres, Error as SqlxError};
use thiserror::Error;
use dotenv::{var, Error as VarError};

static CONNECTION: OnceLock<Pool<Postgres>> = OnceLock::new();

#[derive(Error, Debug)]
pub enum DbConnectionError {
    #[error("The DATABASE_URL environment variable couldn't be found.")]
    EnvironmentError(#[from] VarError),

    #[error("{0:#}")]
    ConnectionError(#[from] SqlxError)
}

pub async fn get_db_connection<'r>() -> Result<&'r Pool<Postgres>, DbConnectionError> {
    if let Some(connection) = CONNECTION.get() {
        return Ok(connection);
    }

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&var("DATABASE_URL")?)
        .await?;

    Ok(CONNECTION.get_or_init(|| pool))
}

#[macro_export]
macro_rules! db {
    () => {
        $crate::helpers::database::connection::get_db_connection().await?
    };
}
