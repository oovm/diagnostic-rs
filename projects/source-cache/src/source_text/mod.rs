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
