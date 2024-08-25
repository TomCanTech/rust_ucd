
#[derive(PartialEq, Debug)]
pub struct Headword {
    sys_headword: String,
    sys_map: i32,
}

#[derive(PartialEq, Debug)]
pub enum EntryRelative {
    EntryAncestor(i32),
    EntryDescendant(i32),
    EntryCousin(i32),
}

impl EntryRelative {
    pub fn build(rel_id: String) -> Option<Self> {
        if rel_id.len() > 1 {
            match rel_id.chars().nth(0).unwrap() {
                'a' => match EntryRelative::parse_rel_id(rel_id) {
                    Some(id) => Some(EntryRelative::EntryAncestor(id)),
                    None => None,
                },
                'd' => match EntryRelative::parse_rel_id(rel_id) {
                    Some(id) => Some(EntryRelative::EntryDescendant(id)),
                    None => None,
                },
                'c' => match EntryRelative::parse_rel_id(rel_id) {
                    Some(id) => Some(EntryRelative::EntryCousin(id)),
                    None => None,
                },
                _ => None,
            }
        } else {
            None
        }
    }
    fn parse_rel_id(rel_id: String) -> Option<i32> {
        match rel_id[1..].chars().all(|digit| digit.is_numeric()) {
            true => Some(rel_id[1..].parse().unwrap()),
            false => None,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_relatives() {
        let relative_1 = EntryRelative::EntryCousin(32);
        let relative_2 = EntryRelative::build("c32".to_string()).unwrap();
        assert_eq!(relative_1, relative_2);
    }
}
