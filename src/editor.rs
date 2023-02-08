use notan::draw::Draw;
use notan::draw::DrawShapes;
use notan::draw::Font;
use notan::prelude::*;

use crate::document::Document;

#[derive(AppState)]
pub struct Editor {
    pub font: Font,
    pub font2: Font,
    pub font3: Font,
    pub font4: Font,
    pub doc: Option<Document>,
    pub state: EditorState,
    pub cursor_pos: Position,
    pub quit: bool,
}

pub enum EditorState {
    Edit,
    FileManager,
    Splash,
}

impl Editor {
    pub fn move_cursor(&mut self, key: KeyCode) {
        match key {
            KeyCode::Right => {
                if self.cursor_pos.x < self.doc.as_mut().unwrap().lines[self.cursor_pos.y].len() {
                    self.cursor_pos.x += 1;
                }
            }
            KeyCode::Left => {
                if self.cursor_pos.x > 0 {
                    self.cursor_pos.x -= 1;
                }
            }
            KeyCode::Up => {
                if self.cursor_pos.y > 0 {
                    self.cursor_pos.y -= 1;
                    if self.cursor_pos.x > self.doc.as_mut().unwrap().lines[self.cursor_pos.y].len()
                    {
                        self.cursor_pos.x = self.doc.as_mut().unwrap().lines[self.cursor_pos.y]
                            .len()
                            .saturating_sub(1);
                    }
                }
            }
            KeyCode::Down => {
                if self.cursor_pos.y < self.doc.as_mut().unwrap().lines.len().saturating_sub(1) {
                    self.cursor_pos.y += 1;
                    if self.cursor_pos.x > self.doc.as_mut().unwrap().lines[self.cursor_pos.y].len()
                    {
                        self.cursor_pos.x = self.doc.as_mut().unwrap().lines[self.cursor_pos.y]
                            .len()
                            .saturating_sub(1);
                    }
                }
            }
            _ => {}
        }
        eprintln!("{:?}", &self.cursor_pos);
    }
    pub fn process_char(&mut self, character: char) {
        match self.state {
            EditorState::Splash => {}
            EditorState::Edit => {
                eprintln!("Processing char!");
                eprintln!("{:?}", character);
                self.doc
                    .as_mut()
                    .unwrap()
                    .insert(&self.cursor_pos, character);
                if character == '\n' || character == '\r' {
                    self.move_cursor(KeyCode::Down);
                    eprintln!(
                        "Current line len: {:?}",
                        self.doc.as_mut().unwrap().lines[self.cursor_pos.y].len()
                    );
                    //self.cursor_pos.x = self.doc.as_mut().unwrap().lines[self.cursor_pos.y].len();
                    self.cursor_pos.x = 0;
                } else {
                    self.move_cursor(KeyCode::Right);
                }
            }
            _ => {}
        }
    }
    pub fn process_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Left | KeyCode::Up | KeyCode::Right | KeyCode::Down => {
                self.move_cursor(key);
            }
            KeyCode::Back => match self.doc.as_mut().unwrap().delete(&self.cursor_pos) {
                (_, KeyCode::Left) => self.move_cursor(KeyCode::Left),
                (num, KeyCode::Up) => {
                    if self.cursor_pos.y != 0 {
                        self.cursor_pos.y = self.cursor_pos.y.saturating_sub(1);
                        self.cursor_pos.x = num;
                    }
                }
                _ => {}
            },
            KeyCode::Escape => {
                self.quit = true;
            }
            _ => {}
        }
    }
}

#[derive(Debug)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}
