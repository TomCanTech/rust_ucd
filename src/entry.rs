use entryfields::{EntryRelative, Headword};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(PartialEq, Debug)]

/*
    index = unique entry identifier (integer)
    headword = vector of system-specific headword, system tuples
    relatives = optional vector of relative types: ancestor (makes up current entry), descendant (made up of current entry) and cousin (shares unclear link to current entry)
    part_of_speech = vector of parts of speech that headword belongs to
    definition = vector of definitions corresponding to the part of speech
    notes = simple string
*/
pub struct Entry {
    pub id: i32,
    pub headword: Vec<Headword>,
    pub system_map: Rc<RefCell<HashMap<i32, (String, String)>>>,
    pub relatives: Option<Vec<EntryRelative>>,
    pub part_of_speech: Vec<i32>,
    pub pos_map: Rc<RefCell<HashMap<i32, (String, String)>>>,
    pub definition: Vec<String>,
    pub notes: String,
}

pub mod entryfields;

impl Entry {
    pub fn contains(&self, query: &String) -> bool {
        self.def_contains(query)
    }
    fn def_contains(&self, query: &String) -> bool {
        self.definition
            .iter()
            .any(|def_slice| def_slice.to_lowercase().contains(&query.to_lowercase()))
    }
}
