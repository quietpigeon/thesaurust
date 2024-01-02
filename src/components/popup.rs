use ratatui::{ widgets::Paragraph, style::{ Style, Color }, layout::Alignment };

pub fn new() -> Paragraph<'static> {
    Paragraph::new("foo").style(Style::default().fg(Color::Green)).alignment(Alignment::Center)
}
