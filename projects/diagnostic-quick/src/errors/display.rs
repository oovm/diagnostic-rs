use diagnostic::{Diagnostic, DiagnosticError};

use super::*;

pub fn print_errors(store: &TextStorage, errors: &[QError]) -> QResult {
    let writer = StandardStream::stderr(ColorChoice::Always);
    let config = TerminalConfig::default();
    for error in errors {
        let diagnostic = error.as_diagnostic();
        emit(&mut writer.lock(), &config, &store, &diagnostic)?;
    }
    Ok(())
}

impl From<DiagnosticError> for QError {
    fn from(error: DiagnosticError) -> Self {
        QError::wrap_runtime_error(error)
    }
}

impl QError {
    pub fn as_diagnostic(&self) -> Diagnostic {
        let mut out = Diagnostic::new(self.level);
        match &*self.error {
            QErrorKind::IO(e) => out.message = e.message.to_string(),
            QErrorKind::Syntax(e) => out = out.with_primary(&e.file, e.span.clone(), &e.message),
            QErrorKind::Runtime(e) => out.message = e.message.to_string(),
            QErrorKind::Custom(e) => {
                out.message = e.to_string();
            }
        }
        out
    }
}
