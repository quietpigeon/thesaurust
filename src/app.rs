use crate::{data::Thesaurus, list::StatefulList, selection::Selection};
use tui_input::Input;

#[derive(Clone, Debug)]
pub enum InputMode {
    Normal,
    Editing,
    Selecting,
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
    pub selections: StatefulList<Selection>,
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    /// Update the stateful list when the user enters a new input.
    pub fn update_selections(&mut self) {
        if !self.results.is_empty() {
            let meanings = self.results[0].meanings.clone();
            if meanings.is_some() {
                let selections: Vec<Selection> = meanings
                    .unwrap()
                    .iter()
                    .map(|part| Selection::new(part.partOfSpeech.as_ref().unwrap()))
                    .collect();
                self.selections = StatefulList::with_items(selections);

                // Select the first item as default/
                self.selections.state.select(Some(0))
            }
        }
    }
}
