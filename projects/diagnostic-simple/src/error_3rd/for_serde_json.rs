use crate::QError;
use serde_json::Error;

impl From<Error> for QError {
    fn from(error: Error) -> Self {
        Self::syntax_error(error.to_string())
    }
}
