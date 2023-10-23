use crate::{data::Thesaurus, list::StatefulList};
use tui_input::Input;

#[derive(Clone, Debug)]
pub enum InputMode {
    Normal,
    Editing,
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
    pub selections: StatefulList<String>,
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}
