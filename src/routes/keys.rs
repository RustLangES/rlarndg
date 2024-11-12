use actix_web::{get, http::header::ContentType, HttpResponse, Responder};
use serde_json::to_string;
use crate::models::{user::User, key::ApiKey};

#[get("/user")]
pub async fn get_key_ids(user: User) -> impl Responder {
    let keys = match ApiKey::user_keys(user.id).await {
        Ok(keys) => keys,
        Err(err) => {
            return HttpResponse::InternalServerError()
                .body(err.to_string());
        }
    };

    match to_string(&keys) {
        Ok(keys) => HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(keys),
        Err(err) =>
            HttpResponse::InternalServerError()
                .body(err.to_string())
    }
}
