use crate::{SourceID, SourcePath, SourceText, Url};
use std::{borrow::Cow, collections::HashMap, path::Path};
mod display;

/// A [`Cache`] that fetches [`SourceText`]s from the filesystem.
#[derive(Default, Debug, Clone)]
pub struct SourceCache {
    cache: HashMap<SourceID, SourceText>,
}

impl SourceCache {
    /// Create a new [`SourceCache`].
    pub fn load_local<P>(&mut self, path: P) -> Result<SourceID, std::io::Error>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();
        let text = std::fs::read_to_string(&path)?;
        let source = SourceText::from(text).with_path(path);
        let name_hash = source.source_id();
        self.cache.insert(name_hash, source);
        Ok(name_hash)
    }
    /// Create a new [`SourceCache`].
    pub fn load_remote(&mut self, url: Url) -> Result<SourceID, std::io::Error> {
        let path = url.as_ref();
        let text = std::fs::read_to_string(&path)?;
        let source = SourceText::from(text).with_remote(url);
        let name_hash = source.source_id();
        self.cache.insert(name_hash, source);
        Ok(name_hash)
    }

    /// Create a new [`SourceCache`].
    pub fn load_text<T, N>(&mut self, text: T, name: N) -> SourceID
    where
        T: ToString,
        N: ToString,
    {
        let source = SourceText::snippet(text.to_string(), name.to_string());
        let name_hash = source.source_id();
        self.cache.insert(name_hash, source);
        name_hash
    }
    /// Set the file identifier buy not update the context
    pub unsafe fn set_source<N>(&mut self, file: SourceID, source: N) -> bool
    where
        N: Into<Cow<'static, str>>,
    {
        match self.cache.get_mut(&file) {
            Some(s) => {
                s.set_source(SourcePath::Snippet(source.into()));
                true
            }
            None => false,
        }
    }
    /// Create a new [`SourceCache`].
    pub fn fetch(&self, file: &SourceID) -> Result<&SourceText, std::io::Error> {
        match self.cache.get(file) {
            Some(source) => Ok(source),
            None => Err(std::io::Error::new(std::io::ErrorKind::NotFound, format!("File {:?} not found", file))),
        }
    }
    /// Create a new [`SourceCache`].
    pub fn source_path(&self, file: &SourceID) -> Option<&SourcePath> {
        Some(&self.cache.get(file)?.get_source())
    }
}
