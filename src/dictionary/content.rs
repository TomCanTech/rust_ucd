#[derive(Eq)]
pub enum Content {
    Independent(String),
    Codependent(String, String),
    Associated(String, i64),
    Mapped(i64),
    Multiple(Vec<Content>),
}

impl Content {
    pub fn parse(contents: String) -> Self {
        if contents.contains(';') {
            Self::Multiple(
                contents
                    .split(';')
                    .map(|ind| Self::parse(ind.to_owned()))
                    .collect(),
            )
        } else if contents.contains('=') {
            let left_right: Vec<&str> = contents.split('=').collect();
            Self::Codependent(left_right[0].to_string(), left_right[1].to_string())
        } else if contents.contains(':') {
            let left_right: Vec<&str> = contents.split(':').collect();
            Self::Associated(left_right[0].to_string(), left_right[1].parse().unwrap())
        } else if contents.chars().all(|chr| chr.is_numeric()) {
            Self::Mapped(contents.parse().unwrap())
        } else {
            Self::Independent(contents)
        }
    }
}

impl Ord for Content{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match Content::partial_cmp(&self, other) {
            None => std::cmp::Ordering::Equal,
            Some(order) => order
        }
    }
}

impl PartialEq for Content{
    fn eq(&self, other: &Self) -> bool {
        match self {
            Content::Independent(content) => {
                if let Content::Independent(other_content) = other{
                    content == other_content
                } else {
                    false 
                }
            } 
            Content::Codependent(left,_) => {
                if let Content::Codependent(other_left,_ ) = other {
                    left == other_left
                } else {
                    false
                }
            }
            Content::Associated(_,id) => {
                if let Content::Associated(_,other_id ) = other{
                    id == other_id
                } else {
                    false
                }
            }
            Content::Mapped(id) => {
                if let Content::Mapped(other_id) = other {
                    id == other_id
                } else {
                    false
                }
            }
            Content::Multiple(contents) => {
                if let Content::Multiple(other_contents) = other {
                    contents.iter().all(|content| other_contents.contains(content)) && 
                    other_contents.iter().all(|content| contents.contains(content)) &&
                    contents.len() == other_contents.len()
                } else {
                    false
                }
            }
    }
    }
}
impl PartialOrd for Content{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self {
            Content::Independent(content) => {
                if let Content::Independent(other_content) = other{
                    return String::partial_cmp(content, other_content)
                } else {
                    None
                }
            } 
            Content::Codependent(left,_) => {
                if let Content::Codependent(other_left,_ ) = other {
                    return String::partial_cmp(left, other_left)
                } else {
                    None
                }
            }
            Content::Associated(_,id) => {
                if let Content::Associated(_,other_id ) = other{
                    return i64::partial_cmp(id, other_id)
                } else {
                    None
                }
            }
            Content::Mapped(id) => {
                if let Content::Mapped(other_id) = other {
                    return i64::partial_cmp(id, other_id)
                } else {
                    None
                }
            }
            Content::Multiple(contents) => {
                if let Content::Multiple(other_contents) = other {
                    return Content::partial_cmp(&contents[0], &other_contents[0]) 
                } else {
                    None
                }
            }
    }}
}
