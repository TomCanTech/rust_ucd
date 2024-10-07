pub mod entry_fields;

use super::{Long, Short};
use entry_fields::{Definition, Relative};
use std::collections::HashMap;

#[derive(PartialEq, Debug, Clone)]
pub struct Entry {
    id: Option<i64>,
    headwords: Vec<(String, i64)>,
    mutation: Option<i64>,
    relatives: Option<Vec<Relative>>,
    definitions: Vec<Definition>,
    notes: Option<String>,
}

impl Entry {
    fn get_writ_systems(
        &self,
        writ_systems_map: &HashMap<i64, (Short, Long)>,
    ) -> Vec<Option<(Short, Long)>> {
        let mut writ_systems: Vec<Option<(Short, Long)>> = Vec::with_capacity(self.headwords.len());
        for hw in &self.headwords {
            match writ_systems_map.get(&hw.1) {
                None => writ_systems.push(None),
                Some(sys) => writ_systems.push(Some(sys.clone())),
            }
        }
        writ_systems
    }
    fn get_pos(&self, pos_map: &HashMap<i64, (Short, Long)>) -> Vec<Option<(Short, Long)>> {
        let mut pos: Vec<Option<(Short, Long)>> = Vec::with_capacity(self.definitions.len());
        for def in &self.definitions {
            match pos_map.get(&def.pos_id) {
                None => pos.push(None),
                Some(p) => pos.push(Some(p.clone())),
            }
        }
        pos
    }
}

pub struct EntryBuilder {
    id: Option<i64>,
    headwords: Vec<(String, i64)>,
    mutation: Option<i64>,
    relatives: Option<Vec<Relative>>,
    definitions: Vec<Definition>,
    notes: Option<String>,
}

impl EntryBuilder {
    pub fn new(id: Option<i64>) -> EntryBuilder {
        EntryBuilder {
            id,
            headwords: vec![],
            mutation: None,
            relatives: None,
            definitions: vec![],
            notes: None,
        }
    }

    pub fn headwords(mut self, headwords: String) -> Self {
        let mut headword_vec: Vec<(String, i64)> = Vec::with_capacity(4);
        for hw in headwords.split(';') {
            let hw_tuple_vec: Vec<&str> = hw.split(':').collect();
            let word = String::from(hw_tuple_vec[0]);
            if let Ok(writ_system_id) = hw_tuple_vec[1].parse() {
                headword_vec.push((word, writ_system_id))
            } else {
                headword_vec.push((word, 0));
            }
        }
        self.headwords = headword_vec;
        self
    }
    pub fn mutation(mut self, mutation: Option<i64>) -> Self {
        match mutation {
            Some(mutation) => match mutation {
                1..=5 => {
                    self.mutation = Some(mutation);
                    self
                }
                _ => self,
            },
            None => self,
        }
    }
    pub fn relatives(mut self, relatives: Option<String>) -> Self {
        match relatives {
            Some(relatives) => {
                let relatives = relatives.split(';');
                let mut relatives_vec = Vec::with_capacity(8);
                for relative in relatives {
                    if let Ok(rel) = Relative::new(relative) {
                        relatives_vec.push(rel)
                    }
                }
                self.relatives = Some(relatives_vec);
                self
            }
            None => self,
        }
    }
    pub fn definitions(mut self, pos: String, definition: String) -> Self {
        let mut def_data_vec: Vec<Definition> = Vec::with_capacity(6);
        for (pos_id, def_cont) in pos.split(';').zip(definition.split(';')) {
            def_data_vec.push(Definition::new(pos_id, def_cont));
        }
        self.definitions = def_data_vec;
        self
    }
    pub fn notes(mut self, notes: Option<String>) -> Self {
        match notes {
            None => self,
            Some(notes) => {
                self.notes = Some(notes);
                self
            }
        }
    }
    pub fn build(self) -> Entry {
        Entry {
            id: self.id,
            headwords: self.headwords,
            mutation: self.mutation,
            relatives: self.relatives,
            definitions: self.definitions,
            notes: self.notes,
        }
    }
}
