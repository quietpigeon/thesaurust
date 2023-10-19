#[derive(Debug)]
pub enum ApiError {
    HttpError(reqwest::Error),
    InvalidInput,
}
