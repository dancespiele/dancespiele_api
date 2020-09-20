use jsonwebtoken::errors::Error as ErrorToken;
use reqwest::Error as ReqWestError;
use serde_json::error::Error as SerdeError;
use sled::Error as SledError;
use tera::Error as TeraError;
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
pub struct MailgunError {
    pub error: ReqWestError,
}

impl Reject for MailgunError {}

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
pub struct TemplateError {
    pub error: TeraError,
}

impl Reject for TemplateError {}

#[derive(Debug)]
pub struct Forbidden {
    pub error: String,
}

impl Reject for Forbidden {}
