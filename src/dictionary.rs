pub mod entry;

use crate::error::Result;
use entry::{Entry, EntryBuilder};

use rusqlite::{params, Connection};
use std::collections::HashMap;

pub use String as Short;
pub use String as Long;

#[derive(PartialEq, Debug)]
pub struct Dictionary {
    pub entries: Vec<Entry>,
    pub writ_systems: HashMap<i64, (Short, Long)>,
    pub pos: HashMap<i64, (Short, Long)>,
}

impl Dictionary {
    pub fn new(conn: &Connection) -> Result<Self> {
        let dic = Dictionary {
            entries: Self::init_entries(conn)?,
            writ_systems: Self::init_writ_systems(conn)?,
            pos: Self::init_pos(conn)?,
        };
        Ok(dic)
    }
    fn init_entries(conn: &Connection) -> Result<Vec<Entry>> {
        let mut stmt = conn.prepare(
            "SELECT id, headwords, mutation, relatives, pos, definitions, notes FROM entries",
        )?;
        let entries_iter = stmt.query_map(params![], |row| {
            Ok(EntryBuilder::new(row.get(0)?)
                .headwords(row.get(1)?)
                .mutation(row.get(2)?)
                .relatives(row.get(3)?)
                .definitions(row.get(4)?, row.get(5)?)
                .notes(row.get(6)?)
                .build())
        })?;

        Ok(entries_iter.flatten().collect())
    }
    fn init_writ_systems(conn: &Connection) -> Result<HashMap<i64, (Short, Long)>> {
        let mut stmt = conn.prepare("SELECT id, short, long FROM writ_system")?;
        let system_iter = stmt.query_map(params![], |row| {
            Ok((row.get(0)?, (row.get(1)?, row.get(1)?)))
        })?;
        let writ_systems: HashMap<i64, (Short, Long)> =
            system_iter.filter_map(|sys| sys.ok()).collect();
        Ok(writ_systems)
    }
    fn init_pos(conn: &Connection) -> Result<HashMap<i64, (String, String)>> {
        let mut stmt = conn.prepare("SELECT id, short, long FROM pos")?;
        let pos_iter = stmt.query_map(params![], |row| {
            Ok((row.get(0)?, (row.get(1)?, row.get(1)?)))
        })?;
        let pos: HashMap<i64, (Short, Long)> = pos_iter.filter_map(|sys| sys.ok()).collect();
        Ok(pos)
    }

    pub fn print_entries(&self) {
        for entry in &self.entries {
            println!("{entry:?}")
        }
    }
}
