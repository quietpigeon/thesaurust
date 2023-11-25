use serpapi_search_rust::serp_api_search::SerpApiSearch;

use crate::models::{ data::Thesaurus, errors::ApiError, spellcheck::SearchResults };
use crate::api_key::API_KEY;
use std::collections::HashMap;

const DOMAIN: &'static str = "https://api.dictionaryapi.dev/api/v2/entries/en";

pub fn parse_response(word: String) -> Vec<Thesaurus> {
    match client(word) {
        Ok(t) => t,
        Err(_) => Thesaurus::inject_error_message(String::from("Unsuccessful response")),
    }
}

#[tokio::main]
async fn client(word: String) -> Result<Vec<Thesaurus>, Box<dyn std::error::Error>> {
    let res = match fetch_response(word).await {
        Ok(t) => {
            let resp: Vec<Thesaurus> = serde_json::from_value(t).unwrap();
            resp
        }
        Err(_) => {
            match search().await {
                Ok(t) => Thesaurus::inject_error_message(t),
                Err(_) => Thesaurus::inject_error_message(String::from("Serp Api error")),
            }
        }
    };
    Ok(res)
}

async fn fetch_response(word: String) -> Result<serde_json::Value, ApiError> {
    let url = construct_url(word);
    let response = reqwest::get(&url).await?;
    if response.status().is_success() {
        let results: serde_json::Value = response.json().await?;
        Ok(results)
    } else {
        Err(ApiError::InvalidInput)
    }
}

async fn search() -> Result<String, Box<dyn std::error::Error>> {
    let mut params = HashMap::<String, String>::new();
    params.insert("q".to_string(), "Coffeee".to_string());
    params.insert("hl".to_string(), "en".to_string());
    params.insert("gl".to_string(), "us".to_string());

    let search = SerpApiSearch::google(params, API_KEY.to_string());

    let results = search.json().await?;
    let search_information = &results["search information"];

    let results: SearchResults = serde_json
        ::from_value(search_information.clone())
        .unwrap_or(SearchResults { spelling_fix: String::from("Feature not available.") });

    Ok(results.spelling_fix)
}

fn construct_url(word: String) -> String {
    format!("{}/{}", DOMAIN, word)
}
