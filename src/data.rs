use std::fmt::Debug;

use serde_derive::{Deserialize, Serialize};

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
    //TODO: Combine `get_definitions_from()` and `get_part_of_speech` together.
    pub fn get_definitions_from(thesaurus: &Thesaurus) -> Vec<Definition> {
        let part_of_speech = Self::get_part_of_speech_from(thesaurus);
        if part_of_speech.len() == 0 {
            return Vec::<Definition>::new();
        }
        let meanings = thesaurus.meanings.as_ref().unwrap();
        let meaning = &meanings[0];
        meaning.definitions.as_ref().unwrap().to_vec()
    }

    pub fn get_part_of_speech_from(thesaurus: &Thesaurus) -> String {
        if thesaurus.meanings.is_none() {
            return String::from("");
        }
        let meanings = thesaurus.meanings.as_ref().unwrap();
        let meaning = &meanings[0];
        if meaning.partOfSpeech.is_none() {
            return String::from("");
        } else {
            return meaning.partOfSpeech.as_ref().unwrap().to_string();
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
