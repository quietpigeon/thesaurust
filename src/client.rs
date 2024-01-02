use serpapi_search_rust::serp_api_search::SerpApiSearch;

use crate::models::{ data::Thesaurus, errors::ApiError, word_suggestion::SearchResults };
use crate::api_key::API_KEY;
use std::collections::HashMap;

const DOMAIN: &'static str = "https://api.dictionaryapi.dev/api/v2/entries/en";

pub fn parse_response(word: String, is_spelling_fix_enabled: bool) -> Vec<Thesaurus> {
    match fetch_response(word, is_spelling_fix_enabled) {
        Ok(t) => t,
        Err(_) => Thesaurus::inject_message(String::from("Unsuccessful response")),
    }
}

#[tokio::main]
async fn fetch_response(
    word: String,
    is_spelling_fix_enabled: bool
) -> Result<Vec<Thesaurus>, Box<dyn std::error::Error>> {
    let res = match search_dictionary(word.clone()).await {
        Ok(t) => {
            let resp: Vec<Thesaurus> = serde_json::from_value(t).unwrap();
            resp
        }
        Err(_) => {
            if !is_spelling_fix_enabled {
                Thesaurus::inject_message(String::from("Please double-check your spelling."))
            } else {
                match suggest_spelling(word).await {
                    Ok(t) => Thesaurus::inject_message(t),
                    Err(_) => Thesaurus::inject_message(String::from("Serp Api error")),
                }
            }
        }
    };
    Ok(res)
}

async fn search_dictionary(word: String) -> Result<serde_json::Value, ApiError> {
    let url = construct_url(word);
    let response = reqwest::get(&url).await?;
    if response.status().is_success() {
        let results: serde_json::Value = response.json().await?;
        Ok(results)
    } else {
        Err(ApiError::InvalidInput)
    }
}

async fn suggest_spelling(word: String) -> Result<String, Box<dyn std::error::Error>> {
    let mut params = HashMap::<String, String>::new();
    params.insert("q".to_string(), word.to_string());
    params.insert("hl".to_string(), "en".to_string());
    params.insert("gl".to_string(), "us".to_string());

    let search = SerpApiSearch::google(params, API_KEY.to_string());

    let results = search.json().await?;
    let search_information = &results["search_information"];

    let results: SearchResults = serde_json
        ::from_value(search_information.clone())
        .unwrap_or(SearchResults { spelling_fix: String::from("Feature not available.") });

    Ok(results.spelling_fix)
}

fn parse(value: serde_json::Value, is_suggested: bool) -> (Vec<Thesaurus>, bool) {
    let r: Vec<Thesaurus> = serde_json::from_value(value).unwrap();
    (r, is_suggested)
}

fn construct_url(word: String) -> String {
    format!("{}/{}", DOMAIN, word)
}
