use std::{default, fmt::Debug};

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

trait ErrorIndicator<Thesaurus> {
    fn inject_error_message(&self) -> Vec<Thesaurus> {
        Vec::<Thesaurus>::default()
    }
}

impl Thesaurus {
    fn new(&self) -> Self {
        Self::default()
    }

    /// A function that unwraps the contents inside `Meaning`. It returns a tuple that contains the `partOfSpeech` and `Vec<Definition>`.   
    pub fn unwrap_meanings_at(index: usize, thesaurus: &Thesaurus) -> (String, Vec<Definition>) {
        //TODO: Create unit test to check index and array length.
        if let Some(meanings) = thesaurus.meanings.clone() {
            let meaning = meanings[index].clone();
            if let Some(part_of_speech) = meaning.partOfSpeech.clone() {
                let definitions = meaning.definitions.clone().unwrap();
                return (part_of_speech, definitions);
            } else {
                return (String::from(""), Vec::<Definition>::default());
            };
        } else {
            return (String::from(""), Vec::<Definition>::default());
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
