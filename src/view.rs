use crate::model::{Menu, Model, Window};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph, Tabs,List},
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

fn search_view(_model: &mut Model, frame: &mut Frame, area: Rect) {
    frame.render_widget(Paragraph::new("SEARCH ME"), area);
}

fn settings_view(model: &mut Model, frame: &mut Frame, area: Rect) {
    match model.settings_state.selected_menu {
        None => {
            let menu_list = List::new(model.settings_state.menus.iter().map(|f| f.name().as_str()))
                .highlight_symbol(">>>  ");
            frame.render_stateful_widget(menu_list, area, &mut model.settings_state.menus_list_state);
        }
        Some(menu_index) => {
            popup_view(frame, model.settings_state.menus.get_mut(menu_index).expect("Indexed into non-existent menu"));
        }
    }
}

fn popup_view(frame: &mut Frame, menu: &mut Box<dyn Menu>) {
    let menu_name = menu.name().clone();
    let popup_block = Block::new()
        .borders(Borders::ALL)
        .title(menu_name.as_str());
    let popup_area = center(frame.area(), Constraint::Percentage(80), Constraint::Percentage(80));
    let list_items: Vec<String> = menu.items().iter().map(|&f| f.clone()).collect();
    let list: List = List::new(list_items).block(popup_block).highlight_symbol(">>> ");
    frame.render_stateful_widget(list, popup_area, menu.list_state());
}

fn center(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect{ 
    let [area] = Layout::horizontal([horizontal])
        .flex(layout::Flex::Center)
        .areas(area);
    let [area] = Layout::vertical([vertical]).flex(layout::Flex::Center).areas(area);
    area
}