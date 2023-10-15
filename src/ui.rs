use ratatui::{
    layout::Alignment,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::{app::App, tui::Frame};

pub fn render(app: &mut App, f: &mut Frame) {
    f.render_widget(
        Paragraph::new(format!(
            "Press `Esc` to stop running. \n\
    Press something."
        ))
        .block(
            Block::default()
                .title("Thesaurust")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Green))
        .alignment(Alignment::Center),
        f.size(),
    )
}
