use std::{
    fs::File,
    io::{self, BufReader, BufRead},
    path::{PathBuf, Path}
};

use ropey::{Rope, RopeBuilder};
use thiserror::Error;
use tree_sitter::{Parser, Tree, LanguageError};
use encoding_rs_io::DecodeReaderBytes;

use crate::SyntaxNode;

#[derive(Debug, Clone)]
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

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn root_node(&self) -> SyntaxNode<'_, Script> {
        SyntaxNode::new(self.parse_tree.root_node(), self.rope.clone())
    }
}