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
    let data =
        r#"[{
            "word": "blue",
            "origin": "origin",
            "meanings": [{
                "partOfSpeech": "wee",
                "definitions": [{
                    "definition": "",
                    "example": "",
                    "synonyms": [""],
                    "antonyms": [""]
                }]
            }]
    }]"#;
    let args = Cli::from_args();
    let url = format!("https://api.dictionaryapi.dev/api/v2/entries/en/{}", args.word);
    let response = reqwest::get(&url).await?;
    let r: Vec<Thesaurus> = response.json().await?;
    let datas: Vec<Thesaurus> = serde_json::from_str(data)?;
    let res = &r[0];
    println!("{}", res.meanings[0].partOfSpeech);
    Ok(())
}
