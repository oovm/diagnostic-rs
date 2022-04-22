use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
};

use diagnostic::{DiagnosticLevel, FileID, Span};

pub mod error_io;
pub mod error_runtime;
pub mod error_syntax;

pub type QResult<T = ()> = Result<T, QError>;

pub type Validation<T> = diagnostic::Validation<T, QError>;

#[derive(Debug)]
pub struct QError {
    pub error: Box<YggdrasilErrorKind>,
    pub level: DiagnosticLevel,
    pub source: Option<Box<dyn Error>>,
}

#[derive(Debug)]
pub enum YggdrasilErrorKind {
    ErrorIO(IOError),
    ErrorSyntax(SyntaxError),
    ErrorRuntime(RuntimeError),
    Unreachable,
}

#[derive(Debug)]
pub struct SyntaxError {
    pub message: String,
    pub file: FileID,
    pub span: Span,
}

#[derive(Debug)]
pub struct RuntimeError {
    pub message: String,
}

#[derive(Debug)]
pub struct IOError {
    pub message: String,
    pub file: FileID,
}

impl QError {
    pub fn unreachable() -> Self {
        Self { error: Box::new(YggdrasilErrorKind::Unreachable), level: Default::default() }
    }
    pub fn syntax_error(msg: impl Into<String>) -> Self {
        SyntaxError { message: msg.into(), file: Default::default(), span: Default::default() }.as_error(DiagnosticLevel::Error)
    }
    pub fn runtime_error(msg: impl Into<String>) -> Self {
        RuntimeError { message: msg.into() }.as_error(DiagnosticLevel::Error)
    }
}

impl Error for QError {}

impl Display for QError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Display for YggdrasilErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}
