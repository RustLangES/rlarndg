use rand::{distributions::Alphanumeric, thread_rng, Rng};
use serde::Serialize;
use serde_json::Error as JsonError;
use sqlx::{query, query_as, Error as SqlxError};
use time::OffsetDateTime;
use crate::{db, helpers::database::connection::DbConnectionError};
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
    stripe_session_id: String,
    created_at: OffsetDateTime
}

impl ApiKey {
    pub async fn new(user_id: i32, session_id: &String, paid: f64) -> Result<Self, ApiKeyError> {
        let key = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(100)
            .map(char::from)
            .collect::<String>();

        let key = query_as!(
            Self,
            r#"
                INSERT INTO keys (user_id, token, paid, stripe_session_id)
                VALUES ($1, $2, $3, $4)
                RETURNING *
            "#,
            user_id,
            key,
            paid,
            session_id
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
