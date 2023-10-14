use exitfailure::ExitFailure;
use structopt::StructOpt;

#[path = "protocol/data.rs"]
pub mod protocol;
use crate::protocol::*;

#[tokio::main]
pub async fn fetch_response() -> Result<Vec<Thesaurus>, ExitFailure> {
    let args = from_cli();
    let url = Cli::construct_url(&args);
    let response = reqwest::get(&url).await?;
    let results: Vec<Thesaurus> = response.json().await?;
    Ok(results)
}

/// MARK: Private functions
fn from_cli() -> Cli {
    Cli::from_args()
}

/// MARK: Unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_fetch_response() {}
}
