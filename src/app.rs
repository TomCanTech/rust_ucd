use std::{collections::HashMap};
use crate::{dictionary::Dictionary, Result};
use crossterm::{event::{self, Event, KeyCode, KeyEvent, KeyEventKind}, terminal};
use ratatui::{prelude::*,widgets::*,DefaultTerminal};

const MAX_TAB_INDEX: usize = 2;
const TAB_UNSELECTED: usize = 8;

pub struct App {
    pub state: AppState,
    pub dic: Dictionary,
    pub selected_tab_index: usize,
    pub tab: Option<usize>,
    pub exit: bool
}

#[derive(PartialEq)]
pub enum AppState{
    SelectingTab,
    InTab
}

impl App {
    pub fn new() -> Self{
        App{
        state: AppState::SelectingTab,
        dic: {Dictionary{
            entries: vec![],
            writ_systems: HashMap::new(),
            pos: HashMap::new()
        }},
        selected_tab_index: 0,
        tab: None, 
        exit: false
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        let outer_block = Block::bordered();
        let inner_block = Block::bordered();

        let outer_area = frame.area();
        let inner_area = outer_block.inner(outer_area);
        frame.render_widget(Tabs::new(vec!["Search", "Entries", "Edit"]).bold().block(outer_block).select(self.selected_tab_index)
        ,outer_area);
        frame.render_widget(Paragraph::new("TEST").block(inner_block), inner_area);
    }
    fn handle_events(&mut self) -> Result<()>{
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }
    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Right => if self.selected_tab_index != MAX_TAB_INDEX {self.selected_tab_index += 1},
            KeyCode::Left => if self.selected_tab_index >= 1 && self.selected_tab_index < MAX_TAB_INDEX+1  { self.selected_tab_index-= 1},
            KeyCode::Enter => if self.state == AppState::SelectingTab{match self.tab {
                Some(_) => {},
                None => {self.tab = Some(self.selected_tab_index); self.selected_tab_index = TAB_UNSELECTED}
            }}
            KeyCode::Esc => if self.state == AppState::SelectingTab {match self.tab {
                Some(_) => {self.tab = None; self.selected_tab_index = 0},
                None => {self.exit()}
            }},
            _ => {}
        }
    }
    fn exit(&mut self) {
        self.exit = true;
    }
}
impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let inner = Paragraph::new("DAMN").block(Block::bordered());
        Tabs::new(vec!["Search", "Entries", "Edit"]).bold().block(Block::bordered()).select(self.selected_tab_index).render(area, buf);
    }
}