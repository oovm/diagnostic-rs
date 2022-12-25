use std::{
    hash::{Hash, Hasher},
    ops::{Deref, DerefMut, Range},
};

use serde::{Deserialize, Serialize};

use diagnostic::FileID;

/// Represents an AST object with position
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NodeLocation<T> {
    /// The actual value
    pub value: T,
    /// The Start offset and end offset
    pub range: Range<usize>,
    /// Absolute path to the file where the node resides
    pub file: FileID,
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
    #[inline]
    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> NodeLocation<U> {
        NodeLocation { value: f(self.value), range: self.range, file: self.file }
    }
    #[inline]
    pub fn eq_strict(&self, other: &Self) -> bool
    where
        T: PartialEq,
    {
        self.value == other.value && self.range == other.range && self.file == other.file
    }
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

impl<T> From<T> for NodeLocation<T> {
    fn from(value: T) -> Self {
        Self { value, range: Default::default(), file: Default::default() }
    }
}

impl<T> PartialEq<Self> for NodeLocation<T>
where
    T: Eq,
{
    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&other.value)
    }
}

impl<T> Eq for NodeLocation<T> where T: Eq {}

impl<T> Hash for NodeLocation<T>
where
    T: Hash,
{
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.value.hash(state)
    }
}
