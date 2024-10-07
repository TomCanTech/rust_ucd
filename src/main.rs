pub mod dictionary;
pub mod error;
pub mod model;
pub mod ui;
pub mod message;
pub use self::error::{Error, Result};

use clap::Parser;
use crossterm::event::{Event,KeyCode};
use ratatui::Terminal;
use dictionary::Dictionary;
use ratatui::prelude::Backend;
use rusqlite::Connection;
use std::path::PathBuf;
use tui_textarea::{CursorMove, Input, Key};

#[derive(Parser, Debug)]
#[command(version,about, long_about = None)]
struct Args {
    #[arg(short, long, value_name = "FILE", help = "Read from database file")]
    db_path: PathBuf,
}

fn main() -> Result<()> {
    //Parse Logic
    let args: Args = Args::parse();
    //Init dictionary
    let db_connection = Connection::open(args.db_path)?;
    let dic = Dictionary::new(&db_connection)?;

    //UI Logic
    let mut terminal = ratatui::init();
    terminal.clear()?;
    ratatui::restore();

    Ok(())
}
