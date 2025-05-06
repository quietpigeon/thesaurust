use ratatui::layout::Alignment;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};

pub(crate) fn new() -> Paragraph<'static> {
    Paragraph::new(popup_message())
        .style(Style::default().fg(Color::Green))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true })
        .block(Block::default().borders(Borders::ALL))
}

fn popup_message() -> String {
    String::from("Similar spelling not found.")
}
