mod protocol;

#[cfg(test)]
use mockito;
mod tests {
    use super::*;

    #[tokio::test]
    async fn valid_results() {
        let test_args = protocol::Cli::new("lol".to_string());
        let url = protocol::Cli::construct_url(&test_args);
        let response = reqwest::get(&url).await.unwrap();
        let results: Vec<protocol::Thesaurus> = response.json().await;
        let result = &results[0];
        assert!(result.meanings[0].partOfSpeech.is_empty())
    }
}
