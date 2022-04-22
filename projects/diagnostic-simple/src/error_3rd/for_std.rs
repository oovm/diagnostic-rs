use std::{
    env::VarError,
    num::{ParseFloatError, ParseIntError},
    str::Utf8Error,
    sync::PoisonError,
};

use diagnostic::DiagnosticLevel;

use crate::{
    errors::{IOError, RuntimeError, SyntaxError},
    QError,
};

impl From<Utf8Error> for QError {
    fn from(error: Utf8Error) -> Self {
        SyntaxError::from(error).as_error(DiagnosticLevel::Error)
    }
}

impl From<ParseIntError> for QError {
    fn from(error: ParseIntError) -> Self {
        SyntaxError::from(error).as_error(DiagnosticLevel::Error)
    }
}

impl From<ParseFloatError> for QError {
    fn from(error: ParseFloatError) -> Self {
        SyntaxError::from(error).as_error(DiagnosticLevel::Error)
    }
}

impl From<std::io::Error> for QError {
    fn from(error: std::io::Error) -> Self {
        IOError::from(error).as_error(DiagnosticLevel::Error)
    }
}

impl From<std::fmt::Error> for QError {
    fn from(error: std::fmt::Error) -> Self {
        RuntimeError::from(error).as_error(DiagnosticLevel::Error)
    }
}

impl<T> From<PoisonError<T>> for QError {
    fn from(error: PoisonError<T>) -> Self {
        RuntimeError::from(error).as_error(DiagnosticLevel::Error)
    }
}

impl From<VarError> for QError {
    fn from(error: VarError) -> Self {
        RuntimeError::from(error).as_error(DiagnosticLevel::Error)
    }
}

impl From<()> for QError {
    fn from(_: ()) -> Self {
        Self::unreachable()
    }
}
