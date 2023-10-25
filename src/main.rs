mod app;
mod client;
mod data;
mod errors;
mod list;
mod selection;
mod tui;
mod ui;
mod utils;

use anyhow::Result;
use app::{App, InputMode};
use client::get_data;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{backend::CrosstermBackend, Terminal};
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
                    //TODO: Check if selections is empty or not.
                    KeyCode::Char('j') => {
                        app.selections.next();
                    }
                    KeyCode::Char('k') => {
                        app.selections.previous();
                    }
                    KeyCode::Char('/') => {
                        app.input_mode = InputMode::Editing;
                        app.input.reset();
                    }
                    KeyCode::Char('q') => {
                        app.selections.unselect();
                    }
                    _ => {}
                },
                InputMode::Editing => match key.code {
                    KeyCode::Enter => {
                        // Fetch data
                        app.input_mode = InputMode::Normal;
                        app.results = get_data(app.input.to_string());
                        App::update_selections(&mut app);
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
