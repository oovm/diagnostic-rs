use std::{
    borrow::Cow,
    fmt::{Debug, Display, Formatter},
    hash::{DefaultHasher, Hash, Hasher},
    path::PathBuf,
};
use url::Url;

mod display;

#[derive(Clone, Debug, Default, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SourcePath {
    /// This is an anonymous identifier
    #[default]
    Anonymous,
    /// This is a snippet of identifier
    Snippet(Cow<'static, str>),
    /// This is a local identifier
    Local(PathBuf),
    /// This is a remote identifier
    Remote(Url),
}

/// A type representing a single line of a [`Source`].
#[derive(Copy, Clone, Default, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SourceID {
    pub(crate) hash: u64,
}

impl SourcePath {
    /// Calculate the file from the identifier
    pub fn source_id(&self) -> SourceID {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        SourceID { hash: hasher.finish() }
    }
}
