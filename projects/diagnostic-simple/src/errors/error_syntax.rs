use super::*;

impl<E> From<E> for SyntaxError
where
    E: Error,
{
    fn from(error: E) -> Self {
        SyntaxError { message: error.to_string(), file: Default::default(), span: Default::default() }
    }
}
