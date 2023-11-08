This module (confusingly, I know) does not actually contain a lexer. For now lexing together with parsing is done entirely inside `parser.rs`.
It does however contain types that belong to the tokenization stage.

rust-peg parser with `str` input type returns a collection of expected string tokens upon error. These raw strings can be reparsed into an enum representation of tokens and be handled much more gracefully by LSP.