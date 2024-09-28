use ratatui::{text::Text,layout::{self, Constraint, Layout, Rect}, prelude::Direction, style::Style, symbols, text::Line, widgets::{Block, Borders, Paragraph, Tabs, Widget}, Frame};
use crate::{app::CurrentTab, App};
use tui_textarea::{TextArea,Input,Key};

pub fn ui(frame: &mut Frame, app: &mut App){
    let top_block = Block::new()
        .borders(Borders::TOP | Borders::LEFT | Borders::RIGHT); 
    let tabs = Tabs::new(vec!["Entries", "Search", "Edit", "Settings"])
        .select(app.tab_index as usize)
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
    let bottom_block_area = bottom_block.inner(chunks[1]);
    frame.render_widget(tabs, chunks[0]);        
    frame.render_widget(bottom_block, chunks[1]);
    match app.current_tab{
        CurrentTab::Search => render_search_screen(frame, app, centered_rect(30, 30, bottom_block_area)),
        CurrentTab::Entries => render_entries_screen(frame, app, bottom_block_area),
        _ => {}
    }
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

fn render_search_screen(frame: &mut Frame, app: &mut App, area: Rect) {
    let search_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Max(1),
            Constraint::Max(3)
        ])
        .split(area);
    frame.render_widget(Paragraph::new("Search").centered(), search_layout[0]);
    let search_block = Block::new().borders(Borders::all());
    app.query.set_block(search_block);
    frame.render_widget(&app.query, search_layout[1]);
}

fn render_entries_screen(frame: &mut Frame, app: &mut App, area: Rect) {
    let text = Text::from(
    match &app.dictionary {
        None => {vec![Line::from("foo"), Line::from("bar")]}
        Some(dic) => {
            dic.entries.iter().map(|entry| Line::from(format!("{entry:?}"))).collect()
        }
    });
    frame.render_widget(Paragraph::new(text), area); 
}