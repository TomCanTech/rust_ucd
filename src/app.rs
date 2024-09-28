use std::collections::HashMap;

use crate::dictionary::{entry::Entry, Dictionary};
use tui_textarea::{TextArea};

pub enum CurrentTab{
    Entries,
    Search,
    Edit,
    Settings,
    SelectingTab
}

pub struct App<'a>{
    pub dictionary: Option<Dictionary>,
    pub current_tab: CurrentTab,
    pub tab_index: u16,
    pub currently_editing: Option<Entry>,
    pub query: TextArea<'a>,
    pub exit: bool
}

impl App <'_>{
    pub fn new() -> Self {
        App { dictionary: None, current_tab: CurrentTab::SelectingTab, currently_editing: None,tab_index: 0, query: TextArea::default(),exit: false}
    }
    pub fn save_entry(&mut self) {
        match &mut self.dictionary {
            None => {self.dictionary = Some(Dictionary{
                entries: vec![],
                writ_systems: HashMap::new(),
                pos: HashMap::new()
            })},
            Some(dic) => {
                match &self.currently_editing {
                    None => {},
                    Some(e) => {dic.entries.push(e.clone()); self.currently_editing = None},
                }
            }
        }
    }
    pub fn toggle_editing(&mut self){
        todo!()
    }    
}