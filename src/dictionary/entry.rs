use std::ops::Neg;

pub mod entry_fields;
use crate::dictionary::entry::entry_fields::Relative;

pub struct Entry {
    id: i64,
    headword: Vec<(String, i64)>,
    relatives: Option<Vec<Relative>>,
    pos: Vec<i64>,
    definition: Vec<String>,
    notes: Option<String>,
}
