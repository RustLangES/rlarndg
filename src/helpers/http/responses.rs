use actix_web::{body::BoxBody, http::header::ContentType, HttpResponse};
use serde::Serialize;
use serde_json::to_string;
use time::OffsetDateTime;
use crate::models::key::ApiKey;

#[derive(Serialize)]
pub struct TimedResponse<T: Serialize> {
    author: Option<i32>,
    timestamp: i64,
    value: T
}

impl<T: Serialize> TimedResponse<T> {
    pub fn new(value: T, key: Option<ApiKey>) -> Self {
        Self {
            author: key.map(|key| key.user_id),
            timestamp: OffsetDateTime::now_utc().unix_timestamp(),
            value
        }
    }
}

impl<T: Serialize> From<TimedResponse<T>> for HttpResponse<BoxBody> {
    fn from(val: TimedResponse<T>) -> Self {
        match to_string(&val) {
            Ok(json) => HttpResponse::Ok()
                .content_type(ContentType::json())
                .body(json),
            Err(error) => HttpResponse::InternalServerError()
                .content_type(ContentType::plaintext())
                .body(format!("{error:#}")),
        }
    }
}
