extern crate crossterm;
use crate::game::{Entity, Model, Structures};
use crossterm::terminal::ClearType;
use crossterm::{execute, terminal};
use std::io::{stdout, Stdout, Write};
use std::ptr::write;

pub struct Screen {
    out: Stdout,
}

impl Screen {
    pub fn new() -> Screen {
        Screen {
            out: stdout(),
        }
    }

    pub fn render(&mut self, model: &Model) {
        for row in model.get_cells() {
            for cell in row {
                if cell.occupied() {
                    write!(self.out, "{}", entity_to_char(cell.entity.as_ref().unwrap()));
                }
                else {
                    write!(self.out, "{}", structure_to_char(&cell.structure));
                }
            }
            write!(self.out, "\n");
        }
    }

    pub fn start(&mut self) {
        execute!(self.out, terminal::EnterAlternateScreen);
        terminal::enable_raw_mode();
    }

    pub fn end(&mut self) {
        terminal::disable_raw_mode();
        execute!(self.out, terminal::LeaveAlternateScreen);
    }

    pub fn clear_screen(&mut self) {
        execute!(self.out, terminal::Clear(ClearType::All));
    }
}


fn entity_to_char(e: &Entity) -> char{
    match e { Entity::Player => '@' }
}

fn structure_to_char(s: &Structures) -> char {
    match s {
        Structures::Void => ' ',
        Structures::Wall => '#',
        Structures::Floor => '.'
    }
}