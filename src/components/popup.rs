use ratatui::{
    layout::Alignment,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::models::app::App;

pub(crate) fn new(app: &mut App) -> Paragraph<'static> {
    Paragraph::new(popup_message(app))
        .style(Style::default().fg(Color::Green))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true })
        .block(Block::default().borders(Borders::ALL))
}

fn popup_message(app: &mut App) -> String {
    let message = app.suggested_spelling.clone();
    if app.is_spelling_fix_enabled {
        if message.is_empty() {
            String::from("Similar spelling not found.")
        } else {
            format!("Did you mean {}?", message)
        };
    }

    message
}
