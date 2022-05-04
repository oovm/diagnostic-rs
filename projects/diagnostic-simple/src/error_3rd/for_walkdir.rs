use crate::QError;
use walkdir::Error;

impl From<Error> for QError {
    fn from(value: Error) -> Self {
        todo!()
    }
}
