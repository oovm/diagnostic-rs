use globset::Error;

use crate::{QError, QErrorKind, SyntaxError};

impl From<Error> for QError {
    fn from(error: Error) -> Self {
        let syntax = SyntaxError { message: error.to_string(), file: Default::default(), span: Default::default() };
        Self { error: Box::new(QErrorKind::Syntax(syntax)), level: Default::default(), source: Some(Box::new(error)) }
    }
}
