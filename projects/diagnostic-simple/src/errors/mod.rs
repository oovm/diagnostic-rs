use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
    ops::Range,
};

use diagnostic::{DiagnosticLevel, FileID, Span};

pub mod error_io;
pub mod error_runtime;
pub mod error_syntax;

pub type QResult<T = ()> = Result<T, QError>;

pub type Validation<T> = diagnostic::Validation<T, QError>;

#[derive(Debug)]
pub struct QError {
    pub error: Box<QErrorKind>,
    pub level: DiagnosticLevel,
    pub source: Option<Box<dyn Error>>,
}

#[derive(Debug)]
pub enum QErrorKind {
    IO(IOError),
    Syntax(SyntaxError),
    Runtime(RuntimeError),
    Custom(String),
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
    pub fn syntax_error(msg: impl Into<String>) -> Self {
        let error = SyntaxError { message: msg.into(), file: Default::default(), span: Default::default() };
        Self { error: Box::new(QErrorKind::Syntax(error)), level: Default::default(), source: None }
    }
    pub fn runtime_error(msg: impl Into<String>) -> Self {
        let error = RuntimeError { message: msg.into() };
        Self { error: Box::new(QErrorKind::Runtime(error)), level: Default::default(), source: None }
    }
    pub fn kind(&self) -> &QErrorKind {
        &*self.error
    }
    pub fn with_file(mut self, file: &FileID) -> Self {
        match &mut *self.error {
            QErrorKind::IO(v) => {
                v.file = file.clone();
            }
            QErrorKind::Syntax(v) => {
                v.file = file.clone();
            }
            QErrorKind::Runtime(_) => {}
            QErrorKind::Custom(_) => {}
        }
        self
    }
    pub fn with_range(mut self, range: &Range<usize>) -> Self {
        match &mut *self.error {
            QErrorKind::IO(_) => {}
            QErrorKind::Syntax(v) => v.span = range.clone(),
            QErrorKind::Runtime(_) => {}
            QErrorKind::Custom(_) => {}
        }
        self
    }
}

impl Display for QError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Display for QErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for QError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self.source {
            Some(s) => Some(&**s),
            None => None,
        }
    }
}
