use exitfailure::ExitFailure;

use crate::data::Thesaurus;

const DOMAIN: &str = "https://api.dictionaryapi.dev/api/v2/entries/en";

// TODO: Error handling.
#[tokio::main]
pub async fn fetch_response(word: String) -> Result<Vec<Thesaurus>, ExitFailure> {
    let url = construct_url(word);
    let response = reqwest::get(&url).await?;
    let results: Vec<Thesaurus> = response.json().await?;
    Ok(results)
}

fn construct_url(word: String) -> String {
    format!("{}/{}", DOMAIN, word)
}
