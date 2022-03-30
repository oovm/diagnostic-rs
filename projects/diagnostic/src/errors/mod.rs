//! Source file support for labels reporting.
//!
//! The main trait defined in this module is the [`Files`] trait, which provides
//! provides the minimum amount of functionality required for printing [`Diagnostics`]
//! with the [`term::emit`] function.
//!
//! Simple implementations of this trait are implemented:
//!
//! - [`SimpleFile`]: For single-file use-cases
//! - [`SimpleFiles`]: For multi-file use-cases
//!
//! These data structures provide a pretty minimal API, however,
//! so end-users are encouraged to create their own implementations for their
//! own specific use-cases, such as an implementation that accesses the file
//! system directly (and caches the line start locations), or an implementation
//! using an incremental compilation library like [`salsa`].
//!
//! [`term::emit`]: crate::term::emit
//! [`Diagnostics`]: crate::labels::Diagnostic
//! [`Files`]: Files
//! [`SimpleFile`]: SimpleFile
//! [`SimpleFiles`]: SimpleFiles
//!
//! [`salsa`]: https://crates.io/crates/salsa

use std::{error::Error, fmt::Display, ops::Range};

pub type DiagnosticResult<T = ()> = Result<T, DiagnosticError>;

/// An enum representing an error that happened while looking up a file or a piece of content in that file.
#[derive(Debug)]
#[non_exhaustive]
pub enum DiagnosticError {
    /// A required file is not in the file database.
    FileMissing,
    /// The file is present, but does not contain the specified byte index.
    IndexTooLarge { given: usize, max: usize },
    /// The file is present, but does not contain the specified line index.
    LineTooLarge { given: usize, max: usize },
    /// The file is present and contains the specified line index, but the line does not contain the specified column index.
    ColumnTooLarge { given: usize, max: usize },
    /// The given index is contained in the file, but is not a boundary of a UTF-8 code point.
    InvalidCharBoundary { given: usize },
    /// There was a error while doing IO.
    IOError(std::io::Error),
}

impl From<std::io::Error> for DiagnosticError {
    fn from(err: std::io::Error) -> DiagnosticError {
        DiagnosticError::IOError(err)
    }
}

impl Display for DiagnosticError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DiagnosticError::FileMissing => write!(f, "file missing"),
            DiagnosticError::IndexTooLarge { given, max } => {
                write!(f, "invalid index {}, maximum index is {}", given, max)
            }
            DiagnosticError::LineTooLarge { given, max } => {
                write!(f, "invalid line {}, maximum line is {}", given, max)
            }
            DiagnosticError::ColumnTooLarge { given, max } => {
                write!(f, "invalid column {}, maximum column {}", given, max)
            }
            DiagnosticError::InvalidCharBoundary { .. } => write!(f, "index is not a code point boundary"),
            DiagnosticError::IOError(err) => write!(f, "{}", err),
        }
    }
}

impl Error for DiagnosticError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self {
            DiagnosticError::IOError(err) => Some(err),
            _ => None,
        }
    }
}

/// A user-facing location in a source file.
///
/// Returned by [`Files::location`].
///
/// [`Files::location`]: Files::location
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Location {
    /// The user-facing line number.
    pub line_number: usize,
    /// The user-facing column number.
    pub column_number: usize,
}

/// The column index at the given byte index in the source file.
/// This is the number of characters to the given byte index.
///
/// If the byte index is smaller than the start of the line, then `0` is returned.
/// If the byte index is past the end of the line, the column index of the last
/// character `+ 1` is returned.
///
/// # Example
///
/// ```rust
/// use codespan_reporting::files;
///
/// let source = "\n\n🗻∈🌏\n\n";
///
/// assert_eq!(files::column_index(source, 0..1, 0), 0);
/// assert_eq!(files::column_index(source, 2..13, 0), 0);
/// assert_eq!(files::column_index(source, 2..13, 2 + 0), 0);
/// assert_eq!(files::column_index(source, 2..13, 2 + 1), 0);
/// assert_eq!(files::column_index(source, 2..13, 2 + 4), 1);
/// assert_eq!(files::column_index(source, 2..13, 2 + 8), 2);
/// assert_eq!(files::column_index(source, 2..13, 2 + 10), 2);
/// assert_eq!(files::column_index(source, 2..13, 2 + 11), 3);
/// assert_eq!(files::column_index(source, 2..13, 2 + 12), 3);
/// ```
pub fn column_index(source: &str, line_range: Range<usize>, byte_index: usize) -> usize {
    let end_index = std::cmp::min(byte_index, std::cmp::min(line_range.end, source.len()));

    (line_range.start..end_index).filter(|byte_index| source.is_char_boundary(byte_index + 1)).count()
}

/// Return the starting byte index of each line in the source string.
///
/// This can make it easier to implement [`Files::line_index`] by allowing
/// implementors of [`Files`] to pre-compute the line starts, then search for
/// the corresponding line range, as shown in the example below.
///
/// [`Files`]: Files
/// [`Files::line_index`]: Files::line_index
///
/// # Example
///
/// ```rust
/// use codespan_reporting::files;
///
/// let source = "foo\nbar\r\n\nbaz";
/// let line_starts: Vec<_> = files::line_starts(source).collect();
///
/// assert_eq!(
///     line_starts,
///     [
///         0,  // "foo\n"
///         4,  // "bar\r\n"
///         9,  // ""
///         10, // "baz"
///     ],
/// );
///
/// fn line_index(line_starts: &[usize], byte_index: usize) -> Option<usize> {
///     match line_starts.binary_search(&byte_index) {
///         Ok(line) => Some(line),
///         Err(next_line) => Some(next_line - 1),
///     }
/// }
///
/// assert_eq!(line_index(&line_starts, 5), Some(1));
/// ```
// NOTE: this is copied in `codespan::file::line_starts` and should be kept in sync.
pub fn line_starts(source: &str) -> impl '_ + Iterator<Item = usize> {
    std::iter::once(0).chain(source.match_indices('\n').map(|(i, _)| i + 1))
}

#[cfg(test)]
mod test {
    use crate::text_cache::TextCache;

    const TEST_SOURCE: &str = "foo\nbar\r\n\nbaz";

    #[test]
    fn line_starts() {
        let file = TextCache::anonymous(TEST_SOURCE);
        assert_eq!(
            file.line_starts,
            [
                0,  // "foo\n"
                4,  // "bar\r\n"
                9,  // ""
                10, // "baz"
            ],
        );
    }

    #[test]
    fn line_span_sources() {
        let file = TextCache::anonymous(TEST_SOURCE);
        let line_sources = (0..4)
            .map(|line| {
                let line_range = file.line_range(line).unwrap();
                &file.source[line_range]
            })
            .collect::<Vec<_>>();
        assert_eq!(line_sources, ["foo\n", "bar\r\n", "\n", "baz"]);
    }
}
