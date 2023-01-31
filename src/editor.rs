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
    pub fn move_cursor(&mut self, direction: Dir) {}
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

enum Dir {
    Left,
    Right,
    Up,
    Down,
}

pub struct Position {
    pub x: usize,
    pub y: usize,
}
