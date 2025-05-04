use crate::models::{errors::Error, thesaurus::Thesaurus};

pub(crate) struct WordDef(pub(crate) Vec<Thesaurus>);

#[tokio::main]
pub(crate) async fn look_up_handler(word: &str, url: &str) -> Result<WordDef, Error> {
    let url = format!("{url}/api/v2/entries/en/{word}");
    let response = reqwest::get(url).await?;
    if response.status().is_success() {
        let v: serde_json::Value = response.json().await?;
        let t: Vec<Thesaurus> = serde_json::from_value(v)?;
        Ok(WordDef(t))
    } else {
        Err(Error::BadStatus(response.status()))
    }
}

#[cfg(test)]
mod tests {
    use crate::client::dictionary::look_up_handler;
    use pretty_assertions::assert_eq;

    #[test]
    fn fetch_response_successful() {
        let mut server = mockito::Server::new();
        let url = server.url();
        let expected_response = r#"[{"word":"foo","phonetics":[],"meanings":[{"partOfSpeech":"noun","definitions":[{"definition":"bar","synonyms":[],"antonyms":[]}],"synonyms":[],"antonyms":[]}],"license":{"name":"CC BY-SA 3.0","url":"https://creativecommons.org/licenses/by-sa/3.0"},"sourceUrls":["https://en.wiktionary.org/wiki/foo"]}]"#;
        let mock = server
            .mock("GET", "/api/v2/entries/en/foo")
            .with_status(200)
            .with_body(expected_response)
            .create();
        let result = look_up_handler("foo", &format!("{url}")).unwrap().0;
        let t = result[0].clone();
        let m = &t.meanings.unwrap()[0];
        let pos = m.partOfSpeech.as_ref().unwrap();
        let d = m.definitions.as_ref().unwrap();

        assert_eq!(&t.word.unwrap(), "foo");
        assert_eq!(pos, "noun");
        assert_eq!(d.len(), 1);
        assert_eq!(d[0].definition.as_ref().unwrap(), "bar");

        mock.assert();
    }
}
