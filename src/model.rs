use tui_menu::MenuState;
use crate::{dictionary::Dictionary, message::{Message, SettingsMsg, TabMsg}};

pub struct Model{
    pub dictionary: Option<Dictionary>,
    pub running_state: RunningState,
    pub current_window: Option<Window>,
    pub selected_tab: usize,
    pub last_selected_tab: usize,
    pub settings_state: SettingsState,
    pub selected_setting: Option<SettingsMenu>
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
            selected_setting: None
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
            model.settings_state.writ_system_menu.select();
            model.selected_setting = Some(SettingsMenu::WritingSystem);
        }
        Message::Settings(SettingsMsg::LeaveMenu) => {
            model.settings_state.writ_system_menu.reset();
            model.selected_setting = None
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
    pub writ_system_menu: tui_menu::MenuState<i64>,
}

impl Default for SettingsState {
    fn default() -> Self {
        SettingsState{
            writ_system_menu: MenuState::new(vec![])
        }
    }
}

pub enum SettingsMenu{
    WritingSystem
}