use exitfailure::ExitFailure;
use serde_json::Value;

#[path = "protocol/data.rs"]
pub mod protocol;
use crate::protocol::*;

#[tokio::main]
pub async fn fetch_response(args: Cli) -> Result<Value, ExitFailure> {
    let url = Cli::construct_url(&args);
    let response = reqwest::get(&url).await?;
    let results: serde_json::Value = response.json().await?;
    // let t: Vec<Thesaurus> = serde_json::from_value(results).unwrap();
    Ok(results)
}
