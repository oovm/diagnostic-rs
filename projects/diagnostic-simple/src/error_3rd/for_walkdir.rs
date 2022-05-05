use walkdir::Error;

use diagnostic::FileID;

use crate::{IOError, QError, QErrorKind, RuntimeError};

impl From<Error> for QError {
    fn from(value: Error) -> Self {
        match value.io_error() {
            Some(s) => {
                let file = match value.path() {
                    Some(s) => match FileID::try_from(s) {
                        Ok(o) => o,
                        Err(_) => Default::default(),
                    },
                    None => Default::default(),
                };
                let io = IOError { message: s.to_string(), file };
                return QError {
                    error: Box::new(QErrorKind::IO(io)),
                    level: Default::default(),
                    source: Some(Box::new(value)),
                };
            }
            None => {}
        }
        let runtime = RuntimeError { message: value.to_string() };
        QError { error: Box::new(QErrorKind::Runtime(runtime)), level: Default::default(), source: Some(Box::new(value)) }
    }
}
