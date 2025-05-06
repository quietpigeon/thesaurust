use serde_derive::Deserialize;
use std::fmt::Debug;

/// Components of a response from the Free Dictionary API.
#[derive(Default, Clone, Deserialize, Debug)]
pub(crate) struct Thesaurus {
    #[allow(unused)]
    pub word: Option<String>,

    // A word can have multiple meanings, hence it is represented as an array of meanings.
    pub meanings: Option<Vec<Meaning>>,
}

impl Thesaurus {
    /// A function that unwraps the contents inside `Meaning`. It returns a tuple that contains the `partOfSpeech` and `Vec<Definition>`.
    pub(crate) fn unwrap_meanings_at(
        index: usize,
        thesaurus: &Thesaurus,
    ) -> (String, Vec<Definition>) {
        //TODO: Create unit test to check index and array length.
        if let Some(meanings) = thesaurus.meanings.clone() {
            let meaning = meanings[index].clone();
            match (meaning.partOfSpeech.clone(), meaning.definitions.clone()) {
                (Some(p), Some(d)) => (p, d),
                _ => (String::default(), Vec::<Definition>::default()),
            }
        } else {
            (String::default(), Vec::<Definition>::default())
        }
    }
}

#[derive(Default, Clone, Deserialize, Debug)]
#[allow(non_snake_case)]
pub(crate) struct Meaning {
    pub partOfSpeech: Option<String>,
    pub definitions: Option<Vec<Definition>>,
}

#[derive(Default, Clone, Deserialize, Debug)]
pub(crate) struct Definition {
    pub definition: Option<String>,
    pub example: Option<String>,
    pub synonyms: Option<Vec<String>>,
}
