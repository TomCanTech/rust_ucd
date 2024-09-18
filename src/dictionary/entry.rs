use super::content::Content;

pub struct Entry {
    pub id: i64,
    pub data_head: Content,
    pub data: Vec<Content>,
}
