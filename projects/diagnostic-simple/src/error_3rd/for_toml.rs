use crate::{QError, QErrorKind, SyntaxError};

impl From<toml::de::Error> for QError {
    fn from(error: toml::de::Error) -> Self {
        let syntax = SyntaxError { message: error.to_string(), file: Default::default(), span: Default::default() };
        QError { error: Box::new(QErrorKind::Syntax(syntax)), level: Default::default(), source: Some(Box::new(error)) }
    }
}

impl From<toml::ser::Error> for QError {
    fn from(error: toml::ser::Error) -> Self {
        let syntax = SyntaxError { message: error.to_string(), file: Default::default(), span: Default::default() };
        QError { error: Box::new(QErrorKind::Syntax(syntax)), level: Default::default(), source: Some(Box::new(error)) }
    }
}
