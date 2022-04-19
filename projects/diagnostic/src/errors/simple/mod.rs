use std::fmt::Display;

use crate::{Diagnostic, DiagnosticLevel, FileID, Span};

#[derive(Debug, Clone)]
pub struct ErrorWithFile<E> {
    pub error: E,
    pub file: FileID,
}

#[derive(Debug, Clone)]
pub struct ErrorWithFileSpan<T> {
    pub error: T,
    pub file: FileID,
    pub span: Span,
}

impl<E> ErrorWithFile<E> {
    pub fn new<T>(error: T) -> Self
    where
        T: Into<E>,
    {
        Self { error: error.into(), file: Default::default() }
    }
    pub fn set_file<T>(&mut self, file: T)
    where
        T: Into<FileID>,
    {
        self.file = file.into()
    }
    pub fn with_file<T>(mut self, file: T) -> Self
    where
        T: Into<FileID>,
    {
        self.set_file(file);
        self
    }
    pub fn as_report<T>(&self, level: DiagnosticLevel, message: T) -> Diagnostic
    where
        T: Into<String>,
        E: Display,
    {
        Diagnostic::new(level) //
            .with_message(message.into())
            .with_primary(&self.file, 0..0, self.error.to_string())
    }
}

impl<E> ErrorWithFileSpan<E> {
    pub fn new<T>(error: T) -> Self
    where
        T: Into<E>,
    {
        Self { error: error.into(), file: Default::default(), span: Default::default() }
    }
    pub fn set_file<T>(&mut self, file: T)
    where
        T: Into<FileID>,
    {
        self.file = file.into()
    }
    pub fn with_file<T>(mut self, file: T) -> Self
    where
        T: Into<FileID>,
    {
        self.set_file(file);
        self
    }
    pub fn set_span(&mut self, span: Span) {
        self.span = span
    }
    pub fn with_span(mut self, span: Span) -> Self {
        self.set_span(span);
        self
    }
    pub fn as_report<T>(&self, level: DiagnosticLevel, message: T) -> Diagnostic
    where
        T: Into<String>,
        E: Display,
    {
        Diagnostic::new(level) //
            .with_message(message.into())
            .with_primary(&self.file, self.span.clone(), self.error.to_string())
    }
}
