use crate::{IOError, QError, QErrorKind};

impl From<std::io::Error> for QError {
    fn from(error: std::io::Error) -> Self {
        let io = IOError { message: error.to_string(), file: Default::default() };
        QError { error: Box::new(QErrorKind::IO(io)), level: Default::default(), source: Some(Box::new(error)) }
    }
}
