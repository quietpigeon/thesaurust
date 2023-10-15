mod app;
mod event;
mod tui;
mod ui;
mod update;

use std::str::MatchIndices;

use anyhow::Result;
use app::App;
use event::{Event, EventHandler};
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
    let events = EventHandler::new();
    let mut tui = Tui::new(terminal, events);
    tui.enter()?;

    // Start the main loop.
    while !app.should_quit {
        tui.draw(&mut app)?;
        match tui.events.next()? {
            // Some events
        };
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
/*
#[tokio::main]
async fn main() {
    let args = Cli::from_args();
    tokio::task::spawn_blocking(|| {
        let data = fetch_response(args).unwrap();
        let results: Vec<Thesaurus> = serde_json::from_value(data).unwrap();
        let result = &results[0];
        let meanings = result.meanings.as_ref().unwrap();
        println!("Part of speech: {}", meanings[0].partOfSpeech);
    })
    .await
    .expect("Task panicked");
}
*/
