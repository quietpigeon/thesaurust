use crate::consts::{DICTATIONARY_DOMAIN, SUGGEST_DOMAIN};
use crate::models::errors::Error;
use dictionary::{look_up_handler, WordDef};
use suggester::{suggest_handler, Suggestions};

mod dictionary;
mod suggester;

pub(crate) fn suggest(word: &str) -> Result<Suggestions, Error> {
    suggest_handler(word, SUGGEST_DOMAIN)
}

pub(crate) fn look_up(word: &str) -> Result<WordDef, Error> {
    look_up_handler(word, DICTATIONARY_DOMAIN)
}
