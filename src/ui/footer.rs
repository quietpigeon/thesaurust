use ratatui::{
    layout::Alignment,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};

pub(crate) fn with(instructions: &str) -> Paragraph {
    match instructions {
        "default" => block_with("q: Quit"),
        _ => block_with(instructions),
    }
}

fn block_with(s: &str) -> Paragraph {
    Paragraph::new(s)
        .alignment(Alignment::Left)
        .style(Style::default().fg(Color::Green))
        .block(Block::default().borders(Borders::NONE))
}
