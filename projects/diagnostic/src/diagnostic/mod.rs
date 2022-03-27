//! Diagnostic data structures.


use std::{ops::Range, string::ToString};

use serde::{Deserialize, Serialize};

/// A severity level for diagnostic messages.
///
/// These are ordered in the following way:
///
/// ```rust
/// use codespan_reporting::diagnostic::Severity;
///
/// assert!(Severity::Bug > Severity::Error);
/// assert!(Severity::Error > Severity::Warning);
/// assert!(Severity::Warning > Severity::Note);
/// assert!(Severity::Note > Severity::Help);
/// ```
#[derive(Copy, Clone, Hash, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum DiagnosticLevel {
    /// A help message.
    Help,
    /// A note.
    Note,
    /// A warning.
    Warning,
    /// An error.
    Error,
    /// An unexpected bug.
    Bug,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Serialize, Deserialize)]
pub enum LabelStyle {
    /// Labels that describe the primary cause of a diagnostic.
    Primary,
    /// Labels that provide additional context for a diagnostic.
    Secondary,
}

/// A label describing an underlined region of code associated with a diagnostic.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Label {
    /// The file that we are labelling.
    pub file_id: String,
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
    pub fn primary(file_id: impl Into<String>, range: Range<usize>) -> Self {
        Self {
            file_id: file_id.into(),
            style: LabelStyle::Primary,
            range,
            message: String::new(),
        }
    }
    /// Create a new label with a style of [`LabelStyle::Secondary`].
    ///
    /// [`LabelStyle::Secondary`]: LabelStyle::Secondary
    pub fn secondary(file_id: impl Into<String>, range: Range<usize>) -> Self {
        Self {
            file_id: file_id.into(),
            style: LabelStyle::Secondary,
            range,
            message: String::new(),
        }
    }

    /// Add a message to the diagnostic.
    pub fn with_message(mut self, message: impl ToString) -> Label {
        self.message = message.to_string();
        self
    }
}

/// Represents a diagnostic message that can provide information like errors and
/// warnings to the user.
///
/// The position of a Diagnostic is considered to be the position of the [`Label`] that has the earliest starting position and has the highest style which appears in all the labels of the diagnostic.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Diagnostic {
    /// The overall severity of the diagnostic
    pub severity: DiagnosticLevel,
    /// An optional code that identifies this diagnostic.
    pub code: Option<String>,
    /// The main message associated with this diagnostic.
    ///
    /// These should not include line breaks, and in order support the 'short'
    /// diagnostic display mod, the message should be specific enough to make
    /// sense on its own, without additional context provided by labels and notes.
    pub message: String,
    /// Source labels that describe the cause of the diagnostic.
    /// The order of the labels inside the vector does not have any meaning.
    /// The labels are always arranged in the order they appear in the source code.
    pub labels: Vec<Label>,
    /// Notes that are associated with the primary cause of the diagnostic.
    /// These can include line breaks for improved formatting.
    pub notes: Vec<String>,
}

impl Diagnostic {
    /// Create a new diagnostic.
    pub fn new(severity: DiagnosticLevel) -> Diagnostic {
        Diagnostic { severity, code: None, message: String::new(), labels: Vec::new(), notes: Vec::new() }
    }

    /// Set the error code of the diagnostic.
    pub fn with_code(mut self, code: impl ToString) -> Diagnostic {
        self.code = Some(code.to_string());
        self
    }

    /// Set the message of the diagnostic.
    pub fn with_message(mut self, message: impl ToString) -> Diagnostic {
        self.message = message.to_string();
        self
    }

    /// Add some labels to the diagnostic.
    pub fn with_labels(mut self, mut labels: Vec<Label>) -> Diagnostic {
        self.labels.append(&mut labels);
        self
    }

    /// Add some notes to the diagnostic.
    pub fn with_notes(mut self, mut notes: Vec<String>) -> Diagnostic {
        self.notes.append(&mut notes);
        self
    }
}
