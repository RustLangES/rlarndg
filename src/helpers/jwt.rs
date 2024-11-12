use std::sync::OnceLock;
use rand::{distributions::Alphanumeric, thread_rng, Rng};

pub static JWT_HASH: OnceLock<String> = OnceLock::new();

#[inline]
pub fn get_jwt_hash() -> &'static [u8] {
    JWT_HASH.get_or_init(|| {
        thread_rng()
            .sample_iter(&Alphanumeric)
            .take(512)
            .map(char::from)
            .collect::<String>()
    })
        .as_ref()
}

#[macro_export]
macro_rules! jwt_hash {
    (decode) => {
        &jsonwebtoken::DecodingKey::from_secret(
            $crate::helpers::jwt::get_jwt_hash()
        )
    };
    (encode) => {
        &jsonwebtoken::EncodingKey::from_secret(
            $crate::helpers::jwt::get_jwt_hash()
        )
    }
}
