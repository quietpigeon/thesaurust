use ratatui::{
    layout::{ Direction, Layout },
    prelude::{ Alignment, Constraint },
    style::{ Color, Modifier, Style, Stylize },
    widgets::{ Block, Borders, List, ListItem, Paragraph, Wrap },
};

use crate::{ app::{ App, InputMode }, data::Thesaurus, tui::Frame, banner::BANNER};

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
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)].as_ref())
        .horizontal_margin(1)
        .split(main_frame[0]);

    // The `lower_frame` consists of the `part_of_speech` block and the `definitions` block.
    let lower_frame = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(20),
                Constraint::Percentage(80),
            ].as_ref()
        )
        .horizontal_margin(1)
        .split(main_frame[1]);

    // The frame for banner.
    let banner_frame = Layout::default()
        .constraints([Constraint::Percentage(100), Constraint::Percentage(0)])
        .margin(1)
        .split(main_frame[1]);

    // The `right_frame` consists of the `definition` block and the `example` block.
    let right_frame = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(lower_frame[1]);

    let mut definition = String::from("");
    let mut example = String::from("");

    // Index of the item selected by the user in `selections`.
    let idx = app.selections.state.selected();

    if !app.results.is_empty() && idx.is_some() {
        let definitions = Thesaurus::unwrap_meanings_at(idx.unwrap(), &app.results[0]).1;
        if definitions.len() > 0 {
            definition = definitions[0].definition.as_ref().unwrap().to_string();
        }
        if definitions[0].example.is_some() {
            example = definitions[0].example.as_ref().unwrap().to_string();
        }

        // `SELECT` block that shows the part of speech of the word.
        // They are also used to show whether the word has multiple definitions or not.
        let meanings = app.results[0].meanings.clone();
        if meanings.is_some() {
            let selections: Vec<ListItem> = app.selections.items
                .iter()
                .map(|i| ListItem::new(i.content.clone()))
                .collect();

            let selections = List::new(selections)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title( match app.input_mode {
                            InputMode::Selecting => {
                                "SELECT"
                            }
                            _ => {"Part of speech"}
                        })
                        .title_alignment(Alignment::Center)
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
            right_frame[0]
        );

        // Example block.
        f.render_widget(
            Paragraph::new(String::from(example).add_modifier(Modifier::ITALIC))
                .style(Style::default().fg(Color::Green))
                .wrap(Wrap { trim: true })
                .block(Block::default().borders(Borders::ALL).title("Example")),
            right_frame[1]
        );
    } else {
        f.render_widget(
            Paragraph::new(
                BANNER
            )
                .style(Style::default().fg(Color::Green))
                .alignment(Alignment::Center),
            banner_frame[0]
        );
    }
    // Search bar.
    f.render_widget(
        Paragraph::new(app.input.value())
            .style(match app.input_mode {
                InputMode::Normal | InputMode::Selecting => Style::default().fg(Color::Green),
                InputMode::Editing => Style::default().fg(Color::Yellow),
            })
            .wrap(Wrap { trim: true })
            .block(Block::default().borders(Borders::ALL).title("Search")),
        upper_frame[0]
    );

    // Help bar.
    f.render_widget(
        Paragraph::new(String::from("Press `Esc` to stop running, `/` to start."))
            .style(Style::default().fg(Color::Green))
            .wrap(Wrap { trim: true })
            .block(Block::default().borders(Borders::ALL).title("Help")),
        upper_frame[1]
    );
}
