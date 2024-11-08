use actix_web::{get, web::Query, HttpResponse, Responder};
use serde::Deserialize;
use crate::{frame_bytes, helpers::{color::Color, random::{get_bool, get_unsigned}, responses::TimedResponse}};

#[get("/unsigned")]
pub async fn random_unsigned() -> impl Responder {
    let bytes = frame_bytes!();

    TimedResponse::new(
        get_unsigned(&bytes)
    )
        .into()
}

#[get("/signed")]
pub async fn random_signed() -> impl Responder {
    let bytes = frame_bytes!();

    let mut number = get_unsigned(&bytes) as i32;

    if get_bool(&bytes) {
        number = -number;
    }

    TimedResponse::new(number)
        .into()
}

#[get("/boolean")]
pub async fn random_bool() -> impl Responder {
    let bytes = frame_bytes!();

    TimedResponse::new(
        get_bool(&bytes)
    )
        .into()
}

#[derive(Deserialize)]
struct ColorQuery {
    format: Option<String>
}

#[get("/color")]
pub async fn random_color(query: Query<ColorQuery>) -> impl Responder {
    let bytes = frame_bytes!();

    let color = Color::from(get_unsigned(&bytes));

    let format = query
        .format
        .clone()
        .unwrap_or("rgb".to_string());

    match format.as_str() {
        "rgb" => {
            TimedResponse::new(color)
                .into()
        },
        "hex" => {
            TimedResponse::new(color.as_hex())
                .into()
        },
        _ => {
            HttpResponse::BadRequest()
                .body("Invalid format parameter, expected or either rgb or hex.")
        }
    }
}
