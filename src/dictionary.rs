use crate::entry::Entry;
use std::path::PathBuf;
struct Dictionary {
    path: PathBuf,
    entries: Vec<Entry>,
}

#[warn(unused)]
impl Dictionary {
    pub fn build_dic(dic_path: String) {}
    pub fn search_dic(query: String) {}
    pub fn render_self() {}
    pub fn print_dic() {}
}
