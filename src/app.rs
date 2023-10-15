/// Application.
#[derive(Debug, Default)]
pub struct App {
    pub should_quit: bool,
    pub input: String
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}