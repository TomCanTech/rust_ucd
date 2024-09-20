pub mod entry;

use entry::EntryBuilder;
use rusqlite::{params, Connection};

use crate::dictionary::entry::Entry;
use crate::error::{Error, Result};
pub struct Dictionary {
    entries: Vec<Entry>,
}

impl Dictionary {
    pub fn new(conn: &Connection) -> Result<Self> {
        let dic = Dictionary {
            entries: Self::get_entries(conn)?,
        };
        Ok(dic)
    }
    fn get_entries(conn: &Connection) -> Result<Vec<Entry>> {
        let mut stmt = conn.prepare(
            "SELECT id, headword, mutation, relatives, pos, definition, notes FROM entries",
        )?;
        let entries_iter = stmt.query_map(params![], |row| {
            Ok(EntryBuilder::new(row.get(0)?)
                .headword(row.get(1)?)
                .mutation(row.get(2)?)
                .relatives(row.get(3)?)
                .pos(row.get(4)?)
                .definition(row.get(5)?)
                .notes(row.get(5)?)
                .build())
        })?;
        Ok(entries_iter
            .filter(|entry_res| entry_res.is_ok())
            .map(|entry_ok| entry_ok.unwrap())
            .collect())
    }
    pub fn print_entries(&self) {
        self.entries.iter()
            .for_each(|entry| println!("{:?}", entry));
    }
}

