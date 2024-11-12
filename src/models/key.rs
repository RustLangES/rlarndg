use rand::{distributions::Alphanumeric, thread_rng, Rng};
use serde::Serialize;
use serde_json::{to_string, Error as JsonError};
use sqlx::{query_as, Error as SqlxError};
use time::OffsetDateTime;
use crate::{db, helpers::database::DbConnectionError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiKeyError {
    #[error("Query: {0:#}")]
    Query(#[from] SqlxError),

    #[error("Connection: {0:#}")]
    Connection(#[from] DbConnectionError),

    #[error("Json: {0:#}")]
    Json(#[from] JsonError)
}

#[derive(Serialize)]
pub struct ApiKey {
    id: i32,
    user_id: i32,
    token: String,
    paid: f64,
    created_at: OffsetDateTime
}

impl ApiKey {
    pub async fn new(user_id: i32, paid: f64) -> Result<Self, ApiKeyError> {
        let key = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(100)
            .map(char::from)
            .collect::<String>();

        let key = query_as!(
            Self,
            r#"
                INSERT INTO keys (user_id, token, paid)
                VALUES ($1, $2, $3)
                RETURNING id, user_id, token, paid, created_at
            "#,
            user_id,
            key,
            paid
        )
            .fetch_one(db!())
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

    pub fn to_string(&self) -> Result<String, ApiKeyError> {
        Ok(to_string(&self)?)
    }
}
