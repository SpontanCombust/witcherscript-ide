/// Modules providing implementations for [`Backend`]'s LanguageServer methods.

pub mod initialization;
pub mod document_ops;
pub mod configuration;
pub mod workspace;

pub mod selection_range;
pub mod document_symbols;
pub mod goto;
pub mod hover;

pub mod custom;

mod common;