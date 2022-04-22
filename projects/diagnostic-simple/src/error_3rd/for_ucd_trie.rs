use diagnostic::DiagnosticLevel;
use ucd_trie::Error;

use crate::{errors::RuntimeError, QError};

impl From<Error> for QError {
    fn from(e: Error) -> Self {
        RuntimeError { message: e.to_string() }.as_error(DiagnosticLevel::Error)
    }
}
