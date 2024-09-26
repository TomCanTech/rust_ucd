pub mod dictionary;
pub mod error;
pub mod app;
pub mod ui;
pub use self::error::{Error, Result};

use ui::ui;
use app::{App, CurrentTab};
use clap::Parser;
use crossterm::event::{self, Event,KeyCode};
use ratatui::Terminal;
use dictionary::Dictionary;
use ratatui::prelude::Backend;
use rusqlite::Connection;
use std::path::PathBuf;

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
    let mut app = App::new();
    let app_result = run_app(&mut terminal, &mut app);
    ratatui::restore();

    if let Ok(pos_mess) = app_result {

    } else if let Err(err) = app_result {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>,app: &mut App ) -> Result<()> {
    while !app.exit{
        terminal.draw(|f| ui(f,app))?;
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release{
                continue;
            }
            match app.current_tab {
                CurrentTab::SelectingTab => match key.code {
                    KeyCode::Esc => {
                        app.exit = true
                    }
                    _ => {}
                }
                _ => {}
            }
        }
    }
    Ok(())
}
