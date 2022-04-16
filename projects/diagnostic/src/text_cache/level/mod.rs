use super::*;
use crate::Span;

/// A severity level for labels messages.
///
/// These are ordered in the following way:
///
/// ```rust
/// use diagnostic::DiagnosticLevel;
///
/// assert!(DiagnosticLevel::Fatal > DiagnosticLevel::Error);
/// assert!(DiagnosticLevel::Error > DiagnosticLevel::Warning);
/// assert!(DiagnosticLevel::Warning > DiagnosticLevel::Info);
/// assert!(DiagnosticLevel::Info > DiagnosticLevel::Custom);
/// ```
#[derive(Copy, Clone, Hash, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum DiagnosticLevel {
    /// A custom diagnostic level
    Custom(&'static str),
    /// A useful information.
    Info,
    /// A warning that problems may arise
    Warning,
    /// An error.
    Error,
    /// An unexpected bug.
    Fatal,
}

/// Represents a labels message that can provide information like errors and
/// warnings to the user.
///
/// The position of a Diagnostic is considered to be the position of the [`Label`] that has the earliest starting position and has the highest style which appears in all the labels of the labels.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Diagnostic {
    /// The overall severity of the labels
    pub severity: DiagnosticLevel,
    /// An optional code that identifies this labels.
    pub code: Option<String>,
    /// The main message associated with this labels.
    ///
    /// These should not include line breaks, and in order support the 'short'
    /// labels display mod, the message should be specific enough to make
    /// sense on its own, without additional context provided by labels and notes.
    pub message: String,
    /// Source labels that describe the cause of the labels.
    /// The order of the labels inside the vector does not have any meaning.
    /// The labels are always arranged in the order they appear in the source code.
    pub labels: Vec<Label>,
    /// Notes that are associated with the primary cause of the labels.
    /// These can include line breaks for improved formatting.
    pub notes: Vec<String>,
}

impl Diagnostic {
    /// Create a new label.
    pub fn new(severity: DiagnosticLevel) -> Self {
        Diagnostic { severity, code: None, message: String::new(), labels: Vec::new(), notes: Vec::new() }
    }
    /// Create a new info label.
    pub fn info() -> Self {
        Diagnostic::new(DiagnosticLevel::Info)
    }
    /// Create a new warning label.
    pub fn warning() -> Self {
        Diagnostic::new(DiagnosticLevel::Warning)
    }
    /// Create a new error label.
    pub fn error() -> Self {
        Diagnostic::new(DiagnosticLevel::Error)
    }
    /// Create a new fatal label.
    pub fn fatal() -> Self {
        Diagnostic::new(DiagnosticLevel::Fatal)
    }

    /// Set the error code of the labels.
    pub fn with_code(mut self, code: impl Display) -> Self {
        self.code = Some(code.to_string());
        self
    }

    /// Set the message of the labels.
    pub fn with_message(mut self, message: impl Display) -> Self {
        self.message = message.to_string();
        self
    }

    /// Add some labels to the labels.
    pub fn with_primary(mut self, file_id: &FileID, range: Span, message: impl Display) -> Self {
        let label = Label::primary(file_id, range, message);
        self.labels.push(label);
        self
    }

    /// Add some labels to the labels.
    pub fn with_secondary(mut self, file_id: &FileID, range: Span, message: impl Display) -> Self {
        let label = Label::secondary(file_id, range, message);
        self.labels.push(label);
        self
    }

    /// Add some labels to the labels.
    pub fn with_labels(mut self, mut labels: Vec<Label>) -> Self {
        self.labels.append(&mut labels);
        self
    }

    /// Add some notes to the labels.
    pub fn with_note(mut self, note: impl Display) -> Self {
        self.notes.push(note.to_string());
        self
    }

    /// Add some notes to the labels.
    pub fn with_notes(mut self, mut notes: Vec<String>) -> Self {
        self.notes.append(&mut notes);
        self
    }
}
