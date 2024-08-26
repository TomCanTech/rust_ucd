use crate::entry::Entry;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use rusqlite::{Connection, Result};

struct Dictionary {
    entries: Vec<Entry>,
    system_map: Rc<RefCell<HashMap<i32, (String, String)>>>,
    pos_map: Rc<RefCell<HashMap<i32, (String, String)>>>,
}
impl Dictionary{
    fn from_sqlite(conn: &rusqlite::Connection) -> Result<Self>{
        let mut sys_map = Dictionary::make_hash(conn, "SELECT key, abbreviation, full FROM system_map")?;
        let mut pos_map = Dictionary::make_hash(conn, "SELECT key, abbreviation, full FROM pos_map")?;
        let mut dictionary = Dictionary{
            entries: vec![],
            system_map: Rc::new(RefCell::new(sys_map)),
            pos_map: Rc::new(RefCell::new(pos_map)),
        };
        let mut entry_retrieve_stmt = conn.prepare("SELECT id, headword, relatives,part_of_speech,definition,notes FROM entry").unwrap();

        Ok(dictionary)
    }
    fn make_hash(conn: &rusqlite::Connection,retrieve_stmt: &str) -> Result<HashMap<i32,(String,String)>> {
        let mut map_retrieve_stmt = conn.prepare(retrieve_stmt)?;
        let map_iter = map_retrieve_stmt.query_map([],   |row| {
            Ok((row.get_unwrap::<usize,i32>(0), row.get::<usize,String>(1),row.get::<usize,String>(2)))
        });

        let mut map: HashMap<i32,(String,String)> = HashMap::new();
        for pair in map_iter.unwrap() {
            match pair {
                Ok((x,y,z)) => {map.insert(x,(y?,z?));},
                Err(_) => break
            }
        };
        Ok(map)
    }

}






#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::rc::Rc;

    use super::Dictionary;
    use crate::entry::entryfields::{Headword, EntryRelative};
    use crate::Entry;
    #[test]
    fn share_maps() {
        let mut dic_1 = Dictionary {
            entries: vec![],
            system_map: Rc::new(RefCell::new(HashMap::from([
                (1 as i32, ("KK".to_string(), "Kernewek Kemmyn".to_string())),
                (2 as i32,("UCR".to_string(), "Unified Cornish Revised".to_string())),
            ]))),
            pos_map: Rc::new(RefCell::new(HashMap::from([
                (1 as i32, ("NF".to_string(),"Feminine Noun".to_string())),
                (3 as i32, ("NM".to_string(),"Masculine Noun".to_string()))
            ]))),
        };

        let entry_1 = Entry {
            id: 32,
            headword: vec![
                Headword {
                    sys_headword: "hi".to_string(),
                    sys_key: 0,
                },
                Headword {
                    sys_headword: "hey".to_string(),
                    sys_key: 1,
                },
            ],
            system_map: Rc::clone(&dic_1.system_map),
            relatives: Some(vec![EntryRelative::EntryAncestor(32), EntryRelative::EntryCousin(48)]),
            part_of_speech: vec![1, 1],
            pos_map: Rc::clone(&dic_1.pos_map),
            definition: vec!["greeting".to_string(), "(A call for attention)".to_string()],
            notes: String::from("Further variation to be found"),
        };
        let entry_2 = Entry {
            id: 32,
            headword: vec![
                Headword {
                    sys_headword: "there".to_string(),
                    sys_key: 0,
                },
                Headword {
                    sys_headword: "dere".to_string(),
                    sys_key: 1,
                },
            ],
            system_map: Rc::clone(&dic_1.system_map),
            relatives: Some(vec![EntryRelative::EntryAncestor(32), EntryRelative::EntryCousin(48)]),
            part_of_speech: vec![3, 1],
            pos_map: Rc::clone(&dic_1.pos_map),
            definition: vec!["there".to_string(), "yonder".to_string()],
            notes: String::from("No greater variations"),
        };
        dic_1.entries.push(entry_1);
        dic_1.entries.push(entry_2);

        let pos_val_1 = dic_1.entries[0].pos_map.borrow().get(&1).unwrap().clone();
        let pos_val_2 = dic_1.entries[1].pos_map.borrow().get(&1).unwrap().clone();
        let pos_val_3 = dic_1.pos_map.borrow().get(&1).unwrap().clone();

        assert_eq!(pos_val_1,pos_val_2);
        assert_eq!(pos_val_2,pos_val_3);
    }
    #[test]
    fn get_maps() {

    }
}
