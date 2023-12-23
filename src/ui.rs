use ratatui::{
    layout::{ Direction, Layout },
    prelude::{ Alignment, Constraint },
    style::{ Color, Modifier, Style, Stylize },
    widgets::{ Block, Borders, List, ListItem, Paragraph, Wrap },
};

use crate::{
    models::{ data::Thesaurus, app::{ InputMode, App } },
    banner::BANNER,
    tui::Frame,
    components::{ search_bar, definition_block, example_block, banner_block },
};

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

    // The `footer` consists of a spacer and two sets of instructions.
    let footer = Layout::default()
        .direction(Direction::Vertical)
        // Spacer(50%), Instructions(25%), Default instructions(25%)
        .constraints(
            [
                Constraint::Percentage(50),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
            ].as_ref()
        )
        .split(main_frame[2]);

    // Index of the item selected by the user in `part_of_speech_list`.
    let mut idx = 0;
    if let Some(i) = app.part_of_speech_list.state.selected() {
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

        // `Part Of Speech` block that shows the part of speech of the word.
        // They are also used to show whether the word has multiple definitions or not.
        let meanings = app.results[0].meanings.clone();
        if meanings.is_some() {
            let parts_of_speech: Vec<ListItem> = app.part_of_speech_list.items
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

            // `Part Of Speech` block
            f.render_stateful_widget(
                parts_of_speech,
                lower_frame[0],
                &mut app.part_of_speech_list.state
            );
        }

        f.render_widget(definition_block::new(app, definitions, definition), right_frame[0]);
        f.render_widget(example_block::new(example), right_frame[1]);
    } else {
        f.render_widget(banner_block::new(), banner_frame[0]);
    }

    f.render_widget(search_bar::new(app), upper_frame[0]);

    // Instructions.
    let instructions = App::update_instructions(app);
    let default_instructions = "q: Quit";
    f.render_widget(create_footer_block(instructions), footer[1]);
    f.render_widget(create_footer_block(default_instructions), footer[2]);
}

// Helper function to create footer blocks.
fn create_footer_block(s: &str) -> Paragraph {
    Paragraph::new(s)
        .alignment(Alignment::Left)
        .style(Style::default().fg(Color::Green))
        .block(Block::default().borders(Borders::NONE))
}
