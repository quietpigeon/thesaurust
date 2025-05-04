use reqwest::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum Error {
    #[error("http request failed: {0}")]
    HttpRequest(#[from] reqwest::Error),

    #[error("failed to read json: {0}")]
    JsonRead(#[from] serde_json::Error),

    #[error("unexpected status code: {0}")]
    BadStatus(StatusCode),

    #[error(transparent)]
    TUI(#[from] anyhow::Error),

    #[error(transparent)]
    TerminalBackend(#[from] std::io::Error),
}
