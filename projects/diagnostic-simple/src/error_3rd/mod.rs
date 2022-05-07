use std::error::Error;

#[cfg(feature = "async-walkdir")]
pub use async_walkdir::WalkDir as AsyncWalkDir;
#[cfg(feature = "globset")]
pub use globset::{Glob, GlobSet, GlobSetBuilder};
#[cfg(feature = "toml")]
pub use toml::Value as Toml;
#[cfg(feature = "url")]
pub use url::Url;
#[cfg(feature = "walkdir")]
pub use walkdir::WalkDir;

use crate::{QError, QErrorKind, RuntimeError};

#[cfg(feature = "rust_decimal")]
pub use self::for_rust_decimal::*;

// #[cfg(feature = "lsp-types")]
// mod for_lsp;
#[cfg(feature = "num")]
mod for_num;
#[cfg(feature = "peginator")]
mod for_peginator;
#[cfg(feature = "ropey")]
mod for_ropey;
#[cfg(feature = "ucd-trie")]
mod for_ucd_trie;

#[cfg(feature = "url")]
mod for_url;

#[cfg(feature = "sled")]
mod for_sled;

#[cfg(feature = "rust_decimal")]
mod for_rust_decimal;
#[cfg(feature = "serde-binary")]
mod for_serde_binary;

#[cfg(feature = "tl")]
mod for_tl;

#[cfg(feature = "globset")]
mod for_globset;

#[cfg(feature = "serde")]
mod for_serde;

#[cfg(feature = "walkdir")]
mod for_walkdir;

#[cfg(feature = "async-walkdir")]
mod for_walkdir_async;

#[cfg(feature = "toml")]
mod for_toml;

#[cfg(feature = "serde_json")]
mod for_serde_json;

#[cfg(feature = "chrono")]
mod for_chrono;
#[cfg(feature = "mime")]
mod for_mime;

mod for_font_kit;

impl QError {
    #[inline]
    pub(crate) fn fast_runtime_error(error: impl Error + 'static) -> QError {
        QError {
            error: Box::new(QErrorKind::Runtime(RuntimeError::from(&error))),
            level: Default::default(),
            source: Some(Box::new(error)),
        }
    }
}
