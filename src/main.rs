pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>;

fn main() -> Result<()> {
    Ok(())
}
