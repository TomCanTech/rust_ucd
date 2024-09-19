use super::content::Content;

#[derive(PartialEq, Eq, Ord)]
pub struct Entry {
    pub id: i64,
    pub data_head: Content,
    pub data: Vec<Content>,
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Content::partial_cmp(&self.data_head, &other.data_head)
    }
}