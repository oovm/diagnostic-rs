use diagnostic::Diagnostic;

use super::*;

impl<E> From<E> for RuntimeError
where
    E: Error,
{
    fn from(error: E) -> Self {
        RuntimeError { message: error.to_string() }
    }
}
