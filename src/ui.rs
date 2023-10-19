use ratatui::{
    layout::{ Direction, Layout },
    prelude::Constraint,
    style::{ Color, Style },
    widgets::{ Block, Borders, Paragraph, Wrap },
};

use crate::{ app::{ App, InputMode }, data::Thesaurus, tui::Frame };

pub fn render(app: &mut App, f: &mut Frame) {
    // Main frame.
    let main_frame = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Length(3), Constraint::Length(9), Constraint::Min(1)].as_ref())
        .split(f.size());

    // The `upper_frame` consists of the search bar and the help bar.
    let upper_frame = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(75), Constraint::Percentage(25)].as_ref())
        .split(main_frame[0]);

    // The `lower_frame` consists of the `part_of_speech` block and the `definitions` block.
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

    // `Part of speech` block.
    // TODO: Enable user to navigate between different meanings. Default should show the first result.
    let mut part_of_speech = String::from("");
    if app.results.len() > 0 {
        part_of_speech = Thesaurus::get_part_of_speech_from(&app.results[0]);
    }
    f.render_widget(
        Paragraph::new(String::from(part_of_speech))
            .style(Style::default().fg(Color::Green))
            .wrap(Wrap { trim: true })
            .block(Block::default().borders(Borders::ALL).title("Part of speech")),
        lower_frame[0]
    );

    // Definition block.
    let mut definition = String::from("");
    if app.results.len() > 0 {
        let definitions = Thesaurus::get_definitions_from(&app.results[0]);
        if definitions.len() > 0 {
            definition = definitions[0].definition.as_ref().unwrap().to_string();
        }
    }
    f.render_widget(
        Paragraph::new(String::from(definition))
            .style(Style::default().fg(Color::Green))
            .wrap(Wrap { trim: true })
            .block(Block::default().borders(Borders::ALL).title("Definition")),
        lower_frame[1]
    );
}
