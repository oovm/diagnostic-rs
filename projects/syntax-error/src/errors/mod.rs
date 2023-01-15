use diagnostic::{Diagnostic, DiagnosticLevel, FileSpan, Label, SourceID};
use std::{
    error::Error,
    fmt::{Display, Formatter},
    ops::Range,
};

#[derive(Clone, Debug)]
pub struct SyntaxError {
    pub info: String,
    pub span: FileSpan<u32>,
    pub level: DiagnosticLevel,
}

impl Error for SyntaxError {}

impl Display for SyntaxError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str(&self.info)
    }
}

impl SyntaxError {
    pub fn new(file: SourceID) -> Self {
        Self { span: file.with_range(0..0), info: String::new(), level: DiagnosticLevel::Error }
    }
    pub fn with_message<T>(mut self, info: T) -> Self
    where
        T: ToString,
    {
        self.info = info.to_string();
        self
    }
    pub fn with_range(mut self, range: Range<u32>) -> Self {
        self.span.set_range(range);
        self
    }
    pub fn with_span(mut self, span: FileSpan<u32>) -> Self {
        self.span = span;
        self
    }
    pub fn with_level(mut self, level: DiagnosticLevel) -> Self {
        self.level = level;
        self
    }
    pub fn as_report(&self) -> Diagnostic {
        // let mut report = Diagnostic::new(self.level, self.span.get_file(), self.span.get_range().start);
        // report.set_message(self.to_string());
        // let label = Label::new(self.span);
        // report.add_label(label);
        // report.finish()
        todo!()
    }
}
