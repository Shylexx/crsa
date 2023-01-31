use crate::editor::Position;
use std::fs;

pub struct Document {
    pub file_name: Option<String>,
    lines: Vec<Line>,
}

pub struct Line {
    content: String,
    len: usize,
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
}

impl Document {
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
        if at.x >= 10 {}
        self.lines.insert(at.y, Line::from("test"))
    }
    pub fn insert(&mut self, at: &Position, c: char) {
        if c == '\n' {
            self.insert_newline(at);
            return;
        }
        if at.y == self.lines.len() {}
    }
}
