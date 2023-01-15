use super::*;
use std::borrow::Cow;

mod display;

use source_cache::SourcePath;
use std::fmt::Write;

/// A type representing a single line of a [`Source`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Line {
    /// Get the offset of this line in the original [`Source`] (i.e: the number of characters that precede it).
    pub offset: u32,
    /// Get the character length of this line.
    pub length: u32,
    chars: String,
}

impl Line {
    /// Get the character length of this line.
    pub fn get_length(&self) -> usize {
        self.length as usize
    }

    /// Get the offset span of this line in the original [`Source`].
    pub fn get_range(&self) -> Range<u32> {
        self.offset..self.offset + self.length
    }

    /// Return an iterator over the characters in the line, excluding trailing whitespace.
    pub fn chars(&self) -> impl Iterator<Item = char> + '_ {
        self.chars.chars()
    }
    /// Get the view of this line in the original [`Source`].
    pub fn view(&self) -> &str {
        &self.chars
    }
}

/// A type representing a single identifier that may be referred to by [`Span`]s.
///
/// In most cases, a identifier is a single input file.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Source {
    display_path: SourcePath,
    lines: Vec<Line>,
    /// bytes in identifier
    pub length: u32,
}

impl<S: AsRef<str>> From<S> for Source {
    /// Generate a [`Source`] from the given [`str`].
    ///
    /// Note that this function can be expensive for long strings. Use an implementor of [`Cache`] where possible.
    fn from(s: S) -> Self {
        let mut offset = 0;
        // (Last line, last line ends with CR)
        let mut last_line: Option<(Line, bool)> = None;
        let mut lines: Vec<Line> = s
            .as_ref()
            .split_inclusive([
                '\r',       // Carriage return
                '\n',       // Line feed
                '\x0B',     // Vertical tab
                '\x0C',     // Form feed
                '\u{0085}', // Next line
                '\u{2028}', // Line separator
                '\u{2029}', // Paragraph separator
            ])
            .flat_map(|line| {
                // Returns last line and set `last_line` to current `line`
                // A hack that makes `flat_map` deals with consecutive lines

                if let Some((last, ends_with_cr)) = last_line.as_mut() {
                    if *ends_with_cr && line == "\n" {
                        last.length += 1;
                        offset += 1;
                        return replace(&mut last_line, None).map(|(l, _)| l);
                    }
                }

                let len = line.len();
                let ends_with_cr = line.ends_with('\r');
                let line = Line { offset, length: len as u32, chars: line.trim_end().to_owned() };
                offset += line.length;
                replace(&mut last_line, Some((line, ends_with_cr))).map(|(l, _)| l)
            })
            .collect();

        if let Some((l, _)) = last_line {
            lines.push(l);
        }

        Self { display_path: SourcePath::Anonymous, lines, length: offset }
    }
}

impl Source {
    /// Get the length of the total number of characters in the identifier.
    pub fn get_length(&self) -> usize {
        self.length as usize
    }

    /// Return an iterator over the characters in the identifier.
    pub fn chars(&self) -> impl Iterator<Item = char> + '_ {
        self.lines.iter().map(|l| l.chars()).flatten()
    }

    /// Get access to a specific, zero-indexed [`Line`].
    pub fn line(&self, idx: usize) -> Option<&Line> {
        self.lines.get(idx)
    }

    /// Return an iterator over the [`Line`]s in this identifier.
    pub fn lines(&self) -> impl ExactSizeIterator<Item = &Line> + '_ {
        self.lines.iter()
    }

    /// Get the line that the given offset appears on, and the line/column numbers of the offset.
    ///
    /// Note that the line/column numbers are zero-indexed.
    pub fn get_offset_line(&self, offset: u32) -> Option<(&Line, usize, u32)> {
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
    /// Set path name of identifier
    pub fn set_path(&mut self, path: &Path) -> bool {
        self.display_path = SourcePath::Local(path.to_path_buf());
        true
    }
    /// Get path name of identifier
    pub fn with_path(mut self, path: &Path) -> Self {
        self.set_path(path);
        self
    }
    /// Set path name of identifier
    #[cfg(feature = "url")]
    pub fn set_remote(&mut self, url: Url) -> bool {
        self.display_path = SourcePath::Remote(url);
        true
    }
    /// Get path name of identifier
    #[cfg(feature = "url")]
    pub fn with_remote(mut self, url: Url) -> Self {
        self.set_remote(url);
        self
    }
    /// Get the range of lines that this span runs across.
    ///
    /// The resulting range is guaranteed to contain valid line indices (i.e: those that can be used for
    /// [`Source::line`]).
    pub fn get_line_range(&self, span: &Range<u32>) -> Range<usize> {
        let start = self.get_offset_line(span.start).map_or(0, |(_, l, _)| l);
        let end = self.get_offset_line(span.end.saturating_sub(1).max(span.start)).map_or(self.lines.len(), |(_, l, _)| l + 1);
        start..end
    }
}

/// A [`Cache`] that fetches [`Source`]s from the filesystem.
#[derive(Default, Debug, Clone)]
pub struct FileCache {
    files: HashMap<SourceID, Source>,
}

impl FileCache {
    /// Create a new [`FileCache`].
    pub fn load_local<P>(&mut self, path: P) -> Result<SourceID, std::io::Error>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();
        let text = std::fs::read_to_string(&path)?;
        let source = Source::from(text).with_path(path);
        let name_hash = source.display_path.source_id();
        self.files.insert(name_hash, source);
        Ok(name_hash)
    }
    /// Create a new [`FileCache`].
    #[cfg(feature = "url")]
    pub fn load_remote(&mut self, url: Url) -> Result<SourceID, std::io::Error> {
        let path = url.as_ref();
        let text = std::fs::read_to_string(&path)?;
        let source = Source::from(text).with_remote(url);
        let name_hash = source.display_path.source_id();
        self.files.insert(name_hash, source);
        Ok(name_hash)
    }

    /// Create a new [`FileCache`].
    pub fn load_text<T, N>(&mut self, text: T, name: N) -> SourceID
    where
        T: ToString,
        N: ToString,
    {
        let mut source = Source::from(text.to_string());
        let name = name.to_string();
        source.display_path = SourcePath::Snippet(Cow::Owned(name));
        let name_hash = source.display_path.source_id();
        self.files.insert(name_hash, source);
        name_hash
    }
    /// Set the file identifier buy not update the context
    pub unsafe fn set_source(&mut self, file: SourceID, source: String) -> bool {
        match self.files.get_mut(&file) {
            Some(s) => {
                s.display_path = SourcePath::Snippet(Cow::Owned(source));
                true
            }
            None => false,
        }
    }
    /// Create a new [`FileCache`].
    pub fn fetch(&self, file: &SourceID) -> Result<&Source, std::io::Error> {
        match self.files.get(file) {
            Some(source) => Ok(source),
            None => Err(std::io::Error::new(std::io::ErrorKind::NotFound, format!("File {:?} not found", file))),
        }
    }
    /// Create a new [`FileCache`].
    pub fn source_path(&self, file: &SourceID) -> Option<&SourcePath> {
        Some(&self.files.get(file)?.display_path)
    }
}
