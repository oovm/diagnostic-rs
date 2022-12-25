use crate::{DiagnosticLevel, DokiError, DokiErrorKind};
use html_parser::Error;

impl From<Error> for DokiError {
    fn from(e: Error) -> Self {
        let kind = match e {
            Error::Parsing(e) => DokiErrorKind::SyntaxError(e),
            Error::IO(e) => DokiErrorKind::IOError(e),
            Error::Cli(_) => {
                unimplemented!()
            }
            Error::Serde(_) => {
                unimplemented!()
            }
        };
        Self { kind: Box::new(kind), level: DiagnosticLevel::None, file: None, range: None }
    }
}
