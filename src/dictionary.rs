pub mod content;
pub mod entry;

use content::Content;
use entry::Entry;
use rusqlite::Connection;
use std::collections::HashMap;

pub struct Dictionary {
    entries: Vec<Entry>,
    shared_entry_data: HashMap<i64, Content>,
    entry_data_columns: Vec<String>,
}

impl Dictionary {
    fn get_entries(conn: &Connection) -> Self {
        let mut entry_stmt = conn.prepare("SELECT * FROM entries").unwrap();
        let entry_column_len = entry_stmt.column_count();
        let entries_iter = entry_stmt
            .query_map([], |row| {
                let mut data: Vec<Content> = vec![];
                for col in 2..entry_column_len {
                    data.push(Content::parse(row.get(col).unwrap()));
                }
                Ok(Entry {
                    id: row.get(0).unwrap(),
                    data_head: Content::parse(row.get(1).unwrap()),
                    data,
                })
            })
            .unwrap();
        Self {
            entries: entries_iter.map(|entry| entry.unwrap()).collect(),
            shared_entry_data: HashMap::new(),
            entry_data_columns: vec![],
        }
    }
    fn sort_entries(&self) {
        todo!()
    }
}
