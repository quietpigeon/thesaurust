use ratatui::{
    widgets::{ Paragraph, Wrap, Block, Borders },
    style::{ Style, Color },
    layout::Alignment,
};

use crate::models::app::App;

pub fn new(app: &mut App) -> Paragraph<'static> {
    Paragraph::new(popup_message(app))
        .style(Style::default().fg(Color::Green))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true })
        .block(Block::default().borders(Borders::ALL))
}

fn popup_message(app: &mut App) -> String {
    let message = app.suggested_spelling.clone();
    if app.is_spelling_fix_enabled {
        return format!("Did you mean {}?", message);
    }
    return message;
}
