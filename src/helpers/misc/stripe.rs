use reqwest::{header::AUTHORIZATION, Client, Error as RequestError};
use serde::Deserialize;
use thiserror::Error;

use crate::models::{key::{ApiKey, ApiKeyError}, user::User};

#[derive(Error, Debug)]
pub enum StripeError {
    #[error("{0:#}")]
    Request(#[from] RequestError),

    #[error("{0:#}")]
    ApiKey(#[from] ApiKeyError)
}

#[derive(Deserialize)]
struct StripeSession {
    url: String
}

pub async fn create_stripe_payment(host: &String, user: &User, amount: f32) -> Result<String, StripeError> {
    let StripeSession { url } = Client::new()
        .post("https://api.stripe.com/v1/checkout/sessions")
        .header(AUTHORIZATION, format!("Bearer {}", lc!("STRIPE_SECRET")))
        .form(&[
            ("payment_method_types[]", "card"),
            ("line_items[0][price_data][currency]", "usd"),
            ("line_items[0][price_data][product_data][name]", "RlARndG API key"),
            ("line_items[0][price_data][unit_amount]", &(amount * 100f32).round().to_string()),
            ("line_items[0][quantity]", "1"),
            ("mode", "payment"),
            ("success_url", &format!(
                "{host}/api/keys/checkout/success?i={}&p={}&c={{CHECKOUT_SESSION_ID}}",
                user.id,
                amount
            )),
            ("cancel_url", &format!("{host}/pricing"))
        ])
        .send()
        .await?
        .json::<StripeSession>()
        .await?;

    Ok(url)
}

#[derive(Deserialize)]
struct StripeSessionResponse {
    payment_intent: String
}

#[derive(Deserialize)]
struct PaymentIntentResponse {
    status: String
}

pub async fn verify_payment(session_id: &String) -> Result<bool, StripeError> {
    if ApiKey::fetch_exists(session_id).await? {
        return Ok(false);
    }

    let StripeSessionResponse { payment_intent } = Client::new()
        .get(format!("https://api.stripe.com/v1/checkout/sessions/{session_id}"))
        .header(AUTHORIZATION, format!("Bearer {}", lc!("STRIPE_SECRET")))
        .send()
        .await?
        .json()
        .await?;

    let PaymentIntentResponse { status } = Client::new()
        .get(format!("https://api.stripe.com/v1/payment_intents/{payment_intent}"))
        .header(AUTHORIZATION, format!("Bearer {}", lc!("STRIPE_SECRET")))
        .send()
        .await?
        .json()
        .await?;

    Ok(status == "succeeded")
}
