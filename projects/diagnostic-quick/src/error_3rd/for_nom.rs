use crate::DokiError;
use nom::{error::Error, Err};

impl From<Err<Error<&str>>> for DokiError {
    fn from(e: Err<Error<&str>>) -> Self {
        DokiError::syntax_error(e.to_string())
    }
}
