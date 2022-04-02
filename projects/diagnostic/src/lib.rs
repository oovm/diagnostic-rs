#![doc = include_str ! ("../Readme.md")]

pub use self::{
    errors::{DiagnosticError, DiagnosticResult, Location},
    text_cache::{
        builder::{Diagnostic, DiagnosticLevel},
        labels::{Label, LabelStyle},
        TextCache, TextStorage,
    },
};

pub mod term;

mod errors;
mod text_cache;
