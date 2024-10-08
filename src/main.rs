pub mod dictionary;
pub mod error;
pub mod model;
pub mod view;
pub mod message;

pub use self::error::{Error, Result};
use clap::Parser;
use model::{Model,RunningState, WritingSystemMenu};
use dictionary::Dictionary;
use rusqlite::Connection;
use tui_menu::{MenuItem, MenuState};
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

    //Init model
    let mut terminal = ratatui::init();
    let mut model = Model::default();
    let writ_system_menu = WritingSystemMenu{
        items: dictionary.writ_systems.iter().map(|f| (f.1.0.clone(), f.0.clone())).collect(),
        selected: 0
    };
    model.settings_state.menus.push(Box::new(writ_system_menu));
    model.dictionary = Some(dictionary);
    //Run app
    while model.running_state != RunningState::Done {
        terminal.draw(|f| view(&mut model, f))?;
        let mut current_msg = message::handle_event(&model)?;
        while current_msg.is_some() {
            current_msg = model::update(&mut model, current_msg.unwrap());
        }
    }
    //Restore terminal
    terminal.clear()?;
    ratatui::restore();

    Ok(())
}
