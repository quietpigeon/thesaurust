use crate::{ data::Thesaurus, list::{ StatefulList, StatefulListType } };
use tui_input::Input;

#[derive(Clone, Debug)]
pub enum InputMode {
    Normal,
    Editing,
    SelectPartOfSpeech,
    SelectDefinition,
}

impl Default for InputMode {
    fn default() -> Self {
        InputMode::Normal
    }
}

/// Application.
#[derive(Clone, Debug, Default)]
pub struct App {
    pub should_quit: bool,
    pub input: Input,
    pub input_mode: InputMode,
    pub results: Vec<Thesaurus>,
    pub part_of_speech_list: StatefulList<String>,
    pub definition_list: StatefulList<String>,
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    /// Update the stateful lists.
    pub fn update_stateful_lists(&mut self, list_type: StatefulListType) {
        match list_type {
            StatefulListType::PartOfSpeech => {
                self.update_part_of_speech_list();
            }
            StatefulListType::Definition => {
                self.update_definition_list();
            }
            _ => {
                self.update_part_of_speech_list();
                self.update_definition_list();
            }
        }
    }

    /// Update the part of speech list.
    fn update_part_of_speech_list(&mut self) {
        if !self.results.is_empty() {
            let meanings = self.results[0].meanings.clone();
            if meanings.is_some() {
                let part_of_speech_list: Vec<String> = meanings
                    .unwrap()
                    .iter()
                    .map(|i| i.partOfSpeech.clone().unwrap_or(String::from("")))
                    .collect();
                self.part_of_speech_list = StatefulList::with_items(
                    part_of_speech_list,
                    StatefulListType::PartOfSpeech
                );

                // Select the first item as default.
                self.part_of_speech_list.state.select(Some(0))
            }
        }
    }

    /// Update the definition list.
    fn update_definition_list(&mut self) {
        if !self.results.is_empty() {
            if let Some(idx) = self.part_of_speech_list.state.selected() {
                let definitions = Thesaurus::unwrap_meanings_at(idx, &self.results[0]).1;
                let definitions: Vec<String> = definitions
                    .iter()
                    .map(|i| i.definition.clone().unwrap_or(String::from("")))
                    .collect();
                self.definition_list = StatefulList::with_items(
                    definitions,
                    StatefulListType::Definition
                );

                // Select the first item as default.
                self.definition_list.state.select(Some(0))
            }
        }
    }
}
