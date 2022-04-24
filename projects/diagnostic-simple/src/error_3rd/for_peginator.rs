use peginator::ParseError;

use diagnostic::Span;

use crate::{errors::SyntaxError, QError, QErrorKind};

impl From<ParseError> for QError {
    fn from(error: ParseError) -> Self {
        let syntax = SyntaxError {
            message: error.specifics.to_string(),
            file: Default::default(),
            span: Span { start: error.position, end: error.position },
        };
        Self { error: Box::new(QErrorKind::Syntax(syntax)), level: Default::default(), source: Some(Box::new(error)) }
    }
}
