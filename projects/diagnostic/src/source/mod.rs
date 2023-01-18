use source_cache::{SourceID, SourceLine, SourcePath, Url};
use std::{
    borrow::Cow,
    collections::HashMap,
    fmt::{Display, Formatter, Write},
    ops::Range,
    path::Path,
};
mod display;

/// A type representing a single identifier that may be referred to by [`Span`]s.
///
/// In most cases, a identifier is a single input file.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct SourceText {
    /// The path of the identifier.
    pub path: SourcePath,
    /// The text
    pub text: String,
    /// The lines of the identifier.
    pub lines: Vec<SourceLine>,
    /// bytes in identifier
    pub length: u32,
    /// Is the data dirty
    pub dirty: bool,
}

impl SourceText {
    /// Get the length of the total number of characters in the identifier.
    pub fn get_length(&self) -> usize {
        self.length as usize
    }

    /// Get access to a specific, zero-indexed [`SourceLine`].
    pub fn get_line(&self, idx: usize) -> Option<&SourceLine> {
        self.lines.get(idx)
    }

    /// Set path name of identifier
    pub fn set_path(&mut self, path: &Path) -> bool {
        self.path = SourcePath::Local(path.to_path_buf());
        true
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

    /// Return an iterator over the characters in the identifier.
    pub fn chars(&self) -> impl Iterator<Item = char> + '_ {
        self.lines.iter().map(|l| l.chars()).flatten()
    }
    /// Return an iterator over the [`SourceLine`]s in this identifier.
    pub fn lines(&self) -> impl ExactSizeIterator<Item = &SourceLine> + '_ {
        self.lines.iter()
    }
    /// Clear the source cache
    pub fn clear(&mut self) {
        self.text.clear();
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

/// A [`Cache`] that fetches [`SourceText`]s from the filesystem.
#[derive(Default, Debug, Clone)]
pub struct SourceCache {
    files: HashMap<SourceID, SourceText>,
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
        let name_hash = source.path.source_id();
        self.files.insert(name_hash, source);
        Ok(name_hash)
    }
    /// Create a new [`SourceCache`].
    #[cfg(feature = "url")]
    pub fn load_remote(&mut self, url: Url) -> Result<SourceID, std::io::Error> {
        let path = url.as_ref();
        let text = std::fs::read_to_string(&path)?;
        let source = SourceText::from(text).with_remote(url);
        let name_hash = source.path.source_id();
        self.files.insert(name_hash, source);
        Ok(name_hash)
    }

    /// Create a new [`SourceCache`].
    pub fn load_text<T, N>(&mut self, text: T, name: N) -> SourceID
    where
        T: ToString,
        N: ToString,
    {
        let mut source = SourceText::from(text.to_string());
        let name = name.to_string();
        source.path = SourcePath::Snippet(Cow::Owned(name));
        let name_hash = source.path.source_id();
        self.files.insert(name_hash, source);
        name_hash
    }
    /// Set the file identifier buy not update the context
    pub unsafe fn set_source(&mut self, file: SourceID, source: String) -> bool {
        match self.files.get_mut(&file) {
            Some(s) => {
                s.path = SourcePath::Snippet(Cow::Owned(source));
                true
            }
            None => false,
        }
    }
    /// Create a new [`SourceCache`].
    pub fn fetch(&self, file: &SourceID) -> Result<&SourceText, std::io::Error> {
        match self.files.get(file) {
            Some(source) => Ok(source),
            None => Err(std::io::Error::new(std::io::ErrorKind::NotFound, format!("File {:?} not found", file))),
        }
    }
    /// Create a new [`SourceCache`].
    pub fn source_path(&self, file: &SourceID) -> Option<&SourcePath> {
        Some(&self.files.get(file)?.path)
    }
}
