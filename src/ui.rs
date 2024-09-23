use ratatui::{
    crossterm::event::{self, KeyCode, KeyEventKind},
    style::Stylize,
    widgets::Paragraph,
    DefaultTerminal,
};
use crate::{Error,Result};
use crate::*;

pub struct UserInterface {
    dictionary: Dictionary
}

pub fn init_terminal() -> Result<()> {
    let mut terminal = ratatui::init();
    terminal.clear()?;
    run(terminal)?;
    ratatui::restore();
    Ok(())
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    let mut greeting = Paragraph::new("Press B to set bg to black (press 'q' to quit)")
        .white()
        .on_blue();
    loop {
        terminal.draw(|frame| {
            frame.render_widget(greeting.clone(), frame.area());
        })?;
        if let event::Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(());
            }
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('B') {
                greeting = greeting.on_black();
            }
        }

    }
}