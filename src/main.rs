mod app;
mod tui;
mod ui;
mod update;

use std::str::MatchIndices;

use anyhow::Result;
use app::{App, InputMode};
use crossterm::event::{self, DisableMouseCapture, Event, KeyCode};
use ratatui::{
    backend::{self, CrosstermBackend},
    Terminal,
};
use tui::Tui;
use tui_input::backend::crossterm::EventHandler;

fn main() -> Result<()> {
    let mut app = App::new();
    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let mut tui = Tui::new(terminal);
    tui.enter()?;

    // Start the main loop.
    while !app.should_quit {
        tui.draw(&mut app)?;
        //TODO: Add an event handler for the events.
        if let Event::Key(key) = event::read()? {
            match app.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Esc => {
                        App::quit(&mut app);
                    }
                    KeyCode::Enter => {
                        let input_string = app.input.to_string();
                        if input_string.len() > 0 {
                            let _ = thesaurust::fetch_response(input_string);
                        }
                    }
                    KeyCode::Char('/') => {
                        app.input_mode = InputMode::Editing;
                    }
                    _ => {}
                },
                InputMode::Editing => match key.code {
                    KeyCode::Enter => {
                        // Fetch data
                        let _ = thesaurust::fetch_response(app.input.to_string());
                        app.input_mode = InputMode::Normal;
                        app.input.reset();
                    }
                    KeyCode::Esc => {
                        app.input_mode = InputMode::Normal;
                    }
                    _ => {
                        app.input.handle_event(&Event::Key(key));
                    }
                },
            }
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
