use ratatui::{
    layout::{Alignment, Direction, Layout},
    prelude::Constraint,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
};
use serde::de;

use crate::{
    app::{App, InputMode},
    tui::Frame,
    data::Thesaurus,
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
            .wrap(Wrap { trim: true })
            .block(Block::default().borders(Borders::ALL).title("Search")),
        chunks[1],
    );

    // Part of speech.
    let mut word = String::from("");
    if app.results.len() > 0 {
        word = get_definition(&app.results[0]);
    }
    f.render_widget(
        Paragraph::new(String::from(word))
            .style(Style::default().fg(Color::Green))
            .wrap(Wrap {trim: true})
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Definition"),
            ),
        chunks[2],
    );
}

/// Retrieves first definition avaiable in the api response.
fn get_definition(results: &Thesaurus) -> String {
    let meanings = results.meanings.as_ref().unwrap();
    let meaning = &meanings[0];
    let definitions = meaning.definitions.as_ref().unwrap();
    definitions[0].definition.as_ref().unwrap().to_string()
}
