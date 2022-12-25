use sled::{CompareAndSwapError, Error};

use crate::{QError, QErrorKind, RuntimeError};

impl From<Error> for QError {
    fn from(error: Error) -> Self {
        match error {
            Error::Io(o) => QError::from(o),
            _ => QError {
                error: Box::new(QErrorKind::Runtime(RuntimeError::from(&error))),
                level: Default::default(),
                source: Some(Box::new(error)),
            },
        }
    }
}

impl From<CompareAndSwapError> for QError {
    fn from(error: CompareAndSwapError) -> Self {
        QError {
            error: Box::new(QErrorKind::Runtime(RuntimeError::from(&error))),
            level: Default::default(),
            source: Some(Box::new(error)),
        }
    }
}
