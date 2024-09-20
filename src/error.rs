use derive_more::From;

pub type Result<T> = core::result::Result<T, Error>;
#[derive(Debug, From)]
pub enum Error {
    #[from]
    InvalidLeadingChar(String),

    //External
    #[from]
    ParseInt(std::num::ParseIntError),
    #[from]
    Rusqlite(rusqlite::Error),
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for Error {}
