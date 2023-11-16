use std::{
    fs::File,
    io,
    path::{PathBuf, Path}
};

use ropey::Rope;
use thiserror::Error;
use tree_sitter::{Parser, Tree, LanguageError};
use encoding_rs_io::DecodeReaderBytes;

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

        let rope = Rope::from_reader(decoder).map_err(FileReadError)?;

        let mut parser = Parser::new();
        parser.set_language(tree_sitter_witcherscript::language()).map_err(ParserInitError)?;

        let parse_tree = parser.parse_with(&mut |offset, _| {
            if offset < rope.len_bytes() {
                rope.chunk_at_byte(offset).0
            } else {
                &""
            }
        }, None).unwrap();

        Ok(Self {
            path: path.as_ref().into(), 
            rope,
            parse_tree
        })
    }
}
