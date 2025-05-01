use crate::models::{data::Thesaurus, errors::Error, word_suggestion::SearchResults};
use serpapi_search_rust::serp_api_search::SerpApiSearch;
use std::collections::HashMap;
use std::env;

static DOMAIN: &str = "https://api.dictionaryapi.dev/api/v2/entries/en";

pub(crate) struct WordInfo {
    pub t: Vec<Thesaurus>,
    pub is_spelling_suggested: bool,
}

pub(crate) fn parse_response(word: &str, is_spelling_fix_enabled: &bool) -> WordInfo {
    match fetch_response(word, is_spelling_fix_enabled) {
        Ok(t) => t,
        Err(_) => WordInfo {
            t: Thesaurus::inject_message("Unsuccessful response"),
            is_spelling_suggested: false,
        },
    }
}

#[tokio::main]
async fn fetch_response(word: &str, is_spelling_fix_enabled: &bool) -> Result<WordInfo, Error> {
    let res = match search_dictionary(word).await {
        Ok(t) => {
            let resp: Vec<Thesaurus> = serde_json::from_value(t)?;
            WordInfo {
                t: resp,
                is_spelling_suggested: false,
            }
        }
        Err(_) => {
            if !is_spelling_fix_enabled {
                WordInfo {
                    t: Thesaurus::inject_message("Please double-check your spelling."),
                    is_spelling_suggested: false,
                }
            } else {
                match suggest_spelling(word).await {
                    Ok(t) => WordInfo {
                        t: Thesaurus::inject_message(&t),
                        is_spelling_suggested: true,
                    },
                    Err(_) => WordInfo {
                        t: Thesaurus::inject_message("Serp Api error"),
                        is_spelling_suggested: false,
                    },
                }
            }
        }
    };

    Ok(res)
}

async fn search_dictionary(word: &str) -> Result<serde_json::Value, Error> {
    let url = format!("{}/{}", DOMAIN, word);
    let response = reqwest::get(&url).await?;
    if response.status().is_success() {
        let results: serde_json::Value = response.json().await?;
        Ok(results)
    } else {
        Err(Error::BadStatus(response.status()))
    }
}

async fn suggest_spelling(word: &str) -> Result<String, Error> {
    let params = HashMap::from([
        ("q".to_string(), word.to_string()),
        ("hl".to_string(), "en".to_string()),
        ("gl".to_string(), "us".to_string()),
    ]);
    let api_key = env::var("API_KEY").unwrap_or_default();
    let search = SerpApiSearch::google(params, api_key);
    let results = search.json().await.map_err(|_| Error::SerpApi)?;
    let search_information = &results["search_information"];
    let results: SearchResults =
        serde_json::from_value(search_information.clone()).unwrap_or(SearchResults {
            spelling_fix: String::from(""),
        });

    Ok(results.spelling_fix)
}
