use exitfailure::ExitFailure;
use reqwest::Url;
use serde_derive::{ Deserialize, Serialize };
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    word: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Thesaurus {
    word: String,
    origin: Option<String>,
    meanings: Vec<Meaning>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Meaning {
    partOfSpeech: String,
    definitions: Vec<Definition>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Definition {
    definition: String,
    example: Option<String>,
    synonyms: Vec<String>,
    antonyms: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let args = Cli::from_args();
    let url = format!("https://api.dictionaryapi.dev/api/v2/entries/en/{}", args.word);
    let response = reqwest::get(&url).await?;
    let results: Vec<Thesaurus> = response.json().await?;
    let result = &results[0];
    println!("{}", result.meanings[0].partOfSpeech);
    Ok(())
}
