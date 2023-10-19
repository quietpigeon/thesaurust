use std::{default, fmt::Debug};

use serde_derive::{Deserialize, Serialize};

use crate::app::InputMode;

/// Components of a response from the Free Dictionary API.
#[derive(Clone, Serialize, Deserialize, Debug)]
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
    fn is_null(&self) -> bool {
        self.word.is_none() && self.origin.is_none() && self.meanings.is_none()
    }

    pub fn get_definitions_from(thesaurus: &Thesaurus) -> Vec<Definition> {
        if thesaurus.is_null() {
            return Vec::<Definition>::new();
        }
        let meanings = thesaurus.meanings.as_ref().unwrap();
        let meaning = &meanings[0];
        if meaning.definitions.is_some() {
            meaning.definitions.as_ref().unwrap().to_vec()
        } else {
            return Vec::<Definition>::new();
        }
    }

    pub fn get_part_of_speech_from(thesaurus: &Thesaurus) -> String {
        if thesaurus.is_null() {
            return String::from("");
        }
        let meanings = thesaurus.meanings.as_ref().unwrap();
        let meaning = &meanings[0];
        if meaning.partOfSpeech.is_some() {
            return meaning.partOfSpeech.as_ref().unwrap().to_string();
        } else {
            return String::from("");
        }
    }

    pub fn generate_from(invalid_input: &String) -> Vec<Thesaurus> {
        let mut thesaurus = Thesaurus::default();
        let mut t = Vec::<Thesaurus>::new();
        thesaurus.word = Some(invalid_input.to_string());
        let _ = t.push(thesaurus);
        t
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct Meaning {
    pub partOfSpeech: Option<String>,
    pub definitions: Option<Vec<Definition>>,
}

impl Default for Meaning {
    fn default() -> Self {
        Meaning {
            partOfSpeech: None,
            definitions: None,
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Definition {
    pub definition: Option<String>,
    pub example: Option<String>,
    synonyms: Option<Vec<String>>,
    antonyms: Option<Vec<String>>,
}

impl Default for Definition {
    fn default() -> Self {
        Definition {
            definition: None,
            example: None,
            synonyms: None,
            antonyms: None,
        }
    }
}
