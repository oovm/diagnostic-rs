//! Diagnostic reporting support for the codespan crate.

#![forbid(unsafe_code)]

pub use self::{
    errors::{DiagnosticError, DiagnosticResult},
    text_cache::{
        builder::{Diagnostic, DiagnosticLevel},
        labels::{Label, LabelStyle},
        TextCache, TextStorage,
    },
};

pub mod term;

mod errors;
mod text_cache;
