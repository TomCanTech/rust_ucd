pub mod entry_fields;

use crate::dictionary::entry::entry_fields::Relative;
use crate::error::{Error, Result};

#[derive(PartialEq, Debug)]
pub struct Entry {
    id: i64,
    headword: Vec<(String, i64)>,
    mutation: Option<i64>,
    relatives: Option<Vec<Relative>>,
    pos: Vec<i64>,
    definition: Vec<String>,
    notes: Option<String>,
}

impl Entry {
    fn new() -> Self {
        Entry {
            id: 0,
            headword: vec![],
            mutation: None,
            relatives: None,
            pos: vec![],
            definition: vec![],
            notes: None,
        }
    }
}
pub struct EntryBuilder {
    id: i64,
    pub headword: Vec<(String, i64)>,
    mutation: Option<i64>,
    relatives: Option<Vec<Relative>>,
    pos: Vec<i64>,
    definition: Vec<String>,
    notes: Option<String>,
}

impl EntryBuilder {
    pub fn new(id: i64) -> EntryBuilder {
        EntryBuilder {
            id,
            headword: vec![],
            mutation: None,
            relatives: None,
            pos: vec![],
            definition: vec![],
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
    pub fn pos(mut self, pos: String) -> Self {
        let pos = pos.split(';');
        let mut pos_vec: Vec<i64> = vec![];
        for p in pos {
            if let Ok(pos) = p.parse() {
                pos_vec.push(pos);
            }
        }
        self.pos = pos_vec;
        self
    }
    pub fn definition(mut self, definition: String) -> Self {
        let definition = definition.split(';');
        self.definition = definition.map(|def| def.to_string()).collect();
        self
    }
    pub fn notes(mut self, notes: String) -> Self {
        self.notes = Some(notes);
        self
    }
    pub fn build(self) -> Entry {
        Entry {
            id: self.id,
            headword: self.headword,
            mutation: self.mutation,
            relatives: self.relatives,
            pos: self.pos,
            definition: self.definition,
            notes: self.notes,
        }
    }
}

#[test]
fn builder_test() {
    let mut entry = Entry::new();
    let entry_from_builder = EntryBuilder::new(0).build();
    assert_eq!(entry, entry_from_builder)
}
