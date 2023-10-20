use crate::{data::Thesaurus, errors::ApiError};

const DOMAIN: &'static str = "https://api.dictionaryapi.dev/api/v2/entries/en";

pub fn get_data(word: String) -> Vec<Thesaurus> {
    match fetch_response(word) {
        Ok(t) => {
            let resp: Vec<Thesaurus> = serde_json::from_value(t).unwrap();
            resp
        }
        Err(_) => Thesaurus::inject_error_message(),
    }
}

// TODO: Error handling.
// Return a `serde_json::Value` first, then check the fields to see if the input is valid or not.
#[tokio::main]
pub async fn fetch_response(word: String) -> Result<serde_json::Value, ApiError> {
    let url = construct_url(word);
    let response = reqwest::get(&url).await?;
    if response.status().is_success() {
        let results: serde_json::Value = response.json().await?;
        Ok(results)
    } else {
        Err(ApiError::InvalidData)
    }
}

fn construct_url(word: String) -> String {
    format!("{}/{}", DOMAIN, word)
}
