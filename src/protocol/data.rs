use serde_derive::{Deserialize, Serialize};
use structopt::StructOpt;

const DOMAIN: &str = "https://api.dictionaryapi.dev/api/v2/entries/en";

/// The word that the user wants to search for in the thesaurus.
#[derive(StructOpt)]
pub struct Cli {
    pub word: String,
}

impl Cli {
    pub fn new(word: String) -> Self {
        Self { word }
    }

    pub fn construct_url(&self) -> String {
        format!("{}/{}", DOMAIN, self.word)
    }
}

/// Components of a response from the Free Dictionary API.
#[derive(Serialize, Deserialize, Debug)]
pub struct Thesaurus {
    pub word: String,
    pub origin: Option<String>,

    // A word can have multiple meanings, hence it is represented as an array of meanings.
    pub meanings: Option<Vec<Meaning>>,
}

impl Thesaurus {
    pub fn new(word: String, origin: String, meanings: Vec<Meaning>) -> Self {
        Self {
            word,
            origin: Some(origin),
            meanings: Some(meanings),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct Meaning {
    pub partOfSpeech: Option<String>,
    pub definitions: Option<Vec<Definition>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Definition {
    pub definition: Option<String>,
    pub example: Option<String>,
    synonyms: Option<Vec<String>>,
    antonyms: Option<Vec<String>>,
}
