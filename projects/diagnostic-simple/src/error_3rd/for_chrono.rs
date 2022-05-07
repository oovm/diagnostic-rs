use chrono::ParseError;

use crate::{QError, QErrorKind, SyntaxError};

impl From<ParseError> for QError {
    fn from(error: ParseError) -> Self {
        let syntax = SyntaxError::from(&error);
        Self { error: Box::new(QErrorKind::Syntax(syntax)), level: Default::default(), source: Some(Box::new(error)) }
    }
}
