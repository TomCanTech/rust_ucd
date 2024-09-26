use std::io;
use crate::{dictionary::Dictionary, Result};
use ratatui::{
    crossterm::event::{self, KeyCode, KeyEventKind},
    style::Stylize,
    widgets::Paragraph,
    DefaultTerminal,
};

pub struct App {
    dic: Dictionary,
    exit: bool
}

fn run_ui() -> Result<()> {
    let mut terminal = ratatui::init();
    terminal.clear()?;
    let app_result = run_app(terminal);
    ratatui::restore();
    app_result
}

fn run_app(mut terminal: DefaultTerminal) -> Result<()> {
    loop {
        terminal.draw(|frame| {
            let greeting = Paragraph::new("Hello Ratatui! (press 'q' to quit)")
                .white()
                .on_blue();
            frame.render_widget(greeting, frame.area());
        })?;

        if let event::Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(());
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn terminal_works() {
        run_ui();
    }
}