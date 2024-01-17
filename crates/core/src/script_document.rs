use std::{io::{self, BufReader, BufRead}, path::Path, fs::File};
use encoding_rs_io::DecodeReaderBytes;
use ropey::{Rope, RopeBuilder};
use tree_sitter::InputEdit;

#[derive(Debug, Clone)]
pub struct ScriptDocument {
    pub(crate) rope: Rope,
    pub(crate) edits: Vec<InputEdit>
}

impl ScriptDocument {
    pub fn from_str(s: &str) -> Self {
        let rope = Rope::from_str(s);

        Self {
            rope,
            edits: Vec::new()
        }
    }

    pub fn from_file<P>(path: P) -> Result<Self, io::Error> 
    where P: AsRef<Path> {
        let f = File::open(&path)?;
        let decoder = DecodeReaderBytes::new(f);
        let mut reader = BufReader::new(decoder);

        let mut builder = RopeBuilder::new();
        let mut line = String::new();
        while let Ok(b) = reader.read_line(&mut line) {
            if b == 0 {
                break;
            }
            builder.append(&line);
            line.clear();
        }

        let rope = builder.finish();

        Ok(Self {
            rope,
            edits: Vec::new()
        })
    }

    pub fn text_at(&self, range: lsp_types::Range) -> String {
        let start_char = self.rope.line_to_char(range.start.line as usize) + range.start.character as usize;
        let end_char = self.rope.line_to_char(range.end.line as usize) + range.end.character as usize;
        self.rope.slice(start_char..end_char).to_string()
    }

    //TODO needs testing!
    pub fn edit(&mut self, event: &lsp_types::TextDocumentContentChangeEvent) {
        let point_offset = string_point_offset(&event.text);

        if let Some(range) = &event.range {
            self.edits.push(InputEdit {
                start_byte: self.rope.position_to_byte(&range.start),
                old_end_byte: self.rope.position_to_byte(&range.end),
                new_end_byte: self.rope.position_to_byte(&range.start) + event.text.bytes().len(),
                start_position: position_to_point(range.start.clone()),
                old_end_position: position_to_point(range.end.clone()),
                new_end_position: tree_sitter::Point {
                    row: range.start.line as usize + point_offset.row,
                    column: if point_offset.row == 0 { 
                        range.start.character as usize + point_offset.column 
                    } else { 
                        point_offset.column 
                    },
                },
            });

            if range.start != range.end {
                self.rope.remove(self.rope.position_to_char(&range.start)..self.rope.position_to_char(&range.end));
            }

            if !event.text.is_empty() {
                self.rope.insert(self.rope.position_to_char(&range.start), &event.text);
            }
        } else {
            // We will not specify any edit in the case fo a full text sync.
            // This will mean that tree-sitter will not reuse nodes,
            // but since we will be mainly using incremental text sync
            // this case should happen only when the document is first opened.

            self.rope = Rope::from_str(&event.text);
        }
    }
}

// There are a ton of name collisions between lsp_types, ropey and tree-sitter.
// That's why I'm putting them in a single module and use only lsp_types outside to not drive myself crazy.

fn position_to_point(position: lsp_types::Position) -> tree_sitter::Point {
    tree_sitter::Point {
        row: position.line as usize,
        column: position.character as usize
    }
}

trait RopeUtils {
    fn position_to_char(&self, position: &lsp_types::Position) -> usize;
    fn position_to_byte(&self, position: &lsp_types::Position) -> usize;
}

impl RopeUtils for Rope {
    fn position_to_char(&self, position: &lsp_types::Position) -> usize {
        self.line_to_char(position.line as usize) + position.character as usize
    }

    fn position_to_byte(&self, position: &lsp_types::Position) -> usize {
        self.char_to_byte(self.position_to_char(position))
    }
}

// returns a relative position of the end of the given string
fn string_point_offset(s: &str) -> tree_sitter::Point {
    let mut row = 0;
    let mut column = 0;
    for c in s.chars() {
        if c == '\n' {
            row += 1;
            column = 0;
        } else {
            column += 1;
        }
    }

    tree_sitter::Point {
        row, column
    }
}