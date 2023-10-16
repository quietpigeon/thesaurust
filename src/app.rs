#[derive(Debug)]
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
#[derive(Debug, Default)]
pub struct App {
    pub should_quit: bool,
    pub input: String,
    pub input_mode: InputMode,
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}
