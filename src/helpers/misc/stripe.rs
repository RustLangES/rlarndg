use dotenv::var;
use reqwest::{Client, Error as RequestError};
use serde::Deserialize;
use thiserror::Error;

macro_rules! secret {
    () => {{
        var("STRIPE_SECRET")
            .map_err(|_| StripeError::MissingSecret)?
    }};
}

#[derive(Error, Debug)]
pub enum StripeError {
    #[error("Missing stripe secret.")]
    MissingSecret,

    #[error("{0:#}")]
    Request(#[from] RequestError)
}

#[derive(Deserialize)]
struct StripeSession {
    url: String
}

pub async fn create_stripe_payment(host: &String, jwt: &String, amount: f32) -> Result<String, StripeError> {
    let StripeSession { url, .. } = Client::new()
        .post("https://api.stripe.com/v1/checkout/sessions")
        .header("Authorization", format!("Bearer {}", secret!()))
        .form(&[
            ("payment_method_types[]", "card"),
            ("line_items[0][price_data][currency]", "usd"),
            ("line_items[0][price_data][product_data][name]", "RlARndG API key"),
            ("line_items[0][price_data][unit_amount]", &(amount * 100f32).round().to_string()),
            ("line_items[0][quantity]", "1"),
            ("mode", "payment"),
            ("success_url", &format!("{host}/api/keys/checkout/success?user={jwt}")),
            ("cancel_url", &format!("{host}/api/keys/checkout/cancel"))
        ])
        .send()
        .await?
        .json::<StripeSession>()
        .await?;

    Ok(url)
}
