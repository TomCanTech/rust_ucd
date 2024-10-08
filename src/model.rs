use crate::{dictionary::Dictionary, message::{Message, SettingsMsg, TabMsg}};

pub struct Model{
    pub dictionary: Option<Dictionary>,
    pub running_state: RunningState,
    pub current_window: Option<Window>,
    pub selected_tab: usize,
    pub last_selected_tab: usize,
    pub settings_state: SettingsState,
}

impl Default for Model {
    fn default() -> Self {
        Model{
            dictionary: None,
            running_state: RunningState::Running,
            current_window: None,
            selected_tab: 0,
            last_selected_tab: 0,
            settings_state: SettingsState::default(),
        }
    }
}

pub fn update(model: &mut Model, msg: Message) -> Option<Message> {
    match msg{
        Message::Tab(TabMsg::MoveTabLeft) => {
            if model.selected_tab >= 1 {
                model.selected_tab -= 1;
                model.last_selected_tab = model.selected_tab;
            }  
        }
        Message::Tab(TabMsg::MoveTabRight) => {
            if model.selected_tab < 1 {
                model.selected_tab += 1;
                model.last_selected_tab = model.selected_tab;
            }
        }
        Message::Tab(TabMsg::SelectTab) => {
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
        Message::Settings(SettingsMsg::SelectMenu) => {
        }
        Message::Settings(SettingsMsg::LeaveMenu) => {
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

pub struct SettingsState{
    pub menus: Vec<Box<dyn Menu>>,
    pub selected_menu: Option<usize>
}

pub trait Menu {
    fn items(&self) -> Vec<&String>;
    fn selected_item(&self) -> usize;
}

impl Default for SettingsState {
    fn default() -> Self {
        SettingsState{
            menus: vec![],
            selected_menu: None
        }
    }
}

pub struct WritingSystemMenu{
    pub items: Vec<(String, i64)>,
    pub selected: usize
}

impl Menu for WritingSystemMenu{
    fn items(&self) -> Vec<&String> {
        self.items.iter().map(|f| &f.0).collect()
    }
    fn selected_item(&self) -> usize {
        self.selected
    }
}