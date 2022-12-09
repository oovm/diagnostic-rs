use email_address::Error;

use crate::QError;

impl From<Error> for QError {
    fn from(error: Error) -> Self {
        QError::fast_syntax_error(error)
    }
}
