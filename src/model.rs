use ratatui::widgets::ListState;

use crate::{
    dictionary::Dictionary,
    message::{MenuMsg, Message, SettingsMsg, TabMsg},
};

pub struct Model {
    pub dictionary: Option<Dictionary>,
    pub running_state: RunningState,
    pub current_window: Option<Window>,
    pub selected_tab: usize,
    pub last_selected_tab: usize,
    pub settings_state: SettingsState,
}

impl Default for Model {
    fn default() -> Self {
        Model {
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
    match msg {
        Message::ExitApp => {
            model.running_state = RunningState::Done;
            return None
        }
        Message::ExitWindow => {
            model.current_window = None;
            model.selected_tab = model.last_selected_tab;
            return None
        } 
        Message::Tab(tab_msg) => update_tab(model, tab_msg),
        Message::Settings(settings_msg) => update_settings(&mut model.settings_state, settings_msg),
        Message::Menu(menu_msg) => update_menu(model, menu_msg),
        _ => {}
    }
    None
}

fn update_tab(model: &mut Model, tab_msg: TabMsg) {
    match tab_msg{
        TabMsg::MoveTabLeft => {
        if model.selected_tab >= 1 {
            model.selected_tab -= 1;
            model.last_selected_tab = model.selected_tab;
        }
    }
        TabMsg::MoveTabRight => {
        if model.selected_tab < 1 {
            model.selected_tab += 1;
            model.last_selected_tab = model.selected_tab;
        }
    }
        TabMsg::SelectTab => {
        model.selected_tab = 8;
        match model.last_selected_tab {
            0 => model.current_window = Some(Window::Search),
            1 => {
                model.current_window = {
                    model.settings_state.menus_list_state.select(Some(0));
                    Some(Window::Settings)
                }
            }
            _ => {}
        }}}}

fn update_settings(settings_state: &mut SettingsState, settings_msg: SettingsMsg) {
    match settings_msg{
        SettingsMsg::NextMenu => {settings_state.menus_list_state.select_next();}
        SettingsMsg::PreviousMenu => {settings_state.menus_list_state.select_previous();}
        SettingsMsg::SelectMenu => {settings_state.selected_menu = Some(settings_state.menus_list_state.selected().unwrap());
            settings_state.menus.get_mut(settings_state.selected_menu.unwrap()).unwrap().list_state().select_first();
        },
        SettingsMsg::LeaveMenu => {settings_state.selected_menu = None},
}}

fn update_menu(model: &mut Model, menu_msg: MenuMsg) {
    match menu_msg {
        MenuMsg::NextItem => {            
            let selected_menu = model.settings_state.menus.get_mut(model.settings_state.selected_menu.unwrap()).unwrap();
            selected_menu.list_state().select_next();}
        MenuMsg::PreviousItem => {            
            let selected_menu = model.settings_state.menus.get_mut(model.settings_state.selected_menu.unwrap()).unwrap();
            selected_menu.list_state().select_previous();}
        MenuMsg::SelectItem => {
            let selected_menu = model.settings_state.menus.get_mut(model.settings_state.selected_menu.unwrap()).unwrap();
            selected_menu.list_state().select_first();
    }
}}

#[derive(PartialEq)]
pub enum RunningState {
    Running,
    Done,
}

#[derive(PartialEq)]
pub enum Window {
    Search,
    Settings,
}

pub struct SettingsState {
    pub menus: Vec<Box<dyn Menu>>,
    pub menus_list_state: ListState,
    pub selected_menu: Option<usize>
}

pub trait Menu {
    fn name(&self) -> &String;
    fn items(&self) -> Vec<&String>;
    fn list_state(&mut self) -> &mut ListState;
}

impl Default for SettingsState {
    fn default() -> Self {
        SettingsState {
            menus: vec![],
            menus_list_state: ListState::default(),
            selected_menu: None
        }
    }
}

pub struct WritingSystemMenu {
    pub name: String,
    pub items: Vec<(String, i64)>,
    pub menu_list_state: ListState,
}

impl Menu for WritingSystemMenu {
    fn name(&self) -> &String {
        &self.name
    }
    fn items(&self) -> Vec<&String> {
        self.items.iter().map(|f| &f.0).collect()
    }
    fn list_state(&mut self) -> &mut ListState {
        &mut self.menu_list_state
    }
}

impl Default for WritingSystemMenu {
    fn default() -> Self {
        WritingSystemMenu {
            name: String::from("Preferred Writing System"),
            items: vec![],
            menu_list_state: ListState::default(),
        }
    }
}
