use crate::model::Model;
use ratatui::{prelude::*, widgets::{Block, Borders}};
use symbols::border;

pub fn view(model: &Model, frame: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(2),
            Constraint::Percentage(100)
        ])
        .split(frame.area());
    let upper_block = Block::new()
        .borders(Borders::TOP | Borders::RIGHT | Borders::LEFT);
    let lower_border_set = border::Set {
        top_left: symbols::line::NORMAL.vertical_right,
        top_right: symbols::line::NORMAL.vertical_left,
        ..symbols::border::PLAIN
    };
    let lower_block = Block::new()
        .border_set(lower_border_set)
        .borders(Borders::ALL);
    frame.render_widget(upper_block, chunks[0]);
    frame.render_widget(lower_block, chunks[1]);
}