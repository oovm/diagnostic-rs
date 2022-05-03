use ropey::Error;

use crate::{errors::RuntimeError, QError, QErrorKind};

impl From<Error> for QError {
    fn from(error: Error) -> Self {
        QError {
            error: Box::new(QErrorKind::Runtime(RuntimeError::from(&error))),
            level: Default::default(),
            source: Some(Box::new(error)),
        }
    }
}
