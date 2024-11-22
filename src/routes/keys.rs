use actix_web::{get, post, web::Query, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;
use crate::{grv, helpers::misc::stripe::{create_stripe_payment, verify_payment}, models::{key::ApiKey, user::User}};

fn get_callback(req: &HttpRequest) -> String {
    let connection_info = req
        .connection_info();

    format!(
        "{}://{}",
        connection_info.scheme(),
        connection_info.host()
    )
}

#[get("/user")]
pub async fn get_key_ids(user: User) -> impl Responder {
    HttpResponse::Ok()
        .json(
            &grv!(ApiKey::user_keys(user.id).await)
                .iter()
                .map(|k| k.id)
                .collect::<Vec<_>>()
        )
}

#[derive(Deserialize)]
struct ResetKeyQuery {
    id: i32
}

#[post("/reset")]
pub async fn reset_key(user: User, query: Query<ResetKeyQuery>) -> impl Responder {
    HttpResponse::Ok()
        .body(grv!(ApiKey::reset_key(user.id, query.id).await))
}

#[derive(Deserialize)]
struct PaymentQuery {
    amount: f32
}

#[get("/checkout")]
pub async fn pay_new_key(user: User, req: HttpRequest, query: Query<PaymentQuery>) -> impl Responder {
    HttpResponse::PermanentRedirect()
        .insert_header((
            "Location",
            grv!(create_stripe_payment(&get_callback(&req), &user, query.amount).await)
        ))
        .finish()
}

#[derive(Deserialize)]
struct CheckoutSuccess {
    #[serde(rename = "i")]
    user_id: i32,
    #[serde(rename = "p")]
    paid: f64,
    #[serde(rename = "c")]
    checkout: String
}

#[get("/checkout/success")]
pub async fn handle_success_payment(req: HttpRequest, query: Query<CheckoutSuccess>) -> impl Responder {
    if !grv!(verify_payment(&query.checkout).await) {
        return HttpResponse::BadRequest()
            .body("Unknown or invalid transaction.");
    }

    let _ = grv!(ApiKey::new(query.user_id, &query.checkout, query.paid).await);

    HttpResponse::PermanentRedirect()
        .insert_header(("Location", format!("{}/transaction/success", get_callback(&req))))
        .finish()
}
