use crate::{model::Window, Result};
use crossterm::event::{self, KeyCode};
use crate::model::Model;
use ratatui::prelude::*;
pub enum Message {
    Tab(TabMessage),
    ExitWindow,
    ExitApp
}

pub enum TabMessage {
    MoveTabRight,
    MoveTabLeft,
    SelectTab,
}

pub fn handle_event(model: &Model) -> Result<Option<Message>> {
    if let event::Event::Key(key) = event::read()?{
        if key.kind == event::KeyEventKind::Press {
            match model.current_window{
                None => return Ok(tab_handle_key(key)),
                Some(Window::Search) => return Ok(search_handle_key(key)),
                Some(Window::Settings) => return Ok(settings_handle_key(key))
            }
        }
    }
    Ok(None)
}

fn tab_handle_key(key: event::KeyEvent) -> Option<Message> {
    match key.code{
        KeyCode::Left => Some(Message::Tab(TabMessage::MoveTabLeft)),
        KeyCode::Right => Some(Message::Tab(TabMessage::MoveTabRight)),
        KeyCode::Enter => Some(Message::Tab(TabMessage::SelectTab)),
        KeyCode::Esc => Some(Message::ExitApp),
        _ => None,
    }
}

fn search_handle_key(key: event::KeyEvent) -> Option<Message> {
    match key.code{
        KeyCode::Esc => Some(Message::ExitWindow),
        _ => None
    }
}

fn settings_handle_key(key: event::KeyEvent) -> Option<Message> {
    match key.code{
        KeyCode::Esc => Some(Message::ExitWindow),
        _ => None
    }
}
