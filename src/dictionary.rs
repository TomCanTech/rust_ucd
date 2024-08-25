use crate::entry::Entry;
use std::collections::HashMap;
struct Dictionary {
    entries: Vec<Entry>,
    system_map: HashMap<i32, (String, String)>,
    pos_map: HashMap<i32, (String, String)>,
}
