use exitfailure::ExitFailure;

#[path = "protocol/data.rs"]
pub mod protocol;
use crate::protocol::*;

#[tokio::main]
pub async fn fetch_response(args: Cli) -> Result<Vec<Thesaurus>, ExitFailure> {
    let url = Cli::construct_url(&args);
    let response = reqwest::get(&url).await?;
    let results: Vec<Thesaurus>= response.json().await?;
    Ok(results)
}
