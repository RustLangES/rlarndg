use actix_web::{get, http::header::ContentType, web::Query, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;
use serde_json::to_string;
use crate::{helpers::stripe::create_stripe_payment, models::{key::ApiKey, user::User}};

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

#[derive(Deserialize)]
struct PaymentQuery {
    amount: f32
}

#[get("/checkout")]
pub async fn pay_new_key(user: User, req: HttpRequest, query: Query<PaymentQuery>) -> impl Responder {
    let callback = {
        let connection_info = req
            .connection_info();

        format!(
            "{}://{}",
            connection_info.scheme(),
            connection_info.host()
        )
    };

    let jwt = match user.jwt() {
        Ok(jwt) => jwt,
        Err(err) => {
            return HttpResponse::InternalServerError()
                .body(err.to_string());
        }
    };

    let payment_url = match create_stripe_payment(&callback, &jwt, query.amount).await {
        Ok(url) => url,
        Err(err) => {
            return HttpResponse::InternalServerError()
                .body(err.to_string());
        }
    };

    HttpResponse::PermanentRedirect()
        .insert_header(("Location", payment_url))
        .finish()
}
