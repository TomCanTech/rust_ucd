
use crate::{dictionary::{self, Dictionary}, message::{Message, TabMessage}};

pub struct Model {
    pub dictionary: Option<Dictionary>,
    pub running_state: RunningState,
    pub current_window: Option<Window>,
    pub selected_tab: usize,
    pub last_selected_tab: usize
}

impl Default for Model {
    fn default() -> Self {
        Model{
            dictionary: None,
            running_state: RunningState::Running,
            current_window: None,
            selected_tab: 0,
            last_selected_tab: 0
        }
    }
}

pub fn update(model: &mut Model, msg: Message) -> Option<Message> {
    match msg{
        Message::Tab(TabMessage::MoveTabLeft) => {
            if model.selected_tab >= 1 {
                model.selected_tab -= 1;
                model.last_selected_tab = model.selected_tab;
            }  
        }
        Message::Tab(TabMessage::MoveTabRight) => {
            if model.selected_tab < 1 {
                model.selected_tab += 1;
                model.last_selected_tab = model.selected_tab;
            }
        }
        Message::Tab(TabMessage::SelectTab) => {
            model.selected_tab = 8;
            match model.last_selected_tab {
                0 => model.current_window = Some(Window::Search),
                1 => model.current_window = Some(Window::Settings),
                _ => {}
            }}
        Message::ExitApp => {
            model.running_state = RunningState::Done;
        }
        Message::ExitWindow => {
            model.current_window = None; 
            model.selected_tab = model.last_selected_tab;
        }
        _ => {}
       } 
    None
}

#[derive(PartialEq)]
pub enum RunningState {
    Running,
    Done,
}

#[derive(PartialEq)]
pub enum Window {
    Search,
    Settings
}