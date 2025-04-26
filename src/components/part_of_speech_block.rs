use ratatui::{
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem},
};

use crate::models::app::{App, InputMode};

pub(crate) fn new(app: &mut App) -> List {
    let parts_of_speech: Vec<ListItem> = app
        .part_of_speech_list
        .items
        .iter()
        .map(|i| ListItem::new(i.clone()))
        .collect();

    List::new(parts_of_speech)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Part Of Speech"),
        )
        .style(match app.input_mode {
            InputMode::SelectPartOfSpeech => Style::default().fg(Color::Yellow),
            _ => Style::default().fg(Color::Green),
        })
        .highlight_style(Style::default().fg(Color::Black).bg(Color::Cyan))
}
