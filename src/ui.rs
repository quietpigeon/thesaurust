use ratatui::{
    layout::{Alignment, Direction, Layout},
    prelude::Constraint,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::{
    app::{App, InputMode},
    tui::Frame,
};

pub fn render(app: &mut App, f: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Min(1),
            ]
            .as_ref(),
        )
        .split(f.size());
    f.render_widget(
        Paragraph::new(format!("Press `Esc` to stop running, `/` to enter."))
            .block(Block::default().borders(Borders::NONE))
            .style(Style::default().fg(Color::Green))
            .alignment(Alignment::Center),
        chunks[0],
    );
    let input = Paragraph::new(app.input.value())
        .style(match app.input_mode {
            InputMode::Normal => Style::default().fg(Color::Green),
            InputMode::Editing => Style::default().fg(Color::Yellow),
        })
        .block(Block::default().borders(Borders::ALL).title("Search"));
    f.render_widget(input, chunks[1]);
}
