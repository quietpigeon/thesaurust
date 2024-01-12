use ratatui::{ widgets::{ Block, Borders, List, ListItem }, style::{ Color, Style } };

use crate::models::app::{ App, InputMode };

pub fn new(app: &mut App) -> List {
    let cloned_list = app.part_of_speech_list.clone();
    let parts_of_speech: Vec<ListItem> = cloned_list.items
        .iter()
        .map(|i| ListItem::new(i.clone()))
        .collect();
    let parts_of_speech = List::new(parts_of_speech)
        .block(Block::default().borders(Borders::ALL).title("Part Of Speech"))
        .style(match app.input_mode {
            InputMode::SelectPartOfSpeech => Style::default().fg(Color::Yellow),
            _ => Style::default().fg(Color::Green),
        })
        .highlight_style(Style::default().fg(Color::Black).bg(Color::Cyan));
    parts_of_speech
}
