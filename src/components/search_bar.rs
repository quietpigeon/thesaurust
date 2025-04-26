use crate::models::app::{App, InputMode};
use ratatui::{
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Wrap},
};

pub(crate) fn new(app: &mut App) -> Paragraph {
    Paragraph::new(app.input.value())
        .style(match app.input_mode {
            InputMode::Editing => Style::default().fg(Color::Yellow),
            _ => Style::default().fg(Color::Green),
        })
        .wrap(Wrap { trim: true })
        .block(Block::default().borders(Borders::ALL).title("Search"))
}
