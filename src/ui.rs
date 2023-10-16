use ratatui::{
    layout::{Alignment, Direction, Layout},
    prelude::Constraint,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
};
use serde::de;

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

    // Instructions.
    f.render_widget(
        Paragraph::new(format!("Press `Esc` to stop running, `/` to start."))
            .block(Block::default().borders(Borders::NONE))
            .style(Style::default().fg(Color::Green))
            .alignment(Alignment::Center),
        chunks[0],
    );

    // Input section.
    f.render_widget(
        Paragraph::new(app.input.value())
            .style(match app.input_mode {
                InputMode::Normal => Style::default().fg(Color::Green),
                InputMode::Editing => Style::default().fg(Color::Yellow),
            })
            .block(Block::default().borders(Borders::ALL).title("Search")),
        chunks[1],
    );

    // Part of speech.
    let mut word = String::from("");
    if app.results.len() == 0 {
        word = String::from("value");
    } else {
        let res = &app.results[0];
        word = res.word.as_ref().unwrap().to_string();
        let temp = res.meanings.as_ref().unwrap();
        let meaning = &temp[0];
        let definitions = &meaning.definitions;
        let d = definitions.iter().find(|s| !s.is_empty());
        //TODO: Return a definition of a word.
    }
    f.render_widget(
        Paragraph::new(String::from(word))
            .style(Style::default().fg(Color::Cyan))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Part of speech"),
            ),
        chunks[2],
    );
}
