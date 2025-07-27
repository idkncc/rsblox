//! Roblox API client.

use std::sync::Mutex;

use reqwest::{Client, IntoUrl, Response, StatusCode};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use thiserror::Error;

mod base;

pub use base::*;

pub struct RobloxData {
    pub(crate) token: String,
    pub(crate) csrf: String,
}

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("json error: {0}")]
    JSON(#[from] serde_json::Error),

    #[error("code error: {0}")]
    Code(CodeError)
}

pub type ApiResult<T> = Result<T, ApiError>;

pub struct ErrorResponse {
}

pub struct RobloxClient {
    pub(crate) data: Mutex<RobloxData>,
    pub(crate) client: Client,
}

