use url::ParseError;

use crate::{errors::SyntaxError, QError, QErrorKind};

impl From<ParseError> for QError {
    fn from(error: ParseError) -> Self {
        QError {
            error: Box::new(QErrorKind::Syntax(SyntaxError::from(&error))),
            level: Default::default(),
            source: Some(Box::new(error)),
        }
    }
}
