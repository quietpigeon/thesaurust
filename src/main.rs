use exitfailure::ExitFailure;
use structopt::StructOpt;
mod protocol;

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let args = protocol::Cli::from_args();
    let url = protocol::Cli::construct_url(&args);
    let response = reqwest::get(&url).await?;
    let results: Vec<protocol::Thesaurus> = response.json().await?;

    // There can be more than one results from the API response, but only the first element is chosen for simplicity.
    let result = &results[0];
    println!("{}", result.meanings[0].partOfSpeech);
    println!("{}", result.meanings[0].definitions[0].definition);
    Ok(())
}
