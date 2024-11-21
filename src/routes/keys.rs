use actix_web::{get, web::Query, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;
use serde_json::to_string;
use crate::{grv, helpers::misc::stripe::create_stripe_payment, models::{key::ApiKey, user::User}};

#[get("/user")]
pub async fn get_key_ids(user: User) -> impl Responder {
    HttpResponse::Ok()
        .json(
            grv!(to_string(
                &grv!(ApiKey::user_keys(user.id).await)
            ))
        )
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

    HttpResponse::PermanentRedirect()
        .insert_header((
            "Location",
            grv!(create_stripe_payment(&callback, &grv!(user.jwt()), query.amount).await)
        ))
        .finish()
}
