use std::{collections::HashMap, future::{ready, Future}, pin::Pin, sync::Mutex};
use actix_web::{dev::Payload, error::{ErrorBadRequest, ErrorInternalServerError, ErrorTooManyRequests, ErrorUnauthorized}, http::header::ToStrError, Error as ActixWebError, FromRequest, HttpRequest};
use bcrypt::{hash, verify, BcryptError, DEFAULT_COST};
use lazy_static::lazy_static;
use log::debug;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use serde::Serialize;
use serde_json::Error as JsonError;
use sqlx::{query, query_as, Error as SqlxError};
use time::{ext::NumericalDuration, OffsetDateTime};
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
            .take(30)
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
        let keys = query_as!(
            Self,
            r#"
                SELECT *
                FROM keys
            "#
        )
            .fetch_all(db!())
            .await?;

        for db_key in keys {
            if verify(&key, &db_key.token)? {
                return Ok(Some(db_key));
            }
        }

        Ok(None)
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

lazy_static! {
    static ref RATE_LIMIT_DICT: Mutex<HashMap<String, OffsetDateTime>>
        = Mutex::new(HashMap::new());
}

impl FromRequest for MaybeApiKey {
    type Error = ActixWebError;

    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        if let Some(value) = req.headers().get("Authorization") {
            let key = match value.to_str() {
                Ok(key) => key.to_string(),
                Err(err) => {
                    return Box::pin(ready(
                        Err(ErrorInternalServerError(format!("{err:#}")))
                    ));
                }
            };

            return Box::pin(async {
                match ApiKey::from_key(key).await {
                    Ok(Some(key)) => Ok(Self::Authorized(key)),
                    Ok(None) => Err(ErrorUnauthorized("The provided key is not valid")),
                    Err(err) => Err(ErrorInternalServerError(format!("{err:#}")))
                }
            });
        };

        let ip = match req.peer_addr() {
            Some(addr) => addr.ip().to_string(),
            None => {
                return Box::pin(
                    ready(Err(
                        ErrorBadRequest("Could not retrieve peer address.")
                    ))
                );
            }
        };

        let mut rl_dict = RATE_LIMIT_DICT
            .lock()
            .unwrap();
        let now = OffsetDateTime::now_utc();

        debug!("Current time: {now}");

        if let Some(time) = rl_dict.get(&ip) {
            if now < *time {
                debug!("Ratelimit for {ip} expires at {time}");

                return Box::pin(
                    ready(Err(
                        ErrorTooManyRequests(format!(
                            "
                            \rToo many requests, you will be able to make
                            \ra request again in {} seconds, unless you provide an api key.
                            ",
                            (*time - now).whole_seconds()
                        ))
                    ))
                );
            }
        }

        let ratelimit = now + 30i64.seconds();

        debug!("Updating ratelimit for IP {ip} setting expiration to {ratelimit}");

        rl_dict.insert(ip, ratelimit);

        Box::pin(ready(Ok(Self::Unauthorized)))
    }
}
