use std::{borrow::Cow, fs::File, io::{self, BufRead, BufReader}};
use abs_path::AbsPath;
use encoding_rs_io::DecodeReaderBytes;
use ropey::{Rope, RopeBuilder};
use tree_sitter as ts;
use lsp_types as lsp;


#[derive(Debug, Clone)]
pub struct ScriptDocument {
    pub(crate) rope: Rope,
    pub(crate) edits: Vec<ts::InputEdit>
}

impl ScriptDocument {
    pub fn from_str(s: &str) -> Self {
        let rope = Rope::from_str(s);

        Self {
            rope,
            edits: Vec::new()
        }
    }

    pub fn from_file(path: &AbsPath) -> Result<Self, io::Error> {
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

    pub fn text_at(&self, range: lsp::Range) -> Cow<'_, str> {
        let start_char = self.rope.position_to_char(&range.start);
        let end_char = self.rope.position_to_char(&range.end);
        self.rope.slice(start_char..end_char).into()
    }

    /// Replace a part of the document, e.g. add a new line of text
    pub fn edit(&mut self, event: &lsp::TextDocumentContentChangeEvent) {
        let point_offset = string_point_offset(&event.text);

        if let Some(range) = &event.range {
            self.edits.push(ts::InputEdit {
                start_byte: self.rope.position_to_byte(&range.start),
                old_end_byte: self.rope.position_to_byte(&range.end),
                new_end_byte: self.rope.position_to_byte(&range.start) + event.text.bytes().len(),
                start_position: position_to_point(range.start.clone()),
                old_end_position: position_to_point(range.end.clone()),
                new_end_position: ts::Point {
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

    /// Replace the entire parsed content of the document
    pub fn replace(&mut self, new_text: &str) {
        self.rope = Rope::from_str(new_text);
        self.edits.clear();
    }
}

// There are a ton of name collisions between lsp_types, ropey and tree-sitter.
// That's why I'm putting them in a single module and use only lsp_types outside to not drive myself crazy.

fn position_to_point(position: lsp::Position) -> ts::Point {
    ts::Point {
        row: position.line as usize,
        column: position.character as usize
    }
}

trait RopeUtils {
    fn position_to_char(&self, position: &lsp::Position) -> usize;
    fn position_to_byte(&self, position: &lsp::Position) -> usize;
}

impl RopeUtils for Rope {
    fn position_to_char(&self, position: &lsp::Position) -> usize {
        self.line_to_char(position.line as usize) + position.character as usize
    }

    fn position_to_byte(&self, position: &lsp::Position) -> usize {
        self.char_to_byte(self.position_to_char(position))
    }
}

// returns a relative position of the end of the given string
fn string_point_offset(s: &str) -> ts::Point {
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

    ts::Point {
        row, column
    }
}





#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_edit() {
        let mut doc = ScriptDocument::from_str(
r#"
exec function test_func() {
    var player: CR4Player;
}

"#);


        // add //
        doc.edit(&lsp::TextDocumentContentChangeEvent {
            range: Some(lsp::Range {
                start: lsp::Position::new(2, 26),
                end: lsp::Position::new(2, 26),
            }),
            range_length: None,
            text: "\n".into(),
        });

        doc.edit(&lsp::TextDocumentContentChangeEvent {
            range: Some(lsp::Range {
                start: lsp::Position::new(3, 0),
                end: lsp::Position::new(3, 0),
            }),
            range_length: None,
            text: "    player = thePlayer;".into(),
        });


        assert_eq!(doc.rope.to_string(), 
r#"
exec function test_func() {
    var player: CR4Player;
    player = thePlayer;
}

"#);

        assert_eq!(doc.edits[0], ts::InputEdit { 
            start_byte: 55, 
            old_end_byte: 55, 
            new_end_byte: 56, 
            start_position: ts::Point::new(2, 26), 
            old_end_position: ts::Point::new(2, 26), 
            new_end_position: ts::Point::new(3, 0) 
        });

        assert_eq!(doc.edits[1], ts::InputEdit { 
            start_byte: 56, 
            old_end_byte: 56, 
            new_end_byte: 79, 
            start_position: ts::Point::new(3, 0), 
            old_end_position: ts::Point::new(3, 0), 
            new_end_position: ts::Point::new(3, 23) 
        });



        // remove //
        doc.edit(&lsp::TextDocumentContentChangeEvent {
            range: Some(lsp::Range {
                start: lsp::Position::new(4, 1),
                end: lsp::Position::new(5, 0),
            }),
            range_length: None,
            text: "".into(),
        });

        doc.edit(&lsp::TextDocumentContentChangeEvent {
            range: Some(lsp::Range {
                start: lsp::Position::new(1, 18),
                end: lsp::Position::new(1, 23),
            }),
            range_length: None,
            text: "".into(),
        });
        

        assert_eq!(doc.rope.to_string(), 
r#"
exec function test() {
    var player: CR4Player;
    player = thePlayer;
}
"#);

        assert_eq!(doc.edits[2], ts::InputEdit { 
            start_byte: 81, 
            old_end_byte: 82, 
            new_end_byte: 81, 
            start_position: ts::Point::new(4, 1), 
            old_end_position: ts::Point::new(5, 0), 
            new_end_position: ts::Point::new(4, 1) 
        });

        assert_eq!(doc.edits[3], ts::InputEdit { 
            start_byte: 19, 
            old_end_byte: 24, 
            new_end_byte: 19, 
            start_position: ts::Point::new(1, 18), 
            old_end_position: ts::Point::new(1, 23), 
            new_end_position: ts::Point::new(1, 18) 
        });



        // replace //
        doc.edit(&lsp::TextDocumentContentChangeEvent {
            range: Some(lsp::Range {
                start: lsp::Position::new(1, 14),
                end: lsp::Position::new(1, 18),
            }),
            range_length: None,
            text: "player_getter".into(),
        });


        assert_eq!(doc.rope.to_string(), 
r#"
exec function player_getter() {
    var player: CR4Player;
    player = thePlayer;
}
"#);

        assert_eq!(doc.edits[4], ts::InputEdit { 
            start_byte: 15, 
            old_end_byte: 19, 
            new_end_byte: 28, 
            start_position: ts::Point::new(1, 14), 
            old_end_position: ts::Point::new(1, 18), 
            new_end_position: ts::Point::new(1, 27) 
        });
    }
}