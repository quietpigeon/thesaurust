use ratatui::{
    layout::{Direction, Layout},
    prelude::{Alignment, Constraint},
    style::{Color, Modifier, Style, Stylize},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
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
        .horizontal_margin(1)
        .split(main_frame[0]);

    // The `lower_frame` consists of the `part_of_speech` block and the `definitions` block.
    let lower_frame = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(80),
                Constraint::Percentage(10),
            ]
            .as_ref(),
        )
        .horizontal_margin(1)
        .split(main_frame[1]);

    let right_frame = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(lower_frame[1]);

    let mut definition = String::from("");
    let mut example = String::from("");
    let idx = app.selections.state.selected();
    if !app.results.is_empty() {
        let definitions = Thesaurus::unwrap_meanings_at(idx.unwrap(), &app.results[0]).1;
        if definitions.len() > 0 {
            definition = definitions[0].definition.as_ref().unwrap().to_string();
        }
        if definitions[0].example.is_some() {
            example = definitions[0].example.as_ref().unwrap().to_string();
        }

        // `Part of speech` block.
        let meanings = app.results[0].meanings.clone();
        if meanings.is_some() {
            let selections: Vec<ListItem> = app
                .selections
                .items
                .iter()
                .map(|i| ListItem::new(i.content.clone()))
                .collect();

            let selections = List::new(selections)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("SELECT")
                        .title_alignment(Alignment::Center),
                )
                .style(Style::default().fg(Color::Green))
                .highlight_style(Style::default().fg(Color::Black).bg(Color::Cyan));

            // `SELECT` block
            f.render_stateful_widget(selections, lower_frame[0], &mut app.selections.state);
        }

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
                .style(Style::default().fg(Color::Green))
                .wrap(Wrap { trim: true })
                .block(Block::default().borders(Borders::ALL).title("Example")),
            right_frame[1],
        );

        // Synonym block.
        f.render_widget(
            // Return the index of the selected item.
            Paragraph::new(String::from(""))
                .style(Style::default().fg(Color::Green))
                .wrap(Wrap { trim: true })
                .block(Block::default().borders(Borders::ALL).title("Synonyms")),
            lower_frame[2],
        );
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
}
