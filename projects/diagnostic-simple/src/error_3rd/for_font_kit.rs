use font_kit::error::{FontLoadingError, GlyphLoadingError, SelectionError};

use crate::QError;

impl From<SelectionError> for QError {
    fn from(error: SelectionError) -> Self {
        QError::fast_runtime_error(error)
    }
}

impl From<FontLoadingError> for QError {
    fn from(error: FontLoadingError) -> Self {
        QError::fast_runtime_error(error)
    }
}

impl From<GlyphLoadingError> for QError {
    fn from(error: GlyphLoadingError) -> Self {
        QError::fast_runtime_error(error)
    }
}
