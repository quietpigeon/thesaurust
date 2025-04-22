use serde_derive::Deserialize;
use std::fmt::Debug;

/// Components of a response from the Free Dictionary API.
#[derive(Default, Clone, Deserialize, Debug)]
pub struct Thesaurus {
    pub word: Option<String>,

    // A word can have multiple meanings, hence it is represented as an array of meanings.
    pub meanings: Option<Vec<Meaning>>,
}

impl Thesaurus {
    /// A function that unwraps the contents inside `Meaning`. It returns a tuple that contains the `partOfSpeech` and `Vec<Definition>`.
    pub fn unwrap_meanings_at(index: usize, thesaurus: &Thesaurus) -> (String, Vec<Definition>) {
        //TODO: Create unit test to check index and array length.
        if let Some(meanings) = thesaurus.meanings.clone() {
            let meaning = meanings[index].clone();
            if let Some(part_of_speech) = meaning.partOfSpeech.clone() {
                let definitions = meaning.definitions.clone().unwrap();
                (part_of_speech, definitions)
            } else {
                (String::from(""), Vec::<Definition>::default())
            }
        } else {
            (String::from(""), Vec::<Definition>::default())
        }
    }

    /// A function that prompts the user to re-enter the word because the word cannot be found in the API.
    pub fn inject_message(msg: String) -> Vec<Thesaurus> {
        let definition = Definition {
            definition: None,
            example: None,
            synonyms: None,
        };
        let d = vec![definition];
        let meaning = Meaning {
            partOfSpeech: Some(String::from("/")),
            definitions: Some(d),
        };
        let m = vec![meaning];
        let thesaurus = Thesaurus {
            word: Some(msg),
            meanings: Some(m),
        };
        vec![thesaurus]
    }
}

#[derive(Default, Clone, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct Meaning {
    pub partOfSpeech: Option<String>,
    pub definitions: Option<Vec<Definition>>,
}

#[derive(Default, Clone, Deserialize, Debug)]
pub struct Definition {
    pub definition: Option<String>,
    pub example: Option<String>,
    pub synonyms: Option<Vec<String>>,
}
