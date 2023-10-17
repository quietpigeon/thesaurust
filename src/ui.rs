use ratatui::{
    layout::{ Alignment, Direction, Layout },
    prelude::Constraint,
    style::{ Color, Style },
    widgets::{ Block, Borders, Paragraph, Wrap },
};

use crate::{ app::{ App, InputMode }, tui::Frame, data::Thesaurus, main };

pub fn render(app: &mut App, f: &mut Frame) {
    // Main frame.
    let main_frame = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Length(3), Constraint::Length(9), Constraint::Min(1)].as_ref())
        .split(f.size());

    let upper_frame = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(75), Constraint::Percentage(25)].as_ref())
        .split(main_frame[0]);

    let lower_frame = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(25), Constraint::Percentage(75)].as_ref())
        .margin(1)
        .split(main_frame[1]);

    // Search bar.
    f.render_widget(
        Paragraph::new(app.input.value())
            .style(match app.input_mode {
                InputMode::Normal => Style::default().fg(Color::Green),
                InputMode::Editing => Style::default().fg(Color::Yellow),
            })
            .wrap(Wrap { trim: true })
            .block(Block::default().borders(Borders::ALL).title("Search")),
        upper_frame[0]
    );

    // Help bar.
    f.render_widget(
        Paragraph::new(String::from("Press `Esc` to stop running, `/` to start.")).block(
            Block::default().borders(Borders::ALL).title("Help")
        ),
        upper_frame[1]
    );

    // Results block.
    f.render_widget(
        Paragraph::new(String::from(""))
            .style(Style::default().fg(Color::Green))
            .wrap(Wrap { trim: true })
            .block(Block::default().borders(Borders::ALL).title("Thesaurust")),
        main_frame[1]
    );

    // `Part of speech`` block.
    let mut part_of_speech = String::from("");
    if app.results.len() > 0 {
        part_of_speech = from_meanings(&app.results[0], String::from("part_of_speech"));
    }
    f.render_widget(
        Paragraph::new(String::from(part_of_speech))
            .style(Style::default().fg(Color::Green))
            .wrap(Wrap { trim: true })
            .block(Block::default().borders(Borders::ALL).title("Part of speech")),
        lower_frame[0]
    );

    // Definition block.
    let mut word = String::from("");
    if app.results.len() > 0 {
        word = from_meanings(&app.results[0], String::from("definition"));
    }
    f.render_widget(
        Paragraph::new(String::from(word))
            .style(Style::default().fg(Color::Green))
            .wrap(Wrap { trim: true })
            .block(Block::default().borders(Borders::ALL).title("Definition")),
        lower_frame[1]
    );
}

//TODO: Move function to app.rs.
/// Retrieves first definition avaiable in the api response.
fn from_meanings(results: &Thesaurus, key: String) -> String {
    let meanings = results.meanings.as_ref().unwrap();
    let meaning = &meanings[0];
    let definitions = meaning.definitions.as_ref().unwrap();
    if key == "definition" {
        definitions[0].definition.as_ref().unwrap().to_string()
    } else {
        meaning.partOfSpeech.to_string()
    }
}
