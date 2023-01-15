use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
    ops::Range,
};

use diagnostic::{
    term::{
        emit,
        termcolor::{ColorChoice, StandardStream},
        TerminalConfig,
    },
    DiagnosticLevel, FileSpan, SourceID, Span, TextStorage,
};

pub mod display;
mod error_io;
mod error_runtime;
mod error_syntax;

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
    pub span: FileSpan,
}

/// An error that occurs during runtime.
///
/// # Arguments
///
/// * `msg`:
///
/// returns: QError
///
/// # Examples
///
/// ```
/// use diagnostic_quick::RuntimeError;
/// RuntimeError::from("runtime error");
/// ```
#[derive(Debug)]
pub struct RuntimeError {
    pub message: String,
}

#[derive(Debug)]
pub struct IOError {
    pub message: String,
    pub file: SourceID,
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
    pub fn with_file(mut self, file: &SourceID) -> Self {
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
    pub fn with_level(mut self, level: impl Into<DiagnosticLevel>) -> Self {
        self.level = level.into();
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
