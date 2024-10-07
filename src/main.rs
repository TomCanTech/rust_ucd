pub mod dictionary;
pub mod error;
pub mod model;
pub mod view;
pub mod message;

pub use self::error::{Error, Result};
use clap::Parser;
use model::{Model,RunningState};
use dictionary::Dictionary;
use rusqlite::Connection;
use std::path::PathBuf;
use tui_textarea::{CursorMove, Input, Key};
use crate::view::view;

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
    let dictionary = Dictionary::new(&db_connection)?;

    //UI Logic
    let mut terminal = ratatui::init();
    let mut model = Model::default();
    model.dictionary = Some(dictionary);
    
    while model.running_state != RunningState::Done {
        terminal.draw(|f| view(&mut model, f))?;
        let mut current_msg = message::handle_event(&model)?;

        while current_msg.is_some() {
            current_msg = model::update(&mut model, current_msg.unwrap());
        }
    }

    terminal.clear()?;
    ratatui::restore();

    Ok(())
}
