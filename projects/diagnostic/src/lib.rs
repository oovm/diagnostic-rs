#![doc = include_str!("../Readme.md")]

pub use self::{
    errors::{validation::Validation, DiagnosticError, DiagnosticResult},
    text_cache::{
        file_id::FileID,
        labels::{Label, LabelStyle},
        level::{Diagnostic, DiagnosticLevel},
        location::{Location, Span},
        TextCache, TextStorage,
    },
};

pub mod term;

mod errors;
mod text_cache;
