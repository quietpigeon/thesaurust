use ratatui::{
    widgets::{ Block, Borders, Paragraph, Wrap },
    style::{ Color, Style, Stylize, Modifier },
};

pub fn new(example: String) -> Paragraph<'static> {
    Paragraph::new(example)
        .add_modifier(Modifier::ITALIC)
        .style(Style::default().fg(Color::Green))
        .wrap(Wrap { trim: true })
        .block(Block::default().borders(Borders::ALL).title("Example"))
}
