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
