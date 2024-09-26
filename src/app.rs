use std::collections::HashMap;

use crate::dictionary::{entry::Entry, Dictionary};

pub enum CurrentTab{
    Entries,
    Search,
    Edit,
    SelectingTab
}

pub struct App{
    pub dictionary: Option<Dictionary>,
    pub current_tab: CurrentTab,
    pub currently_editing: Option<Entry>,
    pub exit: bool
}

impl App {
    pub fn new() -> Self {
        App { dictionary: None, current_tab: CurrentTab::SelectingTab, currently_editing: None, exit: false}
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