use crossterm::event::{Event, KeyCode, KeyModifiers};
use crate::game::{Events};
use crate::game::Direction::{N, S, E, W};


pub fn read() -> Events {
    match crossterm::event::read().unwrap() {
        Event::Key(key_event) => {
            if key_event.modifiers.is_empty() {
                match key_event.code {
                    KeyCode::Char(c) => {
                        match c {
                            'q' => Events::Quit,
                            _ => Events::Nothing
                        }
                    }
                    KeyCode::Up => Events::Move(N),
                    KeyCode::Down => Events::Move(S),
                    KeyCode::Right => Events::Move(E),
                    KeyCode::Left => Events::Move(W),
                    _ => Events::Nothing
                }
            }
            else {
                match (key_event.code, key_event.modifiers) {
                    (KeyCode::Char('c'), KeyModifiers::CONTROL) => Events::Quit,
                    (_, _) => Events::Nothing
                }
            }
        }
        _ => Events::Nothing
    }
}
