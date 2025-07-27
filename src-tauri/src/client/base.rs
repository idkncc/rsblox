use std::fmt;

use reqwest::{IntoUrl, Response};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::client::ApiError;

use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    errors: Vec<CodeError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeError {
    code: i32,
    message: String,
    user_facing_message: String,
}

impl fmt::Display for CodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "error {}: {}", self.code, self.message)
    }
}

impl RobloxClient {
    async fn process_error(&self, response: Response) -> ApiResult<()> {
        let response: ErrorResponse = response.json().await?;

        Err(ApiError::Code(
            response
                .errors
                .first()
                .expect("expected at least one error")
                .clone(),
        ))
    }

    async fn request_get<'de, P, Q, R>(&self, path: P, query: &Q) -> ApiResult<R>
    where
        P: IntoUrl,
        Q: Serialize,
        R: DeserializeOwned,
    {
        let mut retry_attempts = 1;

        loop {
            let response = self.client.get(path.as_str()).query(query).send().await?;

            if response.status().is_success() {
                break Ok(response.json().await?);
            } else {
                self.process_error(response).await?;
            }
        }
    }
}
