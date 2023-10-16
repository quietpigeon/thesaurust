use serde_derive::{Deserialize, Serialize};

/// Components of a response from the Free Dictionary API.
#[derive(Serialize, Deserialize, Debug)]
pub struct Thesaurus {
    pub word: Option<String>,
    pub origin: Option<String>,

    // A word can have multiple meanings, hence it is represented as an array of meanings.
    pub meanings: Option<Vec<Meaning>>,
}

impl Default for Thesaurus {
    fn default() -> Self {
        Thesaurus {
            word: None,
            origin: None,
            meanings: None,
        }
    }
}

impl Thesaurus {
    pub fn new() -> Self {
        Self::default()
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
