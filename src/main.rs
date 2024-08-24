use rusqlite::{params, types::Null, Connection, Result};
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

    let example_entry = Entry {
        index: 0,
        headword: "example".to_string(),
        relatives: Some(vec![2,3,4]),
        part_of_speech: 1,
        definition: vec!["definition_1".to_string(), "definition_2".to_string()],
        notes: "notes_example".to_string(),
    };

    connection.execute(
        "INSERT INTO entry (headword, relatives, part_of_speech, definition, notes) VALUES (?1,?2,?3,?4,?5)", 
        (&example_entry.headword,
            vec_to_string(example_entry.relatives.unwrap_or_else(|| vec![])),
            &example_entry.part_of_speech,
            vec_to_string(example_entry.definition),
            &example_entry.notes))?;

    let mut stmt = connection
        .prepare("Select id, headword, relatives, part_of_speech, definition,notes FROM entry")?;
    let entry_iter = stmt.query_map([], |row| {
        Ok(Entry {
            index: row.get(0)?,
            headword: row.get(1)?,
            relatives: relatives_to_vec(row.get(2).unwrap()),
            part_of_speech: row.get(3)?,
            definition: definitions_to_vec(row.get(4).unwrap()),
            notes: row.get(5)?,
        })
    })?;
    for entry in entry_iter {
        println!("Found entry {:?}", entry.unwrap());
    }
    Ok(())
}

#[derive(Debug)]
struct Entry {
    index: i32,
    headword: String,
    relatives: Option<Vec<i32>>,
    part_of_speech: i32,
    definition: Vec<String>,
    notes: String,
}
fn vec_to_string<T: ToString>(vector: Vec<T>) -> String {
    let mut def_string = String::new();

    for member in vector {
        def_string.push_str(&member.to_string());
        def_string.push('#')
    }

    def_string
}

fn relatives_to_vec(relatives: String) -> Option<Vec<i32>> {
    match relatives.len() {
        0 => None,
        other => Some (
            relatives
                .split('#')
                .filter(|num| num.len() != 0)
                .map(|num| num.parse().unwrap())
                .collect())
        }

}

fn definitions_to_vec(definitions: String) -> Vec<String> {
    definitions.split('#')
    .map(|def| def.to_string())
    .filter(|def| def.len() != 0)
    .collect()
}
