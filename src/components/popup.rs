use crate::models::app::App;
use ratatui::layout::Alignment;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};

pub(crate) fn new(app: &mut App) -> Paragraph<'static> {
    Paragraph::new(popup_message(app))
        .style(Style::default().fg(Color::Green))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true })
        .block(Block::default().borders(Borders::ALL))
}

fn popup_message(app: &mut App) -> String {
    String::from("Similar spelling not found.")
}
