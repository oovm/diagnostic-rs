#![doc = include_str!("../readme.md")]
#![warn(missing_docs)]

mod cache;
mod identifier;
mod text;

pub use crate::{
    cache::SourceCache,
    identifier::{SourceID, SourcePath},
    text::{SourceLine, SourceSpan, SourceText},
};
pub use url::Url;
