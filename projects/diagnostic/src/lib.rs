//! Diagnostic reporting support for the codespan crate.

#![forbid(unsafe_code)]

pub use errors::{DiagnosticError, DiagnosticResult};

pub mod diagnostic;
pub mod errors;
pub mod term;

