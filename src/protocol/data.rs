use serde_derive::{ Deserialize, Serialize };
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Cli {
    pub word: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Thesaurus {
    pub word: String,
    pub origin: Option<String>,
    pub meanings: Vec<Meaning>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Meaning {
    pub partOfSpeech: String,
    pub definitions: Vec<Definition>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Definition {
    pub definition: String,
    pub example: Option<String>,
    synonyms: Vec<String>,
    antonyms: Vec<String>
}
