use chrono::ParseError;

use crate::QError;

impl From<ParseError> for QError {
    fn from(error: ParseError) -> Self {
        QError::fast_syntax_error(error)
    }
}
