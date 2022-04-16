use std::ops::Range;

pub type Span = Range<usize>;

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
/// use diagnostic::column_index;
///
/// let source = "\n\n🗻∈🌏\n\n";
///
/// assert_eq!(column_index(source, 0..1, 0), 0);
/// assert_eq!(column_index(source, 2..13, 0), 0);
/// assert_eq!(column_index(source, 2..13, 2 + 0), 0);
/// assert_eq!(column_index(source, 2..13, 2 + 1), 0);
/// assert_eq!(column_index(source, 2..13, 2 + 4), 1);
/// assert_eq!(column_index(source, 2..13, 2 + 8), 2);
/// assert_eq!(column_index(source, 2..13, 2 + 10), 2);
/// assert_eq!(column_index(source, 2..13, 2 + 11), 3);
/// assert_eq!(column_index(source, 2..13, 2 + 12), 3);
/// ```
pub fn column_index(source: &str, line_range: Span, byte_index: usize) -> usize {
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
/// use diagnostic::line_starts;
///
/// let source = "foo\nbar\r\n\nbaz";
/// let line_starts: Vec<_> = line_starts(source).collect();
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
