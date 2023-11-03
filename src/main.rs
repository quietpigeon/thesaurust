mod app;
mod banner;
mod client;
mod data;
mod errors;
mod list;
mod selection;
mod tui;
mod ui;

use anyhow::Result;
use app::{App, InputMode};
use client::get_data;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{backend::CrosstermBackend, Terminal};
use tui::Tui;
use tui_input::{backend::crossterm::EventHandler, Input};

fn main() -> Result<()> {
    let mut app = App::new();
    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let mut tui = Tui::new(terminal);
    tui.enter()?;

    // Start the main loop.
    while !app.should_quit {
        tui.draw(&mut app)?;
        if let Event::Key(key) = event::read()? {
            match app.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => {
                        App::quit(&mut app);
                    }
                    KeyCode::Char(':') | KeyCode::Char('j') | KeyCode::Char('e') => {
                        app.input_mode = InputMode::Selecting;
                    }
                    KeyCode::Char('/') => {
                        app.input_mode = InputMode::Editing;
                        app.input.reset();
                    }
                    _ => {}
                },
                InputMode::Editing => match key.code {
                    KeyCode::Enter => {
                        app.input_mode = InputMode::Normal;

                        // Fetch data
                        app.results = get_data(app.input.to_string());

                        // Propagate the data into the corresponding stateful lists.
                        App::update_selections(&mut app);
                    }
                    KeyCode::Esc => {
                        app.input_mode = InputMode::Normal;
                    }
                    _ => {
                        app.input.handle_event(&Event::Key(key));
                    }
                },
                InputMode::Selecting => match key.code {
                    KeyCode::Char('j') => {
                        app.selections.down();
                    }
                    KeyCode::Char('k') => {
                        app.selections.up();
                    }
                    KeyCode::Char('q') | KeyCode::Esc => {
                        app.input_mode = InputMode::Normal;
                    }
                    _ => {}
                },
            }
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
