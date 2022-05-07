use image::ImageError;

use crate::QError;

impl From<ImageError> for QError {
    fn from(error: ImageError) -> Self {
        match error {
            ImageError::IoError(e) => QError::fast_io_error(e),
            _ => QError::fast_runtime_error(error),
        }
    }
}
