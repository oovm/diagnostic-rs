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

impl From<std::fmt::Error> for QError {
    fn from(error: std::fmt::Error) -> Self {
        // RuntimeError::from(error).as_error(DiagnosticLevel::Error)
        todo!()
    }
}

impl<T> From<PoisonError<T>> for QError {
    fn from(error: PoisonError<T>) -> Self {
        // RuntimeError::from(error).as_error(DiagnosticLevel::Error)
        todo!()
    }
}

impl From<VarError> for QError {
    fn from(error: VarError) -> Self {
        // RuntimeError::from(error).as_error(DiagnosticLevel::Error)
        todo!()
    }
}

impl From<()> for QError {
    fn from(_: ()) -> Self {
        // Self::unreachable()
        todo!()
    }
}
