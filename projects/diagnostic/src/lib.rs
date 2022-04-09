#![doc = include_str ! ("../Readme.md")]

pub use self::{
    errors::{validation::Validation, DiagnosticError, DiagnosticResult},
    text_cache::{
        builder::{Diagnostic, DiagnosticLevel},
        file_id::FileID,
        labels::{Label, LabelStyle},
        location::{Location, Span},
        TextCache, TextStorage,
    },
};

pub mod term;

mod errors;
mod text_cache;
