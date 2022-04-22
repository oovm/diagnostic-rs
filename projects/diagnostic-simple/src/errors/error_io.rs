use std::io::Error;

use super::*;

impl From<Error> for QError {
    fn from(error: Error) -> Self {
        let io = IOError { message: error.to_string(), file: Default::default() };
        QError { error: Box::new(QErrorKind::IO(io)), level: Default::default(), source: Some(Box::new(error)) }
    }
}
