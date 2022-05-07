use crate::QError;
use num::{
    bigint::{ParseBigIntError, TryFromBigIntError},
    BigInt,
};

impl<T> From<TryFromBigIntError<T>> for QError {
    fn from(error: TryFromBigIntError<T>) -> Self {
        todo!()
    }
}

impl From<ParseBigIntError> for QError {
    fn from(error: ParseBigIntError) -> Self {
        todo!()
    }
}
