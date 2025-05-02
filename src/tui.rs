use crate::{models::app::App, ui};
use anyhow::Result;
use ratatui::crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use ratatui::crossterm::execute;
use ratatui::crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use std::{io, panic};

pub(crate) type Frame<'a> = ratatui::Frame<'a>;
pub(crate) type CrosstermTerminal =
    ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stderr>>;
pub(crate) struct Tui {
    terminal: CrosstermTerminal,
}

impl Tui {
    /// Constructs a new instance of Tui.
    pub(crate) fn new(terminal: CrosstermTerminal) -> Self {
        Self { terminal }
    }

    /// Initializes the terminal interface.
    pub(crate) fn enter(&mut self) -> Result<()> {
        terminal::enable_raw_mode()?;
        execute!(io::stderr(), EnterAlternateScreen, EnableMouseCapture)?;
        let panic_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic| {
            Self::reset().expect("failed to reset the terminal");
            panic_hook(panic);
        }));
        let _ = self.terminal.hide_cursor();
        self.terminal.clear()?;
        Ok(())
    }

    /// Resets the terminal interface.
    fn reset() -> Result<()> {
        terminal::disable_raw_mode()?;
        execute!(io::stderr(), LeaveAlternateScreen, DisableMouseCapture)?;
        Ok(())
    }

    /// Exits the terminal interface.
    pub(crate) fn exit(&mut self) -> Result<()> {
        Self::reset()?;
        self.terminal.show_cursor()?;
        Ok(())
    }

    pub(crate) fn draw(&mut self, app: &mut App) -> Result<()> {
        self.terminal.draw(|frame| ui::render(app, frame))?;
        Ok(())
    }
}
