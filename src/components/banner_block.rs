use crate::banner::BANNER;
use ratatui::{
    layout::Alignment,
    style::{Color, Style},
    widgets::Paragraph,
};

pub(crate) fn new() -> Paragraph<'static> {
    Paragraph::new(BANNER)
        .style(Style::default().fg(Color::Green))
        .alignment(Alignment::Center)
}
