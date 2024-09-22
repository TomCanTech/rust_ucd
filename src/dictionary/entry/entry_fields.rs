use crate::error::{Error, Result};

#[derive(PartialEq, Debug)]

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
#[derive(PartialEq, Debug)]
pub struct DefinitionData {
    pub pos: i64,
    pub definition: String
}

impl DefinitionData {
    pub fn new(pos: Option<&i64>, def: Option<&String>) -> Option<Self>{
        match pos {
            None => def.map(|def| {DefinitionData{
                 pos: 0,
                 definition: def.to_string()
                }}),
            Some(pos) => match def {
                None => Some({DefinitionData{
                        pos: *pos,
                        definition: "NO DEFINITION ASSOCIATED".to_string()
                    }}),
                Some(def) => Some({DefinitionData{
                    pos: *pos,
                    definition: def.to_string()
                }})
            }
        }
    }
}