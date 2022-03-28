//! Diagnostic reporting support for the codespan crate.

#![forbid(unsafe_code)]

pub use self::errors::{DiagnosticError, DiagnosticResult};
pub mod term;

mod errors;
mod text_cache;

pub use self::text_cache::{TextStorage, TextCache, labels::{Diagnostic, DiagnosticLevel, Label, LabelStyle}};