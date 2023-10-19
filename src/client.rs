use crate::{ data::Thesaurus, errors::ApiError };

const DOMAIN: &str = "https://api.dictionaryapi.dev/api/v2/entries/en";

// TODO: Error handling.
#[tokio::main]
pub async fn fetch_response(word: String) -> Result<Vec<Thesaurus>, ApiError> {
    let url = construct_url(word);
    let response = reqwest::get(&url).await?;
    if response.status().is_success() {
        let results: Vec<Thesaurus> = response.json().await?;
        Ok(results)
    } else {
        Err(ApiError::HttpError(reqwest::Error))
    }
}

fn construct_url(word: String) -> String {
    format!("{}/{}", DOMAIN, word)
}
