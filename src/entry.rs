use entryfields::{Headword,EntryRelative};
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
    pub index: i32,
    pub headword: Vec<Headword>,
    pub relatives: Option<Vec<EntryRelative>>,
    pub part_of_speech: Vec<i32>,
    pub definition: Vec<String>,
    pub notes: String,
}

mod entryfields;

impl Entry {
    pub fn build(
        index: i32,
        headword: Vec<Headword>,
        relatives: Option<Vec<EntryRelative>>,
        part_of_speech: Vec<i32>,
        definition: Vec<String>,
        notes: String,
    ) -> Self {
        Entry {
            index: index,
            headword: headword,
            relatives: relatives,
            part_of_speech: part_of_speech,
            definition: definition,
            notes: notes,
        }
    }
    pub fn contains(&self, query: &String) -> bool {
        self.def_contains(query)
    }    
    fn def_contains(&self, query: &String) -> bool {
        self.definition
            .iter()
            .any(|def_slice| def_slice.to_lowercase().contains(&query.to_lowercase()))
    }
    }
