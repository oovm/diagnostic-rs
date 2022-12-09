use crate::QError;
use semver::Error;

impl From<Error> for QError {
    fn from(error: Error) -> Self {
        QError::fast_syntax_error(error)
    }
}
