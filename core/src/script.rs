use std::{
    fs::File,
    io::{self, Write, BufReader, Read, BufRead},
    path::{PathBuf, Path}
};

use ropey::{Rope, RopeBuilder};
use thiserror::Error;
use tree_sitter::{Parser, Tree, LanguageError};
use encoding_rs_io::DecodeReaderBytes;

#[derive(Debug)]
pub struct Script {
    path: PathBuf,
    rope: Rope,
    parse_tree: Tree
}

#[derive(Debug, Error)]
pub enum ScriptError {
    #[error("file failed to open")]
    FileOpenError(#[source] io::Error),
    #[error("failed to read the file")]
    FileReadError(#[source] io::Error),
    #[error("parser failed to initialize")]
    ParserInitError(#[source] LanguageError)
}

impl Script {
    pub fn from_file<P>(path: P) -> Result<Self, ScriptError>
    where P: AsRef<Path> {
        use ScriptError::*;

        let f = File::open(&path).map_err(FileOpenError)?;
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

        let mut parser = Parser::new();
        parser.set_language(tree_sitter_witcherscript::language()).map_err(ParserInitError)?;

        let parse_tree = parser.parse_with(&mut |byte, _| {
            if byte <= rope.len_bytes() {
                let (chunk, start_byte, _, _) = rope.chunk_at_byte(byte);
                &chunk.as_bytes()[byte - start_byte..]
            } else {
                &[]
            }
        }, None).unwrap();

        Ok(Self {
            path: path.as_ref().into(), 
            rope,
            parse_tree
        })
    }
}

// Acquired from tree-sitter-cli
// Will delete later
#[test]
fn test() {
    let s1 = Script::from_file("D:\\Git-repo\\tw3-scripts\\core\\math.ws").unwrap();
    
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    let mut cursor = s1.parse_tree.walk();
    let mut needs_newline = false;
    let mut indent_level = 0;
    let mut did_visit_children = false;
    loop {
        let node = cursor.node();
        let is_named = node.is_named();
        if did_visit_children {
            if is_named {
                stdout.write(b")").unwrap();
                needs_newline = true;
            }
            if cursor.goto_next_sibling() {
                did_visit_children = false;
            } else if cursor.goto_parent() {
                did_visit_children = true;
                indent_level -= 1;
            } else {
                break;
            }
        } else {
            if is_named {
                if needs_newline {
                    stdout.write(b"\n").unwrap();
                }
                for _ in 0..indent_level {
                    stdout.write(b"  ").unwrap();
                }
                let start = node.start_position();
                let end = node.end_position();
                if let Some(field_name) = cursor.field_name() {
                    write!(&mut stdout, "{}: ", field_name).unwrap();
                }
                write!(
                    &mut stdout,
                    "({} [{}, {}] - [{}, {}]",
                    node.kind(),
                    start.row + 1,
                    start.column + 1,
                    end.row + 1,
                    end.column + 1
                ).unwrap();
                needs_newline = true;
            }
            if cursor.goto_first_child() {
                did_visit_children = false;
                indent_level += 1;
            } else {
                did_visit_children = true;
            }
        }
    }
}