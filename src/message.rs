use crate::model::Model;
use crate::Result;
use crossterm::event;
use ratatui::prelude::*;
pub enum Message {
    Tab(TabMessage),
}

enum TabMessage {
    MoveTabRight,
    MoveTabLeft,
    SelectTab,

}

pub fn handle_event(model: &Model) -> Result<Option<Message>> {
    Ok(None)
}

fn handle_key(key: event::KeyEvent) -> Option<Message> {
    None
}