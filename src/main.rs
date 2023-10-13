use exitfailure::ExitFailure;
use structopt::StructOpt;
mod protocol;


#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let args = protocol::Cli::from_args();
    let url = format!("https://api.dictionaryapi.dev/api/v2/entries/en/{}", args.word);
    let response = reqwest::get(&url).await?;
    let results: Vec<protocol::Thesaurus > = response.json().await?;
    let result = &results[0];
    println!("{}", result.meanings[0].partOfSpeech);
    println!("{}", result.meanings[0].definitions[0].definition);
    Ok(())
}
