use ratatui::{ widgets::{ Paragraph }, style::{ Color, Style }, layout::Alignment };

use crate::banner::BANNER;

pub fn new() -> Paragraph<'static> {
    Paragraph::new(BANNER).style(Style::default().fg(Color::Green)).alignment(Alignment::Center)
}
