use super::*;

use std::{
    collections::HashMap,
    hash::{BuildHasher, Hasher},
    mem::replace,
    path::PathBuf,
};

/// A type representing a single line of a [`Source`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Line {
    offset: usize,
    len: usize,
    chars: String,
}

impl Line {
    /// Get the offset of this line in the original [`Source`] (i.e: the number of characters that precede it).
    pub fn offset(&self) -> usize {
        self.offset
    }

    /// Get the character length of this line.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Get the offset span of this line in the original [`Source`].
    pub fn span(&self) -> Range<usize> {
        self.offset..self.offset + self.len
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

/// A type representing a single source that may be referred to by [`Span`]s.
///
/// In most cases, a source is a single input file.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Source {
    file_name: String,
    lines: Vec<Line>,
    len: usize,
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
                        last.len += 1;
                        offset += 1;
                        return replace(&mut last_line, None).map(|(l, _)| l);
                    }
                }

                let len = line.chars().count();
                let ends_with_cr = line.ends_with('\r');
                let line = Line { offset, len, chars: line.trim_end().to_owned() };
                offset += len;
                replace(&mut last_line, Some((line, ends_with_cr))).map(|(l, _)| l)
            })
            .collect();

        if let Some((l, _)) = last_line {
            lines.push(l);
        }

        Self { file_name: "<anonymous>".to_string(), lines, len: offset }
    }
}

impl Source {
    /// Get the length of the total number of characters in the source.
    pub fn length(&self) -> usize {
        self.len
    }

    /// Return an iterator over the characters in the source.
    pub fn chars(&self) -> impl Iterator<Item = char> + '_ {
        self.lines.iter().map(|l| l.chars()).flatten()
    }

    /// Get access to a specific, zero-indexed [`Line`].
    pub fn line(&self, idx: usize) -> Option<&Line> {
        self.lines.get(idx)
    }

    /// Return an iterator over the [`Line`]s in this source.
    pub fn lines(&self) -> impl ExactSizeIterator<Item = &Line> + '_ {
        self.lines.iter()
    }

    /// Get the line that the given offset appears on, and the line/column numbers of the offset.
    ///
    /// Note that the line/column numbers are zero-indexed.
    pub fn get_offset_line(&self, offset: usize) -> Option<(&Line, usize, usize)> {
        if offset <= self.len {
            let idx = self.lines.binary_search_by_key(&offset, |line| line.offset).unwrap_or_else(|idx| idx.saturating_sub(1));
            let line = &self.lines[idx];
            assert!(offset >= line.offset, "offset = {}, line.offset = {}", offset, line.offset);
            Some((line, idx, offset - line.offset))
        }
        else {
            None
        }
    }

    /// Get the range of lines that this span runs across.
    ///
    /// The resulting range is guaranteed to contain valid line indices (i.e: those that can be used for
    /// [`Source::line`]).
    pub fn get_line_range(&self, span: &Range<usize>) -> Range<usize> {
        let start = self.get_offset_line(span.start).map_or(0, |(_, l, _)| l);
        let end = self.get_offset_line(span.end.saturating_sub(1).max(span.start)).map_or(self.lines.len(), |(_, l, _)| l + 1);
        start..end
    }
}

/// A [`Cache`] that fetches [`Source`]s from the filesystem.
#[derive(Default, Debug, Clone)]
pub struct FileCache {
    files: HashMap<FileID, Source>,
}

impl FileCache {
    /// Create a new [`FileCache`].
    pub fn load_local<P>(&mut self, path: P) -> Result<FileID, std::io::Error>
    where
        P: AsRef<PathBuf>,
    {
        let path = path.as_ref();
        let hasher = self.files.hasher();
        let name_hash = {
            let mut hasher = hasher.build_hasher();
            path.hash(&mut hasher);
            FileID { hash: hasher.finish() }
        };
        let text = std::fs::read_to_string(path)?;
        let source = Source::from(text);
        self.files.insert(name_hash, source);
        Ok(name_hash)
    }
    /// Create a new [`FileCache`].
    pub fn load_text<T, N>(&mut self, text: T, name: N) -> FileID
    where
        T: ToString,
        N: ToString,
    {
        let name = name.to_string();
        let hasher = self.files.hasher();
        let name_hash = {
            let mut hasher = hasher.build_hasher();
            name.hash(&mut hasher);
            FileID { hash: hasher.finish() }
        };
        let mut source = Source::from(text.to_string());
        source.file_name = name;
        self.files.insert(name_hash, source);
        name_hash
    }
    /// Create a new [`FileCache`].
    pub fn fetch(&mut self, file: &FileID) -> Result<&Source, std::io::Error> {
        match self.files.get(file) {
            Some(source) => Ok(source),
            None => Err(std::io::Error::new(std::io::ErrorKind::NotFound, format!("File {:?} not found", file))),
        }
    }
    /// Create a new [`FileCache`].
    pub fn display(&self, file: &FileID) -> Option<&str> {
        Some(self.files.get(file)?.file_name.as_str())
    }
}
