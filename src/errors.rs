#[derive(Debug)]
pub enum ApiError {
    HttpError(reqwest::Error),
    InvalidInput,
}

impl From<reqwest::Error> for ApiError {
    fn from(err: reqwest::Error) -> Self {
        ApiError::HttpError(err)
    }
}
