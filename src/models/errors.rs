use reqwest::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum Error {
    #[error("http request failed: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("failed to read json: {0}")]
    JsonRead(#[from] serde_json::Error),

    #[error("unexpected status code: {0}")]
    StatusError(StatusCode),

    #[error("unexpected response from serp api")]
    SerpError,
}
