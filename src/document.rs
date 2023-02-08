use notan::prelude::KeyCode;

use crate::editor::Position;
use std::{fmt::Debug, fs};

pub struct Document {
    pub file_name: Option<String>,
    pub lines: Vec<Line>,
}

pub struct Line {
    pub content: String,
    pub len: usize,
}

impl From<&str> for Line {
    fn from(slice: &str) -> Self {
        Self {
            content: String::from(slice),
            len: slice.len(),
        }
    }
}

impl Line {
    pub fn as_bytes(&self) -> &[u8] {
        self.content.as_bytes()
    }
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
    pub fn insert(&mut self, at: usize, c: char) {
        if at >= self.len() {
            self.content.push(c);
            self.len += 1;
            return;
        }
        self.content.insert(at, c);
        self.len += 1;
    }
    pub fn append(&mut self, new: &Self) {
        self.content = format!("{}{}", self.content, new.content);
        self.len += new.len;
    }
    pub fn split(&mut self, at: usize) -> Self {
        let split: String = self.content.chars().skip(at).collect();
        let split_len: usize = split.len();
        let new_content: String = self.content.chars().take(at).collect();
        self.len = new_content.len();
        self.content = new_content;
        Self {
            content: split,
            len: split_len,
        }
    }
    pub fn delete(&mut self, at: usize) {
        self.content.remove(at);
        self.len -= 1;
    }
}

impl Document {
    pub fn empty() -> Self {
        Self {
            lines: vec![Line::from("")],
            file_name: None,
        }
    }
    pub fn empty_with_name(filename: &str) -> Self {
        Self {
            lines: vec![Line::from("")],
            file_name: Some(filename.to_string()),
        }
    }
    pub fn open(filename: &str) -> Result<Self, std::io::Error> {
        let contents = fs::read_to_string(filename)?;
        let mut lines = Vec::new();
        for line in contents.lines() {
            lines.push(Line::from(line));
        }
        Ok(Self {
            lines,
            file_name: Some(filename.to_string()),
        })
    }
    fn insert_newline(&mut self, at: &Position) {
        if at.y == self.lines.len() {
            self.lines.insert(at.y + 1, Line::from(""));
            return;
        }
        let new_row = self.lines[at.y].split(at.x);
        self.lines.insert(at.y + 1, new_row);
    }
    pub fn insert(&mut self, at: &Position, c: char) {
        if c == '\n' || c == '\r' {
            self.insert_newline(at);
            return;
        }
        self.lines[at.y].insert(at.x + 1, c);
    }
    pub fn delete(&mut self, at: &Position) -> (usize, KeyCode) {
        eprintln!("Deleting char at: {:?}", at);
        if at.x == 0 && at.y != 0 {
            let append = self.lines.remove(at.y);
            let new_cursor = self.lines[at.y - 1].len();
            self.lines[at.y - 1].append(&append);
            return (new_cursor, KeyCode::Up);
        } else if self.lines[at.y].len() > 0 {
            self.lines[at.y].delete(at.x - 1);
            return (0, KeyCode::Left);
        }
        (0, KeyCode::Escape)
    }
    pub fn as_str(&mut self) -> String {
        let mut s = String::new();
        for line in &mut self.lines {
            s.push_str(&line.content);
            s.push_str("\n");
        }
        s.truncate(s.trim().len());
        s
    }
}
