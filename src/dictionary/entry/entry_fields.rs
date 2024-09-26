use crate::error::{Error, Result};

#[derive(PartialEq, Debug, Clone)]

pub enum Relative {
    Ancestor(i64),
    Descendant(i64),
    Unknown(i64),
}

impl Relative {
    pub fn new(r: &str) -> Result<Self> {
        match r.chars().nth(0) {
            Some('a') => Ok(Self::Ancestor(r[1..].parse()?)),
            Some('d') => Ok(Self::Descendant(r[1..].parse()?)),
            Some('u') => Ok(Self::Unknown(r[1..].parse()?)),
            _ => Err(Error::InvalidLeadingChar(r.to_string())),
        }
    }
}
#[derive(PartialEq, Debug, Clone)]
pub struct Definition {
    pub pos_id: i64,
    pub def_content: String,
}

impl Definition {
    pub fn new(pos_id: &str, def_content: &str) -> Self {
        match pos_id.parse() {
            Err(_) => Definition {
                pos_id: 0,
                def_content: String::from(def_content),
            },
            Ok(pos_id) => Definition {
                pos_id,
                def_content: String::from(def_content),
            },
        }
    }
}
