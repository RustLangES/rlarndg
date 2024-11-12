use actix_web::{get, web::Query, HttpResponse, Responder};
use serde::Deserialize;
use crate::{frame_bytes, helpers::{color::Color, random::{get_bool, get_unsigned}, responses::TimedResponse}, models::user::MaybeUser};

#[get("/unsigned")]
pub async fn random_unsigned(user: MaybeUser) -> impl Responder {
    let bytes = frame_bytes!();

    TimedResponse::new(
        get_unsigned(&bytes),
        user.into()
    )
        .into()
}

#[get("/signed")]
pub async fn random_signed(user: MaybeUser) -> impl Responder {
    let bytes = frame_bytes!();

    let mut number = get_unsigned(&bytes) as i32;

    if get_bool(&bytes) {
        number = -number;
    }

    TimedResponse::new(number, user.into())
        .into()
}

#[get("/boolean")]
pub async fn random_bool(user: MaybeUser) -> impl Responder {
    let bytes = frame_bytes!();

    TimedResponse::new(
        get_bool(&bytes),
        user.into()
    )
        .into()
}

#[derive(Deserialize)]
struct ColorQuery {
    format: Option<String>
}

#[get("/color")]
pub async fn random_color(query: Query<ColorQuery>, user: MaybeUser) -> impl Responder {
    let bytes = frame_bytes!();

    let color = Color::from(get_unsigned(&bytes));

    let format = query
        .format
        .clone()
        .unwrap_or("rgb".to_string());

    match format.as_str() {
        "rgb" => {
            TimedResponse::new(color, user.into())
                .into()
        },
        "hex" => {
            TimedResponse::new(color.as_hex(), user.into())
                .into()
        },
        _ => {
            HttpResponse::BadRequest()
                .body("Invalid format parameter, expected or either rgb or hex.")
        }
    }
}
