use exitfailure::ExitFailure;

#[path = "protocol/data.rs"]
pub mod protocol;
use crate::protocol::*;
use serde_json::Value;

#[tokio::main]
pub async fn fetch_response(args: Cli) -> Result<Value, ExitFailure> {
    let url = Cli::construct_url(&args);
    let response = reqwest::get(&url).await?;
    let results: serde_json::Value = response.json().await?;
    Ok(results)
}

/* In main.rs:
#[tokio::main]
async fn main() {
    let args = Cli::from_args();
    tokio::task::spawn_blocking(|| {
        let data = fetch_response(args).unwrap();
        let results: Vec<Thesaurus> = serde_json::from_value(data).unwrap();
        let result = &results[0];
        let meanings = result.meanings.as_ref().unwrap();
        println!("Part of speech: {}", meanings[0].partOfSpeech);
    })
    .await
    .expect("Task panicked");
}
*/
