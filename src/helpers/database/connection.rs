use sqlx::{
    migrate::{MigrateError, Migrator},
    postgres::PgPoolOptions,
    Error as SqlxError, Pool, Postgres,
};
use std::path::Path;
use std::sync::OnceLock;
use thiserror::Error;

static CONNECTION: OnceLock<Pool<Postgres>> = OnceLock::new();

#[derive(Error, Debug)]
pub enum DbConnectionError {
    #[error("{0:#}")]
    Connection(#[from] SqlxError),

    #[error("{0:#}")]
    Migrate(#[from] MigrateError),
}

pub async fn get_db_connection<'r>() -> Result<&'r Pool<Postgres>, DbConnectionError> {
    if let Some(connection) = CONNECTION.get() {
        return Ok(connection);
    }

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&lc!(env!("DATABASE_URL")))
        .await?;

    Migrator::new(Path::new("./migrations"))
        .await?
        .run(&pool)
        .await?;

    Ok(CONNECTION.get_or_init(|| pool))
}

#[macro_export]
macro_rules! db {
    () => {
        $crate::helpers::database::connection::get_db_connection().await?
    };
}
