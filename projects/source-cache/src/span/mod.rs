use crate::SourceID;
use std::{
    fmt::{Debug, Display, Formatter},
    ops::Range,
};

/// A type representing a single line of a [`Source`].
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FileSpan {
    pub start: u32,
    pub end: u32,
    pub file: SourceID,
}

impl Default for FileSpan {
    fn default() -> Self {
        Self { start: 0, end: 0, file: SourceID::default() }
    }
}

impl Debug for FileSpan {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FileSpan").field("start", &self.start).field("end", &self.end).field("file", &self.file).finish()
    }
}

impl Display for FileSpan {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "FileSpan(0x{:X}, {}..{})", self.file.hash, self.start, self.end)
    }
}

impl SourceID {
    /// Create a new [`SourceID`] with the given ID.
    pub unsafe fn new(id: u64) -> Self {
        Self { hash: id }
    }

    /// Create a new [`SourceID`] with the given ID.
    pub fn with_range(self, range: Range<u32>) -> FileSpan {
        FileSpan { start: range.start, end: range.end, file: self }
    }
}

impl FileSpan {
    /// Create a new span with the given start and end offsets, and the given file.
    pub fn new(file: SourceID, start: u32, end: u32) -> Self {
        Self { start, end, file }
    }
    /// Create a new span with the given start and end offsets, and the given file.
    pub fn get_range(&self) -> Range<u32> {
        self.start..self.end
    }
    /// Get the start offset of this span.
    ///
    /// Offsets are zero-indexed character offsets from the beginning of the identifier.
    pub fn get_start(&self) -> u32 {
        self.start
    }

    /// Get the (exclusive) end offset of this span.
    ///
    /// The end offset should *always* be greater than or equal to the start offset as given by [`Span::start`].
    ///
    /// Offsets are zero-indexed character offsets from the beginning of the identifier.
    pub fn get_end(&self) -> u32 {
        self.end
    }
    /// Create a new span with the given start and end offsets, and the given file.
    pub fn set_range(&mut self, range: Range<u32>) {
        self.start = range.start;
        self.end = range.end;
    }
    /// Create a new span with the given start and end offsets, and the given file.
    pub fn with_range(self, range: Range<u32>) -> Self {
        Self { start: range.start, end: range.end, ..self }
    }
    /// Create a new span with the given start and end offsets, and the given file.
    pub fn get_file(&self) -> SourceID {
        self.file
    }
    /// Create a new span with the given start and end offsets, and the given file.
    pub fn set_file(&mut self, file: SourceID) {
        self.file = file;
    }
    /// Create a new span with the given start and end offsets, and the given file.
    pub fn with_file(self, file: SourceID) -> Self {
        Self { file, ..self }
    }

    /// Get the length of this span (difference between the start of the span and the end of the span).
    pub fn length(&self) -> u32 {
        self.end.saturating_sub(self.start)
    }
    /// Determine whether the span contains the given offset.
    pub fn contains(&self, offset: u32) -> bool {
        self.get_range().contains(&offset)
    }
}
