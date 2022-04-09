//! Diagnostic data structures.

use std::{fmt::Display, ops::Range};

use crate::FileID;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Serialize, Deserialize)]
pub enum LabelStyle {
    /// Labels that describe the primary cause of a labels.
    Primary,
    /// Labels that provide additional context for a labels.
    Secondary,
}

/// A label describing an underlined region of code associated with a labels.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Label {
    /// The file that we are labelling.
    pub file_id: FileID,
    /// The style of the label.
    pub style: LabelStyle,
    /// The range in bytes we are going to include in the final snippet.
    pub range: Range<usize>,
    /// An optional message to provide some additional information for the
    /// underlined code. These should not include line breaks.
    pub message: String,
}

impl Label {
    /// Create a new label with a style of [`LabelStyle::Primary`].
    ///
    /// [`LabelStyle::Primary`]: LabelStyle::Primary
    pub fn primary(file_id: impl Into<FileID>, range: Range<usize>, message: impl Display) -> Self {
        Self { file_id: file_id.into(), style: LabelStyle::Primary, range, message: message.to_string() }
    }
    /// Create a new label with a style of [`LabelStyle::Secondary`].
    ///
    /// [`LabelStyle::Secondary`]: LabelStyle::Secondary
    pub fn secondary(file_id: impl Into<FileID>, range: Range<usize>, message: impl Display) -> Self {
        Self { file_id: file_id.into(), style: LabelStyle::Secondary, range, message: message.to_string() }
    }
}
