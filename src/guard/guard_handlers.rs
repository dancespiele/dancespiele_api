use super::{Claims, UserDto};
use crate::error::JwtError;
use jsonwebtoken::{decode, DecodingKey, Validation};
use std::env;
use warp::{reject, Rejection};

pub async fn auth_guard(auth_token: String) -> Result<UserDto, Rejection> {
    let secret = env::var("SECRET").expect("SECRET must be set");

    let token = decode::<Claims>(
        &auth_token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|err| reject::custom(JwtError { error: err }))?;

    let user = UserDto::from(token.claims);

    Ok(user)
}
