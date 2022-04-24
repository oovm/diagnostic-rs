use std::{
    num::{ParseFloatError, ParseIntError},
    str::Utf8Error,
};

use super::*;

impl<E> From<&E> for SyntaxError
where
    E: Error,
{
    fn from(error: &E) -> Self {
        SyntaxError { message: error.to_string(), file: Default::default(), span: Default::default() }
    }
}

impl QError {
    fn wrap_syntax<E: Error + 'static>(error: E) -> Self {
        QError {
            error: Box::new(QErrorKind::Syntax(SyntaxError::from(&error))),
            level: DiagnosticLevel::Error,
            source: Some(Box::new(error)),
        }
    }
}

impl From<Utf8Error> for QError {
    fn from(error: Utf8Error) -> Self {
        QError::wrap_syntax(error)
    }
}

impl From<ParseIntError> for QError {
    fn from(error: ParseIntError) -> Self {
        QError::wrap_syntax(error)
    }
}

impl From<ParseFloatError> for QError {
    fn from(error: ParseFloatError) -> Self {
        QError::wrap_syntax(error)
    }
}
