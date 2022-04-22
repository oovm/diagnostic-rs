use crate::{QError, RuntimeError};
use diagnostic::DiagnosticLevel;
use serde_binary::Error;

impl From<Error> for QError {
    fn from(error: Error) -> Self {
        RuntimeError { message: error.to_string() }.as_error(DiagnosticLevel::Error)
    }
}
