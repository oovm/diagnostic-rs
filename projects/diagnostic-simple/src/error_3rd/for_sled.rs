use diagnostic::DiagnosticLevel;
use sled::{CompareAndSwapError, Error};

use crate::{IOError, QError, RuntimeError};

impl From<Error> for QError {
    fn from(error: Error) -> Self {
        match error {
            Error::Io(o) => IOError::from(o).as_error(DiagnosticLevel::Error),
            _ => RuntimeError { message: error.to_string() }.as_error(DiagnosticLevel::Error),
        }
    }
}

impl From<CompareAndSwapError> for QError {
    fn from(error: CompareAndSwapError) -> Self {
        RuntimeError { message: error.to_string() }.as_error(DiagnosticLevel::Error)
    }
}
