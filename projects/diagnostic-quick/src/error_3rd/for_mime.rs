use mime::FromStrError;

use crate::QError;

impl From<FromStrError> for QError {
    fn from(error: FromStrError) -> Self {
        QError::fast_syntax_error(error)
    }
}
