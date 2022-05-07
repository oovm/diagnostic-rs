use crate::DokiError;
use chrono::ParseError;

impl From<ParseError> for DokiError {
    fn from(e: ParseError) -> Self {
        DokiError::syntax_error(e.to_string())
    }
}
