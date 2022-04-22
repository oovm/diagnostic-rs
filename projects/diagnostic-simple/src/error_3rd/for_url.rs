use diagnostic::DiagnosticLevel;
use url::ParseError;

use crate::{errors::SyntaxError, QError};

impl From<ParseError> for QError {
    fn from(e: ParseError) -> Self {
        SyntaxError { message: e.to_string(), file: Default::default(), span: Default::default() }
            .as_error(DiagnosticLevel::Error)
    }
}
