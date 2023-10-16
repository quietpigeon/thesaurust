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

use structopt::StructOpt;
use thesaurust::{fetch_response, protocol::*};

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
                    KeyCode::Esc => {
                        app.should_quit = true;
                    }
                    _ => {}
                },
            }
            if app.should_quit {
                return Ok(());
            }
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
