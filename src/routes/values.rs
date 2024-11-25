use actix_web::{get, web::Query, HttpResponse, Responder};
use serde::Deserialize;
use crate::{frame_bytes, helpers::{generator::random::{get_bool, get_unsigned}, http::responses::TimedResponse, misc::color::Color}, models::key::MaybeApiKey};

#[get("/unsigned")]
pub async fn random_unsigned(key: MaybeApiKey) -> impl Responder {
    TimedResponse::new(
        get_unsigned(&frame_bytes!()),
        key.into()
    )
        .into()
}

#[get("/signed")]
pub async fn random_signed(key: MaybeApiKey) -> impl Responder {
    let bytes = frame_bytes!();

    let mut number = get_unsigned(&bytes) as i32;

    if get_bool(&bytes) {
        number = -number;
    }

    TimedResponse::new(number, key.into())
        .into()
}

#[get("/boolean")]
pub async fn random_bool(key: MaybeApiKey) -> impl Responder {
    let bytes = frame_bytes!();

    TimedResponse::new(
        get_bool(&bytes),
        key.into()
    )
        .into()
}

#[derive(Deserialize)]
struct ColorQuery {
    format: Option<String>
}

#[get("/color")]
pub async fn random_color(query: Query<ColorQuery>, key: MaybeApiKey) -> impl Responder {
    let bytes = frame_bytes!();

    let color = Color::from(get_unsigned(&bytes));

    let format = query
        .format
        .clone()
        .unwrap_or("rgb".to_string());

    match format.as_str() {
        "rgb" => {
            TimedResponse::new(color, key.into())
                .into()
        },
        "hex" => {
            TimedResponse::new(color.as_hex(), key.into())
                .into()
        },
        _ => {
            HttpResponse::BadRequest()
                .body("Invalid format parameter, expected or either rgb or hex.")
        }
    }
}
