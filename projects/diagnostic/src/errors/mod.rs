//! Source file support for labels reporting.
//!
//! The main trait defined in this module is the [`Files`] trait, which provides
//! provides the minimum amount of functionality required for printing [`Diagnostics`]
//! with the [`term::emit`] function.
//!
//! Simple implementations of this trait are implemented:
//!
//! - [`SimpleFile`]: For single-file use-cases
//! - [`SimpleFiles`]: For multi-file use-cases
//!
//! These data structures provide a pretty minimal API, however,
//! so end-users are encouraged to create their own implementations for their
//! own specific use-cases, such as an implementation that accesses the file
//! system directly (and caches the line start locations), or an implementation
//! using an incremental compilation library like [`salsa`].
//!
//! [`term::emit`]: crate::term::emit
//! [`Diagnostics`]: crate::labels::Diagnostic
//! [`Files`]: Files
//! [`SimpleFile`]: SimpleFile
//! [`SimpleFiles`]: SimpleFiles
//!
//! [`salsa`]: https://crates.io/crates/salsa

pub mod simple;
pub mod validation;

use std::{error::Error, fmt::Display};

pub type DiagnosticResult<T = ()> = Result<T, DiagnosticError>;

/// An enum representing an error that happened while looking up a file or a piece of content in that file.
#[derive(Debug)]
#[non_exhaustive]
pub enum DiagnosticError {
    /// A required file is not in the file database.
    FileMissing,
    /// The file is present, but does not contain the specified byte index.
    IndexTooLarge { given: usize, max: usize },
    /// The file is present, but does not contain the specified line index.
    LineTooLarge { given: usize, max: usize },
    /// The file is present and contains the specified line index, but the line does not contain the specified column index.
    ColumnTooLarge { given: usize, max: usize },
    /// The given index is contained in the file, but is not a boundary of a UTF-8 code point.
    InvalidCharBoundary { given: usize },
    /// There was a error while doing IO.
    IOError(std::io::Error),
}

impl From<std::io::Error> for DiagnosticError {
    fn from(err: std::io::Error) -> DiagnosticError {
        DiagnosticError::IOError(err)
    }
}

impl Display for DiagnosticError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DiagnosticError::FileMissing => write!(f, "File ID not found"),
            DiagnosticError::IndexTooLarge { given, max } => {
                write!(f, "invalid index {}, maximum index is {}", given, max)
            }
            DiagnosticError::LineTooLarge { given, max } => {
                write!(f, "invalid line {}, maximum line is {}", given, max)
            }
            DiagnosticError::ColumnTooLarge { given, max } => {
                write!(f, "invalid column {}, maximum column {}", given, max)
            }
            DiagnosticError::InvalidCharBoundary { .. } => write!(f, "index is not a code point boundary"),
            DiagnosticError::IOError(err) => write!(f, "{}", err),
        }
    }
}

impl Error for DiagnosticError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self {
            DiagnosticError::IOError(err) => Some(err),
            _ => None,
        }
    }
}
