use ratatui::{
    style::{Color, Modifier, Style, Stylize},
    widgets::{Block, Borders, Paragraph, Wrap},
};

pub(crate) fn new(example: &str) -> Paragraph<'static> {
    Paragraph::new(example.to_string())
        .add_modifier(Modifier::ITALIC)
        .style(Style::default().fg(Color::Green))
        .wrap(Wrap { trim: true })
        .block(Block::default().borders(Borders::ALL).title("Example"))
}
