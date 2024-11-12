
use std::time::Duration;

use actix_web::{cookie::Cookie, get, http::header::ContentType, post, web::Json, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;
use time::OffsetDateTime;
use crate::models::user::{User, UserError};

#[derive(Deserialize)]
struct LoginInfo {
    email: String,
    password: String
}

#[post("/login")]
pub async fn login(info: Json<LoginInfo>) -> impl Responder {
    let LoginInfo {email, password} = info.into_inner();

    let user = match User::login(email, password).await {
        Ok(user) => user,
        Err(err) => {
            return HttpResponse::InternalServerError()
                .body(err.to_string());
        }
    };

    let user = match user {
        Some(user) => user,
        None => {
            return HttpResponse::NotFound()
                .body("User not found, check the email or password and try again.");
        }
    };

    let jwt = match user.jwt() {
        Ok(jwt) => jwt,
        Err(err) => {
            return HttpResponse::InternalServerError()
                .body(err.to_string());
        }
    };

    let json = match user.to_string() {
        Ok(json) => json,
        Err(err) => {
            return HttpResponse::InternalServerError()
                .body(err.to_string());
        }
    };

    let mut cookie = Cookie::new("auth", jwt);

    cookie.set_path("/");
    cookie.set_expires(OffsetDateTime::now_utc() + Duration::from_secs(10800));

    HttpResponse::Ok()
        .cookie(cookie)
        .content_type(ContentType::json())
        .body(json)
}

#[post("/signup")]
pub async fn signup(info: Json<LoginInfo>) -> impl Responder {
    let LoginInfo { email, password } = info.into_inner();

    let user = match User::new(email, password).await {
        Ok(user) => user,
        Err(err) => match err {
            UserError::EmailConflict(_) => {
                return HttpResponse::BadRequest()
                    .body(err.to_string());
            },
            _ => {
                return HttpResponse::InternalServerError()
                    .body(err.to_string());
            }
        }
    };

    let jwt = match user.jwt() {
        Ok(jwt) => jwt,
        Err(err) => {
            return HttpResponse::InternalServerError()
                .body(err.to_string());
        }
    };

    let json = match user.to_string() {
        Ok(json) => json,
        Err(err) => {
            return HttpResponse::InternalServerError()
                .body(err.to_string());
        }
    };

    let mut cookie = Cookie::new("auth", jwt);

    cookie.set_path("/");
    cookie.set_expires(OffsetDateTime::now_utc() + Duration::from_secs(10800));

    HttpResponse::Ok()
        .cookie(cookie)
        .content_type(ContentType::json())
        .body(json)
}

// TODO: make this use a request collector

#[get("/user")]
pub async fn get_user(req: HttpRequest) -> impl Responder {
    let user = match req.cookie("auth") {
        Some(cookie) => User::from_jwt(cookie.value().to_string())
            .await,
        None => {
            return HttpResponse::BadRequest()
                .body("No auth cookie was found.");
        }
    };

    match user.and_then(|u| u.to_string()) {
        Ok(user) => HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(user),
        Err(err) => HttpResponse::InternalServerError()
            .body(err.to_string())
    }
}
