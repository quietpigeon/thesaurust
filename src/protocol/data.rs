use serde_derive::{ Deserialize, Serialize };
use structopt::StructOpt;

/// The word that the user wants to search for in the thesaurus.
#[derive(StructOpt)]
pub struct Cli {
    pub word: String,
}

/// Components of a response from the Free Dictionary API.
#[derive(Serialize, Deserialize, Debug)]
pub struct Thesaurus {
    pub word: String,
    pub origin: Option<String>,

    // A word can have multiple meanings, hence it is represented as an array of meanings.
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
