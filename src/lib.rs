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
