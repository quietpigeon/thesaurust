use ratatui::{
    layout::{Direction, Layout},
    prelude::Constraint,
    style::{Color, Modifier, Style, Stylize},
    widgets::{Block, Borders, Paragraph, Wrap, Padding},
};

use crate::{
    app::{App, InputMode},
    data::Thesaurus,
    tui::Frame,
};

pub fn render(app: &mut App, f: &mut Frame) {
    // Main frame.
    let main_frame = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Length(12),
                Constraint::Min(1),
            ]
            .as_ref(),
        )
        .split(f.size());

    // The `upper_frame` consists of the search bar and the help bar.
    let upper_frame = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(75), Constraint::Percentage(25)].as_ref())
        .split(main_frame[0]);

    // The `lower_frame` consists of the `part_of_speech` block and the `definitions` block.
    let lower_frame = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(10), Constraint::Percentage(80), Constraint::Percentage(10)].as_ref())
        .margin(1)
        .split(main_frame[1]);

    let right_frame = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ]
            .as_ref(),
        )
        .split(lower_frame[1]);

    let mut part_of_speech = String::from("");
    let mut definition = String::from("");
    let mut example = String::from("");
    if !app.results.is_empty() {
        part_of_speech = Thesaurus::unwrap_meanings_at(0, &app.results[0]).0;
        let definitions = Thesaurus::unwrap_meanings_at(0, &app.results[0]).1;
        if definitions.len() > 0 {
            definition = definitions[0].definition.as_ref().unwrap().to_string();
            example = definitions[0].example.as_ref().unwrap().to_string();
        }
    }

    // Search bar.
    f.render_widget(
        Paragraph::new(app.input.value())
            .style(match app.input_mode {
                InputMode::Normal => Style::default().fg(Color::Green),
                InputMode::Editing => Style::default().fg(Color::Yellow),
            })
            .wrap(Wrap { trim: true })
            .block(Block::default().borders(Borders::ALL).title("Search")),
        upper_frame[0],
    );

    // Help bar.
    f.render_widget(
        Paragraph::new(String::from("Press `Esc` to stop running, `/` to start."))
            .wrap(Wrap { trim: true })
            .block(Block::default().borders(Borders::ALL).title("Help")),
        upper_frame[1],
    );

    // Results block.
    f.render_widget(
        Paragraph::new(String::from(""))
            .style(Style::default().fg(Color::Green))
            .wrap(Wrap { trim: true })
            .block(Block::default().borders(Borders::ALL).title("Thesaurust")),
        main_frame[1],
    );

    // `Part of speech` block.
    // TODO: Enable user to navigate between different meanings. Default should show the first result.
    f.render_widget(
        Paragraph::new(String::from(part_of_speech))
            .style(Style::default().fg(Color::Green))
            .wrap(Wrap { trim: true })
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Part of speech"),
            ),
        lower_frame[0],
    );

    // Definition block.
    f.render_widget(
        Paragraph::new(String::from(definition))
            .style(Style::default().fg(Color::Green))
            .wrap(Wrap { trim: true })
            .block(Block::default().borders(Borders::ALL).title("Definition")),
        right_frame[0],
    );

    // Example block.
    f.render_widget(
        Paragraph::new(String::from(example).add_modifier(Modifier::ITALIC))
            .style(Style::default())
            .wrap(Wrap { trim: true })
            .block(Block::default().borders(Borders::ALL).title("Example")),
        right_frame[1],
    );

    // Synonym block.
    f.render_widget(
        Paragraph::new(String::from(""))
            .style(Style::default())
            .wrap(Wrap { trim: true })
            .block(Block::default().borders(Borders::ALL).title("Similar")),
        lower_frame[2],
    )

    // TODO: Add synonyms and antonyms block.
}
