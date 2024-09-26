use ratatui::{widgets::Tabs,symbols,layout::{Constraint, Layout, Rect}, prelude::Direction, widgets::{Block, Borders, Paragraph}, Frame};
use crate::App;

pub fn ui(frame: &mut Frame, app: &mut App){
    let top_block = Block::new()
        .borders(Borders::TOP | Borders::LEFT | Borders::RIGHT); 
    let tabs = Tabs::new(vec!["test_1", "test_2", "test_3"])
        .block(top_block);
    let border_set = symbols::border::Set {
        top_left: symbols::line::NORMAL.vertical_right,
        top_right: symbols::line::NORMAL.vertical_left,
        ..symbols::border::PLAIN
    };
    let bottom_block = Block::new()
        .border_set(border_set)
        .borders(Borders::ALL);
    let chunks = Layout::default()
    .direction(Direction::Vertical)
    .constraints([
        Constraint::Min(2),
        Constraint::Percentage(100)
    ])
    .split(frame.area());
        frame.render_widget(tabs, chunks[0]);
        frame.render_widget(bottom_block, chunks[1]);
}  

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2)
        ])
        .split(r);
    Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2)
                ])
                .split(popup_layout[1])[1]
}