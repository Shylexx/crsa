use notan::prelude::*;
use notan::text::*;

use crate::document::Document;

#[derive(AppState)]
pub struct Editor {
    pub font: Font,
    pub doc: Option<Document>,
    pub state: EditorState,
    pub cursor_pos: Position,
}

pub enum EditorState {
    Edit,
    FileManager,
    Splash,
}

impl Editor {
    pub fn move_cursor(&mut self, direction: Dir) {
        match direction {
            Dir::Right => {
                if self.doc.as_mut().unwrap().lines[self.cursor_pos.y].len() > self.cursor_pos.x {
                    self.cursor_pos.x += 1;
                }
            }
            Dir::Left => {
                if self.cursor_pos.x > 0 {
                    self.cursor_pos.x -= 1;
                }
            }
            Dir::Up => {
                if self.cursor_pos.y > 0 {
                    self.cursor_pos.y -= 1;
                }
            }
            Dir::Down => {
                if self.cursor_pos.y > self.doc.as_mut().unwrap().lines.len() {
                    self.cursor_pos.y += 1;
                }
            }
        }
    }
    pub fn process_char(&mut self, character: char) {
        match self.state {
            EditorState::Splash => {}
            EditorState::Edit => {
                self.doc
                    .as_mut()
                    .unwrap()
                    .insert(&self.cursor_pos, character);
                self.move_cursor(Dir::Right);
            }
            _ => {}
        }
    }
}

pub enum Dir {
    Left,
    Right,
    Up,
    Down,
}

pub struct Position {
    pub x: usize,
    pub y: usize,
}
