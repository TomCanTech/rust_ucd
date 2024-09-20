use crate::error::{Error, Result};

pub enum Relative {
    Ancestor(i64),
    Descendant(i64),
    Unknown(i64),
}

impl Relative {
    fn new(r: &str) -> Result<Self> {
        match r.chars().nth(0) {
            Some('a') => Ok(Self::Ancestor(r[1..].parse()?)),
            Some('d') => Ok(Self::Descendant(r[1..].parse()?)),
            Some('u') => Ok(Self::Unknown(r[1..].parse()?)),
            _ => Err(Error::InvalidLeadingChar(r.to_string())),
        }
    }
}
