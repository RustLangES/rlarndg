use std::{future::{ready, Future}, pin::Pin, time::Duration};
use actix_web::{dev::Payload, error::ErrorUnauthorized, Error as ActixWebError, FromRequest, HttpRequest};
use bcrypt::{hash, verify, BcryptError, DEFAULT_COST};
use serde::{Deserialize, Serialize};
use serde_json::{to_string, Error as JsonError};
use sqlx::{query, query_as, Error as SqlxError};
use thiserror::Error;
use time::OffsetDateTime;
use jsonwebtoken::{decode, encode, errors::Error as JwtError, Header, Validation};
use crate::{db, helpers::database::DbConnectionError, jwt_hash};

#[derive(Error, Debug)]
pub enum UserError {
    #[error("Query: {0:#}")]
    Query(#[from] SqlxError),

    #[error("Connection: {0:#}")]
    Connection(#[from] DbConnectionError),

    #[error("Hash: {0:#}")]
    Hash(#[from] BcryptError),

    #[error("Json: {0:#}")]
    Json(#[from] JsonError),

    #[error("The email `{0}` already exists")]
    EmailConflict(String),

    #[error("Jwt: {0:#}")]
    Jwt(#[from] JwtError)
}

#[derive(Debug, Serialize, Deserialize)]
struct UserClaims {
    exp: usize,

    id: i32,
    email: String,
    password: String,
    created_at: OffsetDateTime
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
    #[serde(skip_serializing)]
    pub created_at: OffsetDateTime
}

impl User {
    pub async fn new(email: String, password: String) -> Result<Self, UserError> {
        let user = query!(
            "
                SELECT EXISTS (
                    SELECT 1
                    FROM users
                    WHERE email = $1
                )
            ",
            email
        )
            .fetch_one(db!())
            .await?;

        if user.exists.unwrap_or(false) {
            return Err(UserError::EmailConflict(email));
        }

        Ok(query_as!(
            Self,
            "
                INSERT INTO users (email, password)
                VALUES ($1, $2)
                RETURNING *
            ",
            email,
            hash(password, DEFAULT_COST)?
        )
            .fetch_one(db!())
            .await?)
    }

    pub async fn login(email: String, password: String) -> Result<Option<Self>, UserError> {
        let user = query_as!(
            Self,
            "
                SELECT *
                FROM users
                WHERE email = $1
            ",
            email
        )
            .fetch_optional(db!())
            .await?;

        match user {
            Some(user) if verify(password, &user.password)? => Ok(Some(user)),
            _ => Ok(None)
        }
    }

    pub fn from_jwt(token: String) -> Result<Self, UserError> {
        Ok(
            decode::<UserClaims>(
                &token,
                jwt_hash!(decode),
                &Validation::default()
            )?
                .claims
                .into()
        )
    }

    pub fn jwt(&self) -> Result<String, JwtError> {
        encode(
            &Header::default(),
            &UserClaims::from(self),
            jwt_hash!(encode)
        )
    }

    pub fn to_string(&self) -> Result<String, UserError> {
        Ok(to_string(&self)?)
    }
}

impl From<UserClaims> for User {
    fn from(UserClaims {id, email, password, created_at, ..}: UserClaims) -> Self {
        Self {
            id,
            email,
            password,
            created_at
        }
    }
}

impl UserClaims {
    pub fn new(id: i32, email: String, password: String, created_at: OffsetDateTime) -> Self {
        Self {
            exp: (OffsetDateTime::now_utc() + Duration::from_secs(10800))
                .unix_timestamp() as usize,

            id,
            email,
            password,
            created_at
        }
    }
}

impl From<&User> for UserClaims {
    fn from(User {id, email, password, created_at}: &User) -> Self {
        Self::new(*id, email.clone(), password.clone(), *created_at)
    }
}

impl FromRequest for User {
    type Error = ActixWebError;

    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        Box::pin(ready(
            req.cookie("auth")
                .and_then(|cookie|
                    User::from_jwt(
                        cookie
                            .value()
                            .to_string()
                    )
                        .ok()
                )
                    .ok_or(
                        ErrorUnauthorized("Missing or invalid auth cookie for this endpoint.")
                    )
        ))
    }
}
