use actix_web::{body::BoxBody, http::header::ContentType, HttpResponse};
use serde::Serialize;
use serde_json::to_string;
use time::OffsetDateTime;

#[derive(Serialize)]
pub struct TimedResponse<T: Serialize> {
    author: Option<u8>, // TODO: implement api keys
    timestamp: i64,
    value: T
}

impl<T: Serialize> TimedResponse<T> {
    pub fn new(value: T) -> Self {
        Self {
            author: None,
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
