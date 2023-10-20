use crate::{data::Thesaurus, errors::ApiError};

const DOMAIN: &'static str = "https://api.dictionaryapi.dev/api/v2/entries/en";

pub fn get_data(word: String) -> Vec<Thesaurus> {
    match fetch_response(word.clone()) {
        Ok(t) => t,
        Err(_) => Thesaurus::generate_from(&word),
    }
}

// TODO: Error handling.
// Return a `serde_json::Value` first, then check the fields to see if the input is valid or not.
#[tokio::main]
pub async fn fetch_response(word: String) -> Result<Vec<Thesaurus>, ApiError> {
    let url = construct_url(word);
    let response = reqwest::get(&url).await?;
    if response.status().is_success() {
        let results: Vec<Thesaurus> = response.json().await?;
        Ok(results)
    } else {
        Err(ApiError::InvalidData)
    }
}

fn construct_url(word: String) -> String {
    format!("{}/{}", DOMAIN, word)
}
