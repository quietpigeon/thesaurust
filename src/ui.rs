use ratatui::{
    layout::{ Direction, Layout },
    prelude::{ Alignment, Constraint },
    style::{ Color, Modifier, Style, Stylize },
    widgets::{ Block, Borders, List, ListItem, Paragraph, Wrap },
};

use crate::{ app::{ App, InputMode }, banner::BANNER, data::{ Thesaurus }, tui::Frame };

pub fn render(app: &mut App, f: &mut Frame) {
    // Main frame.
    let main_frame = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Length(9),
                Constraint::Length(4),
                Constraint::Min(1),
            ].as_ref()
        )
        .split(f.size());

    // The `upper_frame` consists of the search bar and the help bar.
    let upper_frame = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100)].as_ref())
        .horizontal_margin(1)
        .split(main_frame[0]);

    // The `lower_frame` consists of the `part_of_speech` block and the `definitions` block.
    let lower_frame = Layout::default()
        .direction(Direction::Horizontal)
        // Part of speech (20%), right frame (80%)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .horizontal_margin(1)
        .split(main_frame[1]);

    // The frame for banner.
    let banner_frame = Layout::default()
        .constraints([Constraint::Percentage(100)])
        .margin(1)
        .split(main_frame[1]);

    // The `right_frame` consists of the `definition` block and the `example` block.
    let right_frame = Layout::default()
        .direction(Direction::Vertical)
        // Definitions(50%), Examples(50%)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(lower_frame[1]);

    // The `footer` consists of a spacer and `controls`.
    let footer = Layout::default()
        .direction(Direction::Vertical)
        // Spacer(50%), Controls(25%), Default controls(25%)
        .constraints(
            [
                Constraint::Percentage(50),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
            ].as_ref()
        )
        .split(main_frame[2]);


    // Index of the item selected by the user in `selections`.
    let mut idx = 0;
    if let Some(i) = app.selections.state.selected() {
        idx = i;
    }

    let mut definition = String::from("");
    let mut example = String::from("");
    if !app.results.is_empty() {
        let definitions = Thesaurus::unwrap_meanings_at(idx, &app.results[0]).1;
        if let Some(d_idx) = app.definition_list.state.selected() {
            if let Some(d) = definitions[d_idx].definition.clone() {
                definition = d;
                if let Some(e) = definitions[d_idx].example.clone() {
                    example = e;
                }
            } 
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
                        .title(match app.input_mode {
                            InputMode::Selecting => "SELECT",
                            _ => "Part of speech",
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
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(
                            format!(
                                "Definition[{:}/{}]",
                                app.definition_list.state.selected().unwrap() + 1,
                                definitions.len()
                            )
                        )
                ),
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
            Paragraph::new(BANNER)
                .style(Style::default().fg(Color::Green))
                .alignment(Alignment::Center),
            banner_frame[0]
        );
    }
    // Search bar.
    f.render_widget(
        Paragraph::new(app.input.value())
            .style(match app.input_mode {
                InputMode::Editing => Style::default().fg(Color::Yellow),
                _ => Style::default().fg(Color::Green),
            })
            .wrap(Wrap { trim: true })
            .block(Block::default().borders(Borders::ALL).title("Search")),
        upper_frame[0]
    );

    let default_instructions = "q: Quit";
    // Controls.
    f.render_widget(
        Paragraph::new(String::from(default_instructions))
            .alignment(Alignment::Left)
            .style(Style::default().fg(Color::Green))
            .wrap(Wrap { trim: true })
            .block(Block::default().borders(Borders::NONE)),
        footer[1]
    );
}
