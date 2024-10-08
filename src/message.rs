use crate::model::Model;
use crate::{model::Window, Result};
use crossterm::event::{self, KeyCode};
pub enum Message {
    Tab(TabMsg),
    Settings(SettingsMsg),
    Menu(MenuMsg),
    ExitWindow,
    ExitApp,
}

pub enum TabMsg {
    MoveTabRight,
    MoveTabLeft,
    SelectTab,
}

pub fn handle_event(model: &Model) -> Result<Option<Message>> {
    if let event::Event::Key(key) = event::read()? {
        if key.kind == event::KeyEventKind::Press {
            match model.current_window {
                None => return Ok(tab_handle_key(key)),
                Some(Window::Search) => return Ok(search_handle_key(key)),
                Some(Window::Settings) => return Ok(settings_handle_key(key, model)),
            }
        }
    }
    Ok(None)
}

fn tab_handle_key(key: event::KeyEvent) -> Option<Message> {
    match key.code {
        KeyCode::Left => Some(Message::Tab(TabMsg::MoveTabLeft)),
        KeyCode::Right => Some(Message::Tab(TabMsg::MoveTabRight)),
        KeyCode::Enter => Some(Message::Tab(TabMsg::SelectTab)),
        KeyCode::Esc => Some(Message::ExitApp),
        _ => None,
    }
}

fn search_handle_key(key: event::KeyEvent) -> Option<Message> {
    match key.code {
        KeyCode::Esc => Some(Message::ExitWindow),
        _ => None,
    }
}

pub enum SettingsMsg {
    SelectMenu,
    LeaveMenu,
    NextMenu,
    PreviousMenu,
}

fn settings_handle_key(key: event::KeyEvent, model: &Model) -> Option<Message> {
    if model.settings_state.selected_menu.is_none() {
        match key.code {
            KeyCode::Up => Some(Message::Settings(SettingsMsg::PreviousMenu)),
            KeyCode::Down => Some(Message::Settings(SettingsMsg::NextMenu)),
            KeyCode::Enter => Some(Message::Settings(SettingsMsg::SelectMenu)),
            KeyCode::Esc => Some(Message::ExitWindow),
            _ => None,
        }
    } else {
        match key.code {
            KeyCode::Esc => Some(Message::Settings(SettingsMsg::LeaveMenu)),
            _ => menu_handle_key(key, model),
        }
    }
}

pub enum MenuMsg {
    NextItem,
    PreviousItem,
    SelectItem
}

fn menu_handle_key(key: event::KeyEvent, model: &Model) -> Option<Message> {
    match key.code {
        KeyCode::Up => Some(Message::Menu(MenuMsg::PreviousItem)),
        KeyCode::Down => Some(Message::Menu(MenuMsg::NextItem)),
        KeyCode::Enter => Some(Message::Menu(MenuMsg::SelectItem)),
        _ => None
    }
}