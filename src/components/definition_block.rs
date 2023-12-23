use ratatui::{ widgets::{ Block, Borders, Paragraph, Wrap }, style::{ Color, Style } };
use crate::{ models::{ app::{ App, InputMode }, data::Definition } };

pub fn new(app: &mut App, definitions: Vec<Definition>, definition: String) -> Paragraph {
    Paragraph::new(String::from(definition))
        .style(match app.input_mode {
            InputMode::SelectDefinition => Style::default().fg(Color::Yellow),
            _ => Style::default().fg(Color::Green),
        })
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
        )
}
