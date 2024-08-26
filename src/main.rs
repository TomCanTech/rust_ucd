mod dictionary;
mod entry;

use entry::Entry;
use rusqlite::{Connection, Result};
fn main() -> Result<()> {
    let connection = Connection::open_in_memory()?;
    connection.execute(
        "CREATE TABLE IF NOT EXISTS entry (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                headword STRING NOT NULL,
                relatives STRING,
                part_of_speech INTEGER NOT NULL,
                definition STRING NOT NULL,
                notes STRING
            )",
        (),
    )?;
    let mut stmt = connection
        .prepare("Select id, headword, relatives, part_of_speech, definition,notes FROM entry")?;

    Ok(())
}

fn vec_to_string<T: ToString>(vector: Vec<T>) -> String {
    let mut def_string = String::new();

    for member in vector {
        def_string.push_str(&member.to_string());
        def_string.push('#')
    }
    def_string.pop();
    def_string
}

fn relatives_to_vec(relatives: String) -> Option<Vec<i32>> {
    match relatives.len() {
        0 => None,
        other => Some(
            relatives
                .split('#')
                .map(|num| num.parse().unwrap())
                .collect(),
        ),
    }
}

fn definitions_to_vec(definitions: String) -> Vec<String> {
    definitions.split('#').map(|def| def.to_string()).collect()
}
