use keys::key_handler;
use models::{app::App, errors::Error};
use ratatui::{backend::CrosstermBackend, crossterm::event::Event, Terminal};
use tui::Tui;

mod banner;
mod client;
mod components;
mod consts;
mod keys;
mod models;
mod tui;
mod ui;

fn main() -> Result<(), Error> {
    // NOTE: The program should continue to run even if `.env` does not exist.
    dotenvy::dotenv().ok();

    let mut app = App::new();
    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let mut tui = Tui::new(terminal);
    tui.enter()?;

    while !app.should_quit {
        tui.draw(&mut app)?;
        if let Event::Key(key) = ratatui::crossterm::event::read()? {
            key_handler(&mut app, key)?;
        }
    }

    tui.exit()?;
    Ok(())
}
