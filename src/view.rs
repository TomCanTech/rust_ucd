use crate::model::{Model, Window};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph, Tabs},
};
use symbols::border;

pub fn view(model: &mut Model, frame: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(2), Constraint::Percentage(100)])
        .split(frame.area());
    let upper_block = Block::new().borders(Borders::TOP | Borders::RIGHT | Borders::LEFT);
    let lower_border_set = border::Set {
        top_left: symbols::line::NORMAL.vertical_right,
        top_right: symbols::line::NORMAL.vertical_left,
        ..symbols::border::PLAIN
    };
    let lower_block = Block::new()
        .border_set(lower_border_set)
        .borders(Borders::ALL);
    let tabs = Tabs::new(vec![
        "Search",
        "Settings"
    ]).block(upper_block)
    .select(model.selected_tab);
    
    let window_area = lower_block.inner(chunks[1].clone());

    frame.render_widget(tabs, chunks[0]);
    frame.render_widget(lower_block, chunks[1]);
    match model.current_window {
        None => {},
        Some(Window::Search) => search_view(model, frame, window_area),
        Some(Window::Settings) => settings_view(model, frame, window_area),
    }
}

fn search_view(model: &mut Model, frame: &mut Frame, area: Rect) {
    frame.render_widget(Paragraph::new("SEARCH ME"), area);
}

fn settings_view(model: &mut Model, frame: &mut Frame, area: Rect) {
    frame.render_widget(Paragraph::new("hello world"), area);
}
