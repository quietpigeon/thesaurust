use crate::models::{input_mode::InputMode, list::StatefulList, thesaurus::Thesaurus};
use tui_input::Input;

/// Application.
#[derive(Clone, Debug, Default)]
pub(crate) struct App {
    pub should_quit: bool,
    pub input: Input,
    pub input_mode: InputMode,
    pub results: Vec<Thesaurus>,
    pub part_of_speech_list: StatefulList<String>,
    pub definition_list: StatefulList<String>,
    pub is_spelling_fix_enabled: bool,
    pub synonym_list: StatefulList<String>,
}

impl App {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn quit(&mut self) {
        self.should_quit = true;
    }

    pub(crate) fn update_instructions(&mut self) -> String {
        match self.input_mode {
            InputMode::Normal if self.part_of_speech_list.items.len() == 1 => {
                String::from("l, h: Change definition  /: Insert")
            }
            InputMode::Normal if !self.results.is_empty() => {
                String::from("j, k: Change part of speech  /: Insert")
            }
            InputMode::Editing => String::from("<ENTER>: Search  <ESC>: Exit"),
            InputMode::SelectPartOfSpeech => String::from("<ENTER>: Select"),
            InputMode::SelectDefinition => String::from("l, h: Change definition  /: Insert"),
            InputMode::Settings => self.toggle_spelling_suggestion(),
            _ => String::from("/: Insert"),
        }
    }

    pub(crate) fn update_all(&mut self) {
        // NOTE: The order of the functions must not be changed.
        self.update_part_of_speech_list();
        self.update_definition_list();
        self.update_synonym_list();
    }

    pub(crate) fn update_definition_list(&mut self) {
        if self.results.is_empty() {
            return;
        }
        if let Some(idx) = self.part_of_speech_list.state.selected() {
            let definitions = Thesaurus::unwrap_meanings_at(idx, &self.results[0]).1;
            let definitions: Vec<String> = definitions
                .into_iter()
                .map(|i| i.definition.unwrap_or_default())
                .collect();
            self.definition_list = StatefulList::with_items(definitions);

            // Select the first item as default.
            self.definition_list.state.select(Some(0))
        }
    }

    pub(crate) fn update_synonym_list(&mut self) {
        if self.results.is_empty() {
            return;
        }
        let pos_idx = self.part_of_speech_list.state.selected().unwrap_or(0);
        let definitions = Thesaurus::unwrap_meanings_at(pos_idx, &self.results[0]).1;
        let def_idx = self.definition_list.state.selected().unwrap_or(0);
        let definition = &definitions[def_idx];
        let synonyms = definition.clone().synonyms;

        if let Some(s) = synonyms {
            self.synonym_list = StatefulList::with_items(s);
        } else {
            self.synonym_list = StatefulList::with_items(Vec::new());
        }
    }

    fn update_part_of_speech_list(&mut self) {
        if self.results.is_empty() {
            return;
        }
        let meanings = self.results[0].meanings.clone();
        if let Some(m) = meanings {
            let part_of_speech_list: Vec<String> = m
                .into_iter()
                .map(|i| i.partOfSpeech.unwrap_or_default())
                .collect();
            self.part_of_speech_list = StatefulList::with_items(part_of_speech_list);

            // Select the first item as default.
            self.part_of_speech_list.state.select(Some(0))
        }
    }

    fn toggle_spelling_suggestion(&mut self) -> String {
        format!("Spelling suggestion: {}", self.is_spelling_fix_enabled)
    }
}

#[cfg(test)]
mod tests {
    use super::{App, InputMode};
    use crate::models::thesaurus::{Definition, Meaning, Thesaurus};
    use pretty_assertions::assert_eq;

    fn mock_app_in(input_mode: InputMode) -> App {
        let mut mock_app = App::new();
        mock_app.input_mode = input_mode;
        return mock_app;
    }

    fn mock_part_of_speech() -> String {
        String::from("noun")
    }

    fn mock_meaning_with(p: Option<String>, d: Option<Vec<Definition>>) -> Meaning {
        Meaning {
            partOfSpeech: p,
            definitions: d,
        }
    }

    fn mock_definition_with(d: Option<String>) -> Definition {
        Definition {
            definition: d,
            ..Default::default()
        }
    }

    fn mock_results_with(m: Vec<Meaning>) -> Vec<Thesaurus> {
        vec![Thesaurus {
            word: Some(String::from("mock")),
            meanings: Some(m),
        }]
    }

    #[test]
    fn test_update_part_of_speech_list() {
        let mut mock_app = mock_app_in(InputMode::default());
        let mock_parts_of_speech = vec![
            String::from("noun"),
            String::from("verb"),
            String::from("adjective"),
        ];
        let mock_meanings = mock_parts_of_speech
            .clone()
            .iter()
            .map(|i| mock_meaning_with(Some(i.to_string()), None))
            .collect();
        mock_app.results = mock_results_with(mock_meanings);
        App::update_part_of_speech_list(&mut mock_app);
        assert_eq!(
            mock_parts_of_speech.len(),
            mock_app.part_of_speech_list.items.len()
        );
        assert_eq!(Some(0), mock_app.part_of_speech_list.state.selected())
    }

    #[test]
    fn test_update_all() {
        let mut mock_app = mock_app_in(InputMode::default());
        let mock_definitions = vec![
            mock_definition_with(Some(String::from("Definition 1"))),
            mock_definition_with(Some(String::from("Definition 2"))),
            mock_definition_with(Some(String::from("Definition 3"))),
        ];
        let mock_meanings = vec![mock_meaning_with(
            Some(mock_part_of_speech()),
            Some(mock_definitions.clone()),
        )];
        mock_app.results = mock_results_with(mock_meanings);
        App::update_all(&mut mock_app);

        assert_eq!(mock_definitions.len(), mock_app.definition_list.items.len());
        assert_eq!(Some(0), mock_app.definition_list.state.selected());
    }

    #[test]
    fn test_instructions_in_normal_mode() {
        let mut mock_app = mock_app_in(InputMode::Normal);
        assert_eq!(App::update_instructions(&mut mock_app), "/: Insert");
    }

    #[test]
    fn test_instructions_for_word_with_single_part_of_speech() {
        let mut mock_app = mock_app_in(InputMode::default());
        mock_app.results =
            mock_results_with(vec![mock_meaning_with(Some(mock_part_of_speech()), None)]);
        App::update_part_of_speech_list(&mut mock_app);
        assert_eq!(
            App::update_instructions(&mut mock_app),
            "l, h: Change definition  /: Insert"
        );
    }

    #[test]
    fn test_instructions_in_normal_mode_with_results() {
        let mut mock_app = mock_app_in(InputMode::Normal);
        mock_app.results =
            mock_results_with(vec![mock_meaning_with(Some(mock_part_of_speech()), None)]);
        assert_eq!(true, !mock_app.results.is_empty());
        assert_eq!(
            App::update_instructions(&mut mock_app),
            "j, k: Change part of speech  /: Insert"
        );
    }

    #[test]
    fn test_instructions_in_editing_mode() {
        let mut mock_app = mock_app_in(InputMode::Editing);
        assert_eq!(
            App::update_instructions(&mut mock_app),
            "<ENTER>: Search  <ESC>: Exit"
        );
    }

    #[test]
    fn test_instructions_in_part_of_speech_selection_mode() {
        let mut mock_app = mock_app_in(InputMode::SelectPartOfSpeech);
        assert_eq!(App::update_instructions(&mut mock_app), "<ENTER>: Select");
    }

    #[test]
    fn test_instructions_in_definition_selection_mode() {
        let mut mock_app = mock_app_in(InputMode::SelectDefinition);
        assert_eq!(
            App::update_instructions(&mut mock_app),
            "l, h: Change definition  /: Insert"
        );
    }

    #[test]
    fn test_instructions_in_settings_mode_with_spelling_fix_enabled() {
        let mut mock_app = mock_app_in(InputMode::Settings);
        mock_app.is_spelling_fix_enabled = true;
        assert_eq!(
            App::update_instructions(&mut mock_app),
            format!("Spelling suggestion: true")
        );
    }

    #[test]
    fn test_instructions_in_settings_mode_with_spelling_fix_disabled() {
        let mut mock_app = mock_app_in(InputMode::Settings);
        // mock_app.is_spelling_fix_enabled is false by default.
        assert_eq!(
            App::update_instructions(&mut mock_app),
            format!("Spelling suggestion: false")
        );
    }
}
