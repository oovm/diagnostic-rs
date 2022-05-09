#![doc = include_str!("../Readme.md")]

pub use self::{
    errors::{
        simple::{ErrorWithFile, ErrorWithFileSpan},
        validation::Validation::{self, Failure, Success},
        DiagnosticError, DiagnosticResult,
    },
    text_cache::{
        file_id::FileID,
        labels::{Label, LabelStyle},
        level::{Diagnostic, DiagnosticLevel},
        location::{column_index, line_starts, Location, Span},
        TextCache, TextStorage,
    },
};

pub mod term;

mod errors;
mod text_cache;
