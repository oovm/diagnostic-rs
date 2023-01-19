use crate::{SourceID, SourcePath};
use std::{
    borrow::Cow,
    fmt::{Debug, Display, Formatter},
    ops::Range,
    path::Path,
};
use url::Url;

mod display;

/// A type representing a single identifier that may be referred to by [`Span`]s.
///
/// In most cases, an identifier is a single input file.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct SourceText {
    /// The path of the identifier.
    path: SourcePath,
    /// The text
    raw: String,
    /// The lines of the identifier.
    lines: Vec<SourceLine>,
    /// bytes in identifier
    length: u32,
    /// Is the data dirty
    dirty: bool,
}

/// A type representing a single line of a [`SourceText`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct SourceLine {
    /// Get the offset of this line in the original [`SourceText`] (i.e: the number of characters that precede it).
    pub offset: u32,
    /// Get the character length of this line.
    pub length: u32,
    /// Get the view of this line in the original [`SourceText`].
    pub text: String,
}

/// A type representing a single line of a [`Source`].
#[derive(Copy, Clone, Default, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SourceSpan {
    /// The start offset of the span
    pub start: u32,
    /// The end offset of the span
    pub end: u32,
    /// The file id of the span
    pub file: SourceID,
}

impl SourceText {
    /// Create a snippet with given name
    pub fn snippet<S, N>(text: S, name: N) -> Self
    where
        S: Into<String>,
        N: Into<Cow<'static, str>>,
    {
        let mut src = Self::from(text);
        src.path = SourcePath::Snippet(name.into());
        src
    }

    /// Get the cache id
    pub fn source_id(&self) -> SourceID {
        self.path.source_id()
    }
    /// Get the length of the total number of characters in the identifier.
    pub fn get_length(&self) -> usize {
        self.length as usize
    }
    /// Get access to a specific, zero-indexed [`SourceLine`].
    pub fn get_line(&self, idx: usize) -> Option<&SourceLine> {
        self.lines.get(idx)
    }
    /// Get the length of the total number of characters in the identifier.
    pub fn get_source(&self) -> &SourcePath {
        &self.path
    }
    /// Set path name of identifier
    pub fn set_source(&mut self, path: SourcePath) {
        self.path = path;
    }
    /// Set path name of identifier
    pub fn set_path(&mut self, path: &Path) {
        self.path = SourcePath::Local(path.to_path_buf());
    }
    /// Get path name of identifier
    pub fn with_path(self, path: &Path) -> Self {
        Self { path: SourcePath::Local(path.to_path_buf()), ..self }
    }
    /// Set path name of identifier
    pub fn set_remote(&mut self, url: Url) -> bool {
        self.path = SourcePath::Remote(url);
        true
    }
    /// Get path name of identifier
    pub fn with_remote(self, url: Url) -> Self {
        Self { path: SourcePath::Remote(url), ..self }
    }

    /// Return the raw text fetch from source
    pub fn text(&self) -> &str {
        self.raw.as_str()
    }
    /// Return an iterator over the [`SourceLine`]s in this identifier.
    pub fn lines(&self) -> &[SourceLine] {
        self.lines.as_slice()
    }
    /// Clear the cache cache
    pub fn clear(&mut self) {
        self.raw.clear();
        self.lines.clear();
        self.dirty = true;
    }
}
impl SourceText {
    /// Get the line that the given offset appears on, and the line/column numbers of the offset.
    ///
    /// Note that the line/column numbers are zero-indexed.
    pub fn get_offset_line(&self, offset: u32) -> Option<(&SourceLine, usize, u32)> {
        if offset <= self.length {
            let idx = self.lines.binary_search_by_key(&offset, |line| line.offset).unwrap_or_else(|idx| idx.saturating_sub(1));
            let line = &self.lines[idx];
            assert!(offset >= line.offset, "offset = {}, line.offset = {}", offset, line.offset);
            Some((line, idx, offset - line.offset))
        }
        else {
            None
        }
    }
    /// Get the range of lines that this source_text runs across.
    ///
    /// The resulting range is guaranteed to contain valid line indices (i.e: those that can be used for
    /// [`SourceText::get_line`]).
    pub fn get_line_range(&self, span: &Range<u32>) -> Range<usize> {
        let start = self.get_offset_line(span.start).map_or(0, |(_, l, _)| l);
        let end = self.get_offset_line(span.end.saturating_sub(1).max(span.start)).map_or(self.lines.len(), |(_, l, _)| l + 1);
        start..end
    }
}

impl SourceSpan {
    /// Create a new source_text with the given start and end offsets, and the given file.
    pub fn new(file: SourceID, start: u32, end: u32) -> Self {
        Self { start, end, file }
    }
    /// Create a new source_text with the given start and end offsets, and the given file.
    pub fn get_range(&self) -> Range<u32> {
        self.start..self.end
    }
    /// Get the start offset of this source_text.
    ///
    /// Offsets are zero-indexed character offsets from the beginning of the identifier.
    pub fn get_start(&self) -> u32 {
        self.start
    }

    /// Get the (exclusive) end offset of this source_text.
    ///
    /// The end offset should *always* be greater than or equal to the start offset as given by [`Span::start`].
    ///
    /// Offsets are zero-indexed character offsets from the beginning of the identifier.
    pub fn get_end(&self) -> u32 {
        self.end
    }
    /// Create a new source_text with the given start and end offsets, and the given file.
    pub fn set_range(&mut self, range: Range<u32>) {
        self.start = range.start;
        self.end = range.end;
    }
    /// Create a new source_text with the given start and end offsets, and the given file.
    pub fn with_range(self, range: Range<u32>) -> Self {
        Self { start: range.start, end: range.end, ..self }
    }
    /// Create a new source_text with the given start and end offsets, and the given file.
    pub fn get_file(&self) -> SourceID {
        self.file
    }
    /// Create a new source_text with the given start and end offsets, and the given file.
    pub fn set_file(&mut self, file: SourceID) {
        self.file = file;
    }
    /// Create a new source_text with the given start and end offsets, and the given file.
    pub fn with_file(self, file: SourceID) -> Self {
        Self { file, ..self }
    }

    /// Get the length of this source_text (difference between the start of the source_text and the end of the source_text).
    pub fn length(&self) -> u32 {
        self.end.saturating_sub(self.start)
    }
    /// Determine whether the source_text contains the given offset.
    pub fn contains(&self, offset: u32) -> bool {
        self.get_range().contains(&offset)
    }
}
impl SourceLine {
    /// Get the offset source_text of this line in the original [`SourceText`].
    pub fn range(&self) -> Range<u32> {
        self.offset..self.offset + self.length
    }

    /// Return an iterator over the characters in the line, excluding trailing whitespace.
    pub fn chars(&self) -> impl Iterator<Item = char> + '_ {
        self.text.chars()
    }
}
