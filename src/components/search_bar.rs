use ratatui::{widgets::{ Block,Borders, Paragraph, Wrap}, style::{ Color, Style},};
use crate::{ models::{ app::{ InputMode, App } } };

pub fn new(app: &mut App) -> Paragraph {
    Paragraph::new(app.input.value())
        .style(match app.input_mode {
            InputMode::Editing => Style::default().fg(Color::Yellow),
            _ => Style::default().fg(Color::Green),
        })
        .wrap(Wrap { trim: true})
            .block(Block::default().borders(Borders::ALL).title("Search"))
}
