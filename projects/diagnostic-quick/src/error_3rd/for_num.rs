use std::error::Error;

use num::bigint::{ParseBigIntError, TryFromBigIntError};

use crate::QError;

impl<T> From<TryFromBigIntError<T>> for QError
where
    T: Error + 'static,
{
    fn from(error: TryFromBigIntError<T>) -> Self {
        QError::fast_syntax_error(error)
    }
}

impl From<ParseBigIntError> for QError {
    fn from(error: ParseBigIntError) -> Self {
        QError::fast_syntax_error(error)
    }
}
