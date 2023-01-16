use crate::SourceID;
use std::{
    fmt::{Debug, Display, Formatter},
    ops::Range,
};

mod display;

/// A type representing a single line of a [`Source`].
#[derive(Copy, Clone, Default, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SourceSpan {
    pub start: u32,
    pub end: u32,
    pub file: SourceID,
}

impl SourceID {
    /// Create a new [`SourceID`] with the given ID.
    pub unsafe fn new(id: u64) -> Self {
        Self { hash: id }
    }

    /// Create a new [`SourceID`] with the given ID.
    pub fn with_range(self, range: Range<u32>) -> SourceSpan {
        SourceSpan { start: range.start, end: range.end, file: self }
    }
}

impl SourceSpan {
    /// Create a new source_span with the given start and end offsets, and the given file.
    pub fn new(file: SourceID, start: u32, end: u32) -> Self {
        Self { start, end, file }
    }
    /// Create a new source_span with the given start and end offsets, and the given file.
    pub fn get_range(&self) -> Range<u32> {
        self.start..self.end
    }
    /// Get the start offset of this source_span.
    ///
    /// Offsets are zero-indexed character offsets from the beginning of the identifier.
    pub fn get_start(&self) -> u32 {
        self.start
    }

    /// Get the (exclusive) end offset of this source_span.
    ///
    /// The end offset should *always* be greater than or equal to the start offset as given by [`Span::start`].
    ///
    /// Offsets are zero-indexed character offsets from the beginning of the identifier.
    pub fn get_end(&self) -> u32 {
        self.end
    }
    /// Create a new source_span with the given start and end offsets, and the given file.
    pub fn set_range(&mut self, range: Range<u32>) {
        self.start = range.start;
        self.end = range.end;
    }
    /// Create a new source_span with the given start and end offsets, and the given file.
    pub fn with_range(self, range: Range<u32>) -> Self {
        Self { start: range.start, end: range.end, ..self }
    }
    /// Create a new source_span with the given start and end offsets, and the given file.
    pub fn get_file(&self) -> SourceID {
        self.file
    }
    /// Create a new source_span with the given start and end offsets, and the given file.
    pub fn set_file(&mut self, file: SourceID) {
        self.file = file;
    }
    /// Create a new source_span with the given start and end offsets, and the given file.
    pub fn with_file(self, file: SourceID) -> Self {
        Self { file, ..self }
    }

    /// Get the length of this source_span (difference between the start of the source_span and the end of the source_span).
    pub fn length(&self) -> u32 {
        self.end.saturating_sub(self.start)
    }
    /// Determine whether the source_span contains the given offset.
    pub fn contains(&self, offset: u32) -> bool {
        self.get_range().contains(&offset)
    }
}
