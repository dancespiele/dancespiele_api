use bcrypt::BcryptError;
use diesel::result::Error as DbError;
use jsonwebtoken::errors::Error as ErrorToken;
use serde_json::error::Error as SerdeError;
use sled::Error as SledError;
use std::str::Utf8Error;
use warp::reject::Reject;

#[derive(Debug)]
pub struct JwtError {
    pub error: ErrorToken,
}

impl Reject for JwtError {}

#[derive(Debug)]
pub struct TreeError {
    pub error: SledError,
}

impl Reject for TreeError {}

#[derive(Debug)]
pub struct TransformError {
    pub error: SerdeError,
}

impl Reject for TransformError {}

#[derive(Debug)]
pub struct BadRequest {
    pub error: String,
}

impl<'a> Reject for BadRequest {}

#[derive(Debug)]
pub struct Forbidden {
    pub error: String,
}

impl Reject for Forbidden {}

#[derive(Debug)]
pub struct ConvertToString {
    pub error: Utf8Error,
}

impl Reject for ConvertToString {}

#[derive(Debug)]
pub struct DatabaseError {
    pub error: DbError,
}

impl Reject for DatabaseError {}

#[derive(Debug)]
pub struct HashPwdError {
    pub error: BcryptError,
}

impl Reject for HashPwdError {}
