use serde_binary::Error;

use crate::QError;

impl From<Error> for QError {
    fn from(error: Error) -> Self {
        Self::runtime_error(error.to_string())
    }
}
