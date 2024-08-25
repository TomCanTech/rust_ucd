#[derive(Debug)]
pub struct Entry {
    pub index: i32,
    pub headword: String,
    pub relatives: Option<Vec<i32>>,
    pub part_of_speech: i32,
    pub definition: Vec<String>,
    pub notes: String,
}

enum EntryRelative {
    EntryAncestor(i32),
    EntryDescendant(i32),
    EntryCousin(i32)
}

impl Entry {
    pub fn build(
        index: i32,
        headword: String,
        relatives: Option<Vec<i32>>,
        part_of_speech: i32,
        definition: Vec<String>,
        notes: String) -> Self 
        {
            Entry{
                index: index,
                headword: headword,
                relatives: relatives,
                part_of_speech: part_of_speech,
                definition: definition,
                notes: notes
            }
    }
    pub fn contains(&self,query: &String) -> bool {
        self.hw_contains(query) || self.def_contains(query)
        }
    fn hw_contains(&self,query: &String) -> bool {
        self.headword.to_lowercase()
        .contains(&query.to_lowercase())
    }
    fn def_contains(&self,query: &String) -> bool {
        self.definition.iter()
        .any(|def_slice| def_slice.to_lowercase().contains(&query.to_lowercase()))
    }
}

impl EntryRelative{
    pub fn build(rel_id: String) -> Result<Self,String>{
        match rel_id.chars()
            .nth(0)
            .unwrap() {
            'a' => Ok(EntryRelative::EntryAncestor(rel_id[1..].parse().unwrap())),
            'd' => Ok(EntryRelative::EntryDescendant(rel_id[1..].parse().unwrap())),
            'c' => Ok(EntryRelative::EntryCousin(rel_id[1..].parse().unwrap())),
            _ => Err("Relative type not implemented".to_string())
        }
    }
}