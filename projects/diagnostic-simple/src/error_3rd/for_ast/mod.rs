use std::ops::{Deref, DerefMut, Range};

use serde::{Deserialize, Serialize};

use diagnostic::FileID;

/// Represents an AST object with position
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct NodeLocation<T> {
    /// The actual value
    pub value: T,
    /// The Start offset and end offset
    pub range: Range<usize>,
    /// Absolute path to the file where the node resides
    pub file: FileID,
}

impl<T> Deref for NodeLocation<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for NodeLocation<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl<T> NodeLocation<T> {
    #[inline]
    pub fn new(value: T, range: &Range<usize>, file: &FileID) -> Self {
        Self { value, range: range.clone(), file: file.clone() }
    }
    #[inline]
    pub fn with_range(mut self, range: &Range<usize>) -> Self {
        self.range = range.clone();
        self
    }
    #[inline]
    pub fn with_file(mut self, file: &FileID) -> Self {
        self.file = file.clone();
        self
    }
}
