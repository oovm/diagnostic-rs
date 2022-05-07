use crate::DokiError;
use git2::Error;

impl From<Error> for DokiError {
    fn from(e: Error) -> Self {
        DokiError::runtime_error(e.to_string())
    }
}
