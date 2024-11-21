use std::time::Duration;
use actix_web::{cookie::Cookie, get, http::header::ContentType, post, web::Json, HttpResponse, Responder};
use serde::Deserialize;
use time::OffsetDateTime;
use crate::{gov, grv, models::user::{User, UserError}};

#[derive(Deserialize)]
struct LoginInfo {
    email: String,
    password: String
}

fn create_auth_cookie(jwt: &String) -> Cookie {
    let mut cookie = Cookie::new("auth", jwt);

    cookie.set_path("/");
    cookie.set_expires(OffsetDateTime::now_utc() + Duration::from_secs(10800));

    cookie
}

#[post("/login")]
pub async fn login(info: Json<LoginInfo>) -> impl Responder {
    let LoginInfo {email, password} = info.into_inner();

    let user = gov!(
        grv!(User::login(email, password).await),
        "User not found, check the email or password and try again."
    );

    HttpResponse::Ok()
        .cookie(create_auth_cookie(&grv!(user.jwt())))
        .json(grv!(user.to_string()))
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

    HttpResponse::Ok()
        .cookie(create_auth_cookie(&grv!(user.jwt())))
        .json(grv!(user.to_string()))
}


#[get("/user")]
pub async fn get_user(user: User) -> impl Responder {
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(grv!(user.to_string()))
}
