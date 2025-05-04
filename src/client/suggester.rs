use serde::{Deserialize, Serialize};

use crate::models::errors::Error;

pub(crate) struct Suggestions(Vec<Suggestion>);

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
struct Suggestion {
    word: Option<String>,
    score: Option<i32>,
}

#[tokio::main]
pub(crate) async fn suggest_handler(word: &str, url: &str) -> Result<Suggestions, Error> {
    let url = format!("{url}/sug?s={word}");
    let response = reqwest::get(&url).await?;
    if response.status().is_success() {
        let results: serde_json::Value = response.json().await?;
        let s: Vec<Suggestion> = serde_json::from_value(results)?;
        Ok(Suggestions(s))
    } else {
        Err(Error::BadStatus(response.status()))
    }
}

#[cfg(test)]
mod tests {
    use super::suggest_handler;

    #[test]
    fn fetch_response_successful() {
        let mut server = mockito::Server::new();
        let url = server.url();
        let expected_response =
            r#"[{"word":"coffee","score":1500},{"word":"coffeemaker","score":180}]"#;
        let mock = server
            .mock("GET", "/sug?s=coffee")
            .with_status(200)
            .with_body(expected_response)
            .create();
        let result = suggest_handler("coffee", &format!("{url}")).unwrap().0;
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].word.as_ref().unwrap(), "coffee");

        let n = 180 as i32;
        assert_eq!(result[1].score.as_ref().unwrap(), &n);

        mock.assert();
    }
}
