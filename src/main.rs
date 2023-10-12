use structopt::StructOpt;
use exitfailure::{ExitFailure};
use serde_derive::{Deserialize, Serialize};
use reqwest::Url;

#[derive(StructOpt)]
struct Cli {
    word: String
}

#[derive(Serialize, Deserialize, Debug)]
struct Thesaurus {
    id: String,
    uuid: String,
    src: String,
    section: String,
    target: Vec<String>,
    syns: Vec<String>,
    ants: Vec<String>,
    offensive: bool
}

impl Thesaurus {
    async fn get(word: &String) -> Result<Self, ExitFailure> {
        let url = format!("https://www.dictionaryapi.com/api/v3/references/thesaurus/json/{}?key=54fc6c18-554f-4c36-8b35-ef1835487d81", word);
        let url = Url::parse(&*url)?;
        let resp =reqwest::get(url).await?.json::<Thesaurus>().await?;
        Ok(resp)
    }
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let args = Cli::from_args();
    let resp = Thesaurus::get(&args.word).await?;
    println!("{}: {}", resp.section,args.word);
    Ok(())
}
