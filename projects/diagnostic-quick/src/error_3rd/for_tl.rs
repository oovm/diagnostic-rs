use tl::errors::ParseError;

use crate::{QError, QErrorKind, SyntaxError};

impl From<ParseError> for QError {
    fn from(error: ParseError) -> Self {
        let syntax = SyntaxError { message: error.to_string(), file: Default::default(), span: Default::default() };
        Self { error: Box::new(QErrorKind::Syntax(syntax)), level: Default::default(), source: Some(Box::new(error)) }
    }
}
