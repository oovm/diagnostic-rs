//! An example that shows how to implement a simple custom file database.
//! The database uses 32-bit file-ids, which could be useful for optimizing
//! memory usage.
//!
//! To run this example, execute the following command from the top level of
//! this repository:
//!
//! ```sh
//! cargo run --example custom_files
//! ```
use std::{
    cmp::Ordering,
    collections::{hash_map::DefaultHasher, BTreeMap},
    fmt::{Debug, Display, Formatter},
    fs::read_to_string,
    hash::{Hash, Hasher},
    ops::Range,
    path::{Path, PathBuf},
    string::ToString,
};

use serde::{Deserialize, Serialize};

use crate::{
    errors::DiagnosticError,
    text_cache::location::{column_index, line_starts},
    DiagnosticResult, FileID, Label, Location,
};

pub mod file_id;
pub mod labels;
pub mod level;
pub mod location;

#[derive(Debug, Clone)]
pub struct TextStorage {
    files: BTreeMap<FileID, TextCache>,
}

/// A file that is backed by an `Arc<String>`.
#[derive(Debug, Clone)]
pub struct TextCache {
    /// Path to original file
    pub path: Option<PathBuf>,
    /// The source code of the file.
    pub source: String,
    /// The starting byte indices in the source code.
    pub line_starts: Vec<usize>,
}

impl TextCache {
    pub fn anonymous(source: impl Display) -> Self {
        let mut out = Self { path: None, source: source.to_string(), line_starts: vec![] };
        out.line_starts = line_starts(&out.source).collect();
        out
    }
    pub fn file(file: PathBuf) -> DiagnosticResult<Self> {
        let mut out = Self { path: Some(file), source: String::new(), line_starts: vec![] };
        out.update()?;
        Ok(out)
    }
    pub fn update(&mut self) -> DiagnosticResult {
        match &self.path {
            Some(s) => {
                self.source = read_to_string(s)?;
                self.line_starts = line_starts(&self.source).collect();
            }
            None => {}
        }
        Ok(())
    }
    pub fn line_start(&self, line_index: usize) -> Result<usize, DiagnosticError> {
        match line_index.cmp(&self.line_starts.len()) {
            Ordering::Less => Ok(*self.line_starts.get(line_index).expect("failed despite previous check")),
            Ordering::Equal => Ok(self.source.len()),
            Ordering::Greater => Err(DiagnosticError::LineTooLarge { given: line_index, max: self.line_starts.len() - 1 }),
        }
    }
    /// The byte range of line in the source of the file.
    pub fn line_range(&self, line_index: usize) -> Result<Range<usize>, DiagnosticError> {
        let line_start = self.line_start(line_index)?;
        let next_line_start = self.line_start(line_index + 1)?;
        Ok(line_start..next_line_start)
    }
}

impl Default for TextStorage {
    fn default() -> Self {
        Self { files: Default::default() }
    }
}

impl TextStorage {
    /// Add a file to the database, returning the handle that can be used to
    /// refer to it again.
    pub fn file<P>(&mut self, file_path: P) -> DiagnosticResult<FileID>
    where
        P: AsRef<Path>,
    {
        let path = file_path.as_ref().to_path_buf();
        let id = FileID::try_from(&path)?;
        let file = TextCache::file(path)?;
        self.files.insert(id.clone(), file);
        Ok(id)
    }
    pub fn anonymous(&mut self, file_text: impl Display) -> FileID {
        let text = file_text.to_string();
        let id = FileID::from(&text);
        let file = TextCache::anonymous(text);
        self.files.insert(id.clone(), file);
        id
    }
    pub fn update(&mut self, name: &FileID) -> DiagnosticResult {
        match self.files.get_mut(name) {
            Some(s) => s.update()?,
            None => {}
        }
        Ok(())
    }
    /// Get the file corresponding to the given id.
    pub fn get(&self, file_id: &FileID) -> DiagnosticResult<&TextCache> {
        // match self.files.get(file) {
        //     None => {}
        //     Some(_) => {}
        // }
        self.files.get(file_id).ok_or(DiagnosticError::FileMissing)
    }
    /// The source code of a file.
    pub fn source(&self, file_id: &FileID) -> DiagnosticResult<&str> {
        Ok(&self.get(file_id)?.source)
    }
    /// The index of the line at the given byte index.
    /// If the byte index is past the end of the file, returns the maximum line index in the file.
    /// This means that this function only fails if the file is not present.
    ///
    /// # Note for trait implementors
    ///
    /// This can be implemented efficiently by performing a binary search over
    /// a list of line starts that was computed by calling the [`line_starts`]
    /// function that is exported from the [`errors`] module. It might be useful
    /// to pre-compute and cache these line starts.
    ///
    /// [`line_starts`]: crate::errors::line_starts
    /// [`errors`]: crate::errors
    pub fn line_index(&self, file_id: &FileID, byte_index: usize) -> DiagnosticResult<usize> {
        self.get(file_id)?.line_starts.binary_search(&byte_index).or_else(|next_line| Ok(next_line - 1))
    }
    /// The user-facing line number at the given line index.
    /// It is not necessarily checked that the specified line index
    /// is actually in the file.
    ///
    /// # Note for trait implementors
    ///
    /// This is usually 1-indexed from the beginning of the file, but
    /// can be useful for implementing something like the
    /// [C preprocessor's `#line` macro][line-macro].
    ///
    /// [line-macro]: https://en.cppreference.com/w/c/preprocessor/line
    #[allow(unused_variables)]
    pub fn line_number(&self, file_id: &FileID, line_index: usize) -> DiagnosticResult<usize> {
        Ok(line_index + 1)
    }
    /// The user-facing column number at the given line index and byte index.
    ///
    /// # Note for trait implementors
    ///
    /// This is usually 1-indexed from the the start of the line.
    /// A default implementation is provided, based on the [`column_index`]
    /// function that is exported from the [`errors`] module.
    ///
    /// [`errors`]: crate::errors
    /// [`column_index`]: crate::errors::column_index
    pub fn column_number(&self, file_id: &FileID, line_index: usize, byte_index: usize) -> DiagnosticResult<usize> {
        let source = self.source(file_id)?;
        let line_range = self.line_range(file_id, line_index)?;
        let column_index = column_index(source.as_ref(), line_range, byte_index);

        Ok(column_index + 1)
    }
    /// Convenience method for returning line and column number at the given
    /// byte index in the file.
    pub fn location(&self, file_id: &FileID, byte_index: usize) -> DiagnosticResult<Location> {
        let line_index = self.line_index(file_id, byte_index)?;

        Ok(Location {
            line_number: self.line_number(file_id, line_index)?,
            column_number: self.column_number(file_id, line_index, byte_index)?,
        })
    }
    /// The byte range of line in the source of the file.
    pub fn line_range(&self, file_id: &FileID, line_index: usize) -> DiagnosticResult<Range<usize>> {
        self.get(file_id)?.line_range(line_index)
    }
}
