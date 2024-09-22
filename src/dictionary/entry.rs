pub mod entry_fields;

use entry_fields::{DefinitionData,Relative};
use std::cmp::max;

#[derive(PartialEq, Debug)]
pub struct Entry {
    id: i64,
    headword: Vec<(String, i64)>,
    mutation: Option<i64>,
    relatives: Option<Vec<Relative>>,
    definition_data: Vec<DefinitionData>,
    notes: Option<String>,
}

impl Entry {}
pub struct EntryBuilder {
    id: i64,
    headword: Vec<(String, i64)>,
    mutation: Option<i64>,
    relatives: Option<Vec<Relative>>,
    definition_data: Vec<DefinitionData>,
    notes: Option<String>,
}

impl EntryBuilder {
    pub fn new(id: i64) -> EntryBuilder{
        EntryBuilder {
            id,
            headword: vec![],
            mutation: None,
            relatives: None,
            definition_data: vec![],
            notes: None,
        }
    }

    pub fn headword(mut self, headword: String) -> Self {
        let headword = headword.split(';');
        let mut headword_vec: Vec<(String, i64)> = vec![];
        for headword in headword {
            let hw_tuple_vec: Vec<&str> = headword.split(':').collect();
            let left = hw_tuple_vec[0].to_string();
            if let Ok(right) = hw_tuple_vec[1].parse() {
                headword_vec.push((left, right))
            }
        }
        self.headword = headword_vec;
        self
    }
    pub fn mutation(mut self, mutation: i64) -> Self {
        self.mutation = Some(mutation);
        self
    }
    pub fn relatives(mut self, relatives: String) -> Self {
        let relatives = relatives.split(';');
        let mut relatives_vec = vec![];
        for relative in relatives {
            if let Ok(rel) = Relative::new(relative) {
                relatives_vec.push(rel)
            }
        }
        self.relatives = Some(relatives_vec);
        self
    }
    pub fn definition_data(mut self, pos: String, definition: String) -> Self {
        let pos_vec: Vec<i64> = pos.split(';').filter_map(|pos| pos.parse().ok()).collect();
        let def_vec: Vec<String> = definition.split(';').map(|def| def.to_string()).collect();
        let mut def_data_vec: Vec<DefinitionData> = vec![];
        for def_data_member in 0..max(pos_vec.len(), def_vec.len()) {
            let pos = pos_vec.get(def_data_member);
            let def = def_vec.get(def_data_member);
            match DefinitionData::new(pos, def) {
                None => {},
                Some(data) => def_data_vec.push(data),
            }
        }
        self.definition_data = def_data_vec;
        self
    }
    pub fn notes(mut self, notes: String) -> Self {
        self.notes = Some(notes);
        self
    }
    pub fn build(self) -> Entry{
        Entry {
            id: self.id,
            headword: self.headword,
            mutation: self.mutation,
            relatives: self.relatives,
            definition_data: self.definition_data,
            notes: self.notes,
        }

    }
}
