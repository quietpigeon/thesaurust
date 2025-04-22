#[derive(Debug)]
pub enum ApiError {
    HttpError,
    InvalidInput,
}

// TODO: Use thiserror for better error handling.
#[allow(unused_variables)]
impl From<reqwest::Error> for ApiError {
    fn from(err: reqwest::Error) -> Self {
        ApiError::HttpError
    }
}
