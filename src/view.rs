extern crate crossterm;
use crate::game::{Entity, Model, Structures};
use crossterm::terminal::ClearType;
use crossterm::{execute, terminal};
use std::io::{BufWriter, stdout, Stdout, Write};
use std::mem::size_of;

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
        let (c, r) = model.get_size();
        let mut buffer = BufWriter::with_capacity(c*r*size_of::<char>()+r, &self.out);
        let cells = model.get_cells();
        for row in 0..r {
            for col in 0..c {
                let cell = cells[row * c + col];
                if cell.occupied() {
                    buffer.write_fmt(format_args!("{}", entity_to_char(cell.entity.as_ref().unwrap()))).unwrap();
                }
                else {
                    buffer.write_fmt(format_args!("{}", structure_to_char(&cell.structure))).unwrap();
                }
            }
            buffer.write_fmt(format_args!("\n")).unwrap();
        }
        buffer.flush().unwrap();
    }

    pub fn start(&mut self) -> crossterm::Result<()>{
        execute!(self.out, terminal::EnterAlternateScreen, crossterm::cursor::Hide)?;
        terminal::enable_raw_mode()?;
        Ok(())
    }

    pub fn end(&mut self) -> crossterm::Result<()>{
        terminal::disable_raw_mode()?;
        execute!(self.out, terminal::LeaveAlternateScreen, crossterm::cursor::Show)?;
        Ok(())
    }

    pub fn clear_screen(&mut self) -> crossterm::Result<()>{
        execute!(self.out, terminal::Clear(ClearType::All))?;
        Ok(())
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