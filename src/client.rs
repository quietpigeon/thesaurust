use crate::consts::{DICTATIONARY_DOMAIN, SUGGEST_DOMAIN};
use crate::models::errors::Error;
use dictionary::{look_up_handler, WordDef};
use suggester::{suggest_handler, Suggestions};

mod dictionary;
mod suggester;

pub(crate) fn look_up(word: &str) -> Result<WordDef, Error> {
    look_up_handler(word, DICTATIONARY_DOMAIN)
}

/// Compares input with the list of suggested words.
/// If the input is spelled correctly, it should match the first suggestion.
/// Otherwise, the first suggestion is returned as it has the closest
/// spelling to the input.
pub(crate) fn spellcheck(word: &str) -> Result<String, Error> {
    let s = suggest(word)?.0;
    if s.is_empty() {
        return Err(Error::GetSuggestion);
    }
    match &s[0].word {
        Some(w) => Ok(w.to_string()),
        None => Err(Error::GetWord),
    }
}

fn suggest(word: &str) -> Result<Suggestions, Error> {
    suggest_handler(word, SUGGEST_DOMAIN)
}

#[cfg(test)]
mod tests {
    use super::spellcheck;
    use pretty_assertions::assert_eq;

    #[test]
    fn spellcheck_word() {
        let mut server = mockito::Server::new();
        let expected_response =
            r#"[{"word":"coffee","score":1500},{"word":"coffeemaker","score":180}]"#;
        let _ = server
            .mock("GET", "/sug?s=coffee")
            .with_status(200)
            .with_body(expected_response)
            .create();

        assert_eq!(&spellcheck("coffee").unwrap(), "coffee");
        assert_eq!(&spellcheck("coffeee").unwrap(), "coffee");
    }
}
