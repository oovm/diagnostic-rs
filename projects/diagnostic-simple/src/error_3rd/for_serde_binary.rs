use serde_binary::Error;

use crate::{QError, QErrorKind, RuntimeError};

impl From<Error> for QError {
    fn from(error: Error) -> Self {
        QError {
            error: Box::new(QErrorKind::Runtime(RuntimeError::from(&error))),
            level: Default::default(),
            source: Some(Box::new(error)),
        }
    }
}
