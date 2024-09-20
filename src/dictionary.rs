pub mod entry;

use crate::dictionary::entry::Entry;
use crate::error::Result;
use clap::builder::Str;
use entry::EntryBuilder;
use rusqlite::{params, Connection};
use std::collections::HashMap;

pub struct Dictionary {
    entries: Vec<Entry>,
    writ_system: HashMap<i64, (String, String)>,
    pos: HashMap<i64, (String, String)>,
}

impl Dictionary {
    pub fn new(conn: &Connection) -> Result<Self> {
        let dic = Dictionary {
            entries: Self::get_entries(conn)?,
            writ_system: Self::get_writ_system_data(conn)?,
            pos: Self::get_pos_data(conn)?,
        };
        Ok(dic)
    }
    fn get_entries(conn: &Connection) -> Result<Vec<Entry>> {
        let mut stmt = conn.prepare("SELECT id, headword, mutation, relatives, pos, definition, notes FROM entries")?;
        let entries_iter = stmt.query_map(params![], |row| {
            Ok(EntryBuilder::new(row.get(0)?)
                .headword(row.get(1)?)
                .mutation(row.get(2)?)
                .relatives(row.get(3)?)
                .pos(row.get(4)?)
                .definition(row.get(5)?)
                .notes(row.get(6)?)
                .build())
        })?;

        Ok(entries_iter
            .filter(|entry_res| entry_res.is_ok())
            .map(|entry_ok| entry_ok.unwrap())
            .collect())
    }
    fn get_writ_system_data(conn: &Connection) -> Result<HashMap<i64, (String, String)>> {
        let mut stmt = conn.prepare("SELECT id, short, long FROM writ_system")?;
        let system_iter = stmt.query_map(params![], |row| {
            let mut triple: (i64, (String, String)) = (0, (String::new(), String::new()));
            triple.0 = row.get(0)?;
            triple.1 .0 = row.get(1)?;
            triple.1 .1 = row.get(2)?;
            Ok(triple)
        })?;
        let mut writ_system_data: HashMap<i64, (String, String)> = HashMap::new();
        system_iter
            .filter(|triple| triple.is_ok())
            .for_each(|triple| {
                let triple = triple.unwrap();
                writ_system_data.insert(triple.0, triple.1);
            });
        Ok(writ_system_data)
    }
    fn get_pos_data(conn: &Connection) -> Result<HashMap<i64, (String, String)>> {
        let mut stmt = conn.prepare("SELECT id, short, long FROM pos")?;
        let pos_iter = stmt.query_map(params![], |row| {
            let mut triple: (i64, (String, String)) = (0, (String::new(), String::new()));
            triple.0 = row.get(0)?;
            triple.1 .0 = row.get(1)?;
            triple.1 .1 = row.get(2)?;
            Ok(triple)
        })?;
        let mut pos_data: HashMap<i64, (String, String)> = HashMap::new();
        pos_iter.filter(|triple| triple.is_ok()).for_each(|triple| {
            let triple = triple.unwrap();
            pos_data.insert(triple.0, triple.1);
        });
        Ok(pos_data)
    }

    pub fn print_entries(&self) {
        self.entries
            .iter()
            .for_each(|entry| println!("{:?}", entry));
    }
}
