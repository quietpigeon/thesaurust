use ratatui::{ widgets::{ Block, Borders, Paragraph }, style::{ Color, Style }, layout::Alignment };

pub fn with(instructions: &str) -> Paragraph {
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
