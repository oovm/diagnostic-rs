#![allow(clippy::needless_return)]
#![doc = include_str!("../Readme.md")]

pub use diagnostic::{DiagnosticLevel, FileID, Span, TextStorage};

pub use self::errors::{IOError, RuntimeError, SyntaxError, Validation, YggdrasilError, YggdrasilErrorKind, YggdrasilResult};

pub mod error_3rd;
mod errors;
// #[cfg(feature = "lsp-types")]
// mod for_lsp;
