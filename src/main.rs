pub mod dictionary;
pub mod error;
pub use self::error::{Error, Result};

use clap::Parser;
use dictionary::Dictionary;
use std::path::PathBuf;
use rusqlite::Connection;

#[derive(Parser, Debug)]
#[command(version,about, long_about = None)]
struct Args{
    #[arg(short, long, value_name = "FILE", help = "Read from database file")]
    db_path: PathBuf
}

fn main() -> Result<()> {
    let args = Args::parse();
    let db_connection = Connection::open(args.db_path)?;
    let dic = Dictionary::new(&db_connection)?;
    dic.print_entries();
    Ok(())
}
