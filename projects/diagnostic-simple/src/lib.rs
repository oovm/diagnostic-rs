#![allow(clippy::needless_return)]
#![doc = include_str!("../Readme.md")]

pub use diagnostic::{DiagnosticLevel, FileID, Span, TextStorage};

pub use self::errors::{IOError, QError, QResult, RuntimeError, SyntaxError, Validation, YggdrasilErrorKind};

pub mod error_3rd;
mod errors;
// #[cfg(feature = "lsp-types")]
// mod for_lsp;
