mod app;
mod tui;
mod ui;
mod update;
mod client;
mod data;

use anyhow::Result;
use app::{App, InputMode};
use client::fetch_response;
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
                            // Fetch data
                        }
                    }
                    KeyCode::Char('/') => {
                        app.input_mode = InputMode::Editing;
                        app.input.reset();
                    }
                    _ => {}
                },
                InputMode::Editing => match key.code {
                    KeyCode::Enter => {
                        // Fetch data
                        app.input_mode = InputMode::Normal;
                        app.results = fetch_response(app.input.to_string()).unwrap();
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
