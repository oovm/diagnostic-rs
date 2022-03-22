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

use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::fs::read_to_string;
use std::ops::Range;
use std::path::PathBuf;

use crate::{diagnostic::{Diagnostic, Label}, DiagnosticResult, term, term::termcolor::{ColorChoice, StandardStream}};
use crate::errors::{DiagnosticError, line_starts};

#[test]
fn main() -> anyhow::Result<()> {
    let mut files = TextCache::default();

    files.add("0.greeting".to_string(), None).ok();
    files.add("1.greeting".to_string(), None).ok();

    let messages = vec![
        Message::UnwantedGreetings { greetings: vec![(0..5), (0..3)] },
        Message::OverTheTopExclamations { exclamations: vec![(11..12)] },
    ];

    let writer = StandardStream::stderr(ColorChoice::Always);
    let config = term::Config::default();
    for message in &messages {
        let writer = &mut writer.lock();
        term::emit(writer, &config, &files, &message.to_diagnostic())?;
    }

    Ok(())
}

#[derive(Debug, Clone)]
pub struct TextCache {
    files: BTreeMap<String, FileCache>,
}

/// A file that is backed by an `Arc<String>`.
#[derive(Debug, Clone)]
pub struct FileCache {
    /// The name of the file.
    name: String,
    /// path
    path: Option<PathBuf>,
    /// The source code of the file.
    source: String,
    /// The starting byte indices in the source code.
    line_starts: Vec<usize>,
}

impl FileCache {
    fn update(&mut self) -> DiagnosticResult {
        match &self.path {
            Some(s) => {
                self.source = read_to_string(s)?;
                self.line_starts = line_starts(&self.source).collect();
            }
            None => {}
        }
        Ok(())
    }

    fn line_start(&self, line_index: usize) -> Result<usize, DiagnosticError> {
        match line_index.cmp(&self.line_starts.len()) {
            Ordering::Less => Ok(*self.line_starts.get(line_index).expect("failed despite previous check")),
            Ordering::Equal => Ok(self.source.len()),
            Ordering::Greater => Err(DiagnosticError::LineTooLarge { given: line_index, max: self.line_starts.len() - 1 }),
        }
    }
}

impl Default for TextCache {
    fn default() -> Self {
        Self {
            files: Default::default()
        }
    }
}

impl TextCache {
    /// Add a file to the database, returning the handle that can be used to
    /// refer to it again.
    pub fn add(&mut self, file_id: String, file_path: Option<PathBuf>) -> DiagnosticResult {
        let mut file = FileCache { name: file_id, line_starts: vec![], source: String::new(), path: file_path };
        file.update()?;
        self.files.insert(file_id.clone(), file);
        Ok(())
    }
    pub fn update(&mut self, name: &str) -> DiagnosticResult {
        match self.files.get_mut(name) {
            Some(s) => {
                s.update()?
            }
            None => {  }
        }
        Ok(())
    }

    /// Get the file corresponding to the given id.
   pub fn get(&self, file: &str) -> Result<&FileCache, DiagnosticError> {
        // match self.files.get(file) {
        //     None => {}
        //     Some(_) => {}
        // }
        self.files.get(file).ok_or(DiagnosticError::FileMissing)
    }

    pub fn name(&self, file_id: &str) -> Result<&str, DiagnosticError> {
        Ok(self.get(file_id)?.name.as_ref())
    }

    pub fn source(&self, file_id: &str) -> Result<&str, DiagnosticError> {
        Ok(&self.get(file_id)?.source)
    }

    pub fn line_index(&self, file_id: &str, byte_index: usize) -> Result<usize, DiagnosticError> {
        self.get(file_id)?.line_starts.binary_search(&byte_index).or_else(|next_line| Ok(next_line - 1))
    }

    pub fn line_range(&self, file_id: &str, line_index: usize) -> Result<Range<usize>, DiagnosticError> {
        let file = self.get(file_id)?;
        let line_start = file.line_start(line_index)?;
        let next_line_start = file.line_start(line_index + 1)?;

        Ok(line_start..next_line_start)
    }
}

/// A Diagnostic message.
enum Message {
    UnwantedGreetings { greetings: Vec<Range<usize>> },
    OverTheTopExclamations { exclamations: Vec<Range<usize>> },
}

impl Message {
    fn to_diagnostic(&self) -> Diagnostic {
        match self {
            Message::UnwantedGreetings { greetings } => Diagnostic::error()
                .with_message("greetings are not allowed")
                .with_labels(
                    greetings
                        .iter()
                        .map(|(file_id, range)| Label::primary(*file_id, range.clone()).with_message("a greeting"))
                        .collect(),
                )
                .with_notes(vec!["found greetings!".to_owned(), "pleas no greetings :(".to_owned()]),
            Message::OverTheTopExclamations { exclamations } => Diagnostic::error()
                .with_message("over-the-top exclamations")
                .with_labels(
                    exclamations
                        .iter()
                        .map(|(file_id, range)| Label::primary(*file_id, range.clone()).with_message("an exclamation"))
                        .collect(),
                )
                .with_notes(vec!["ridiculous!".to_owned()]),
        }
    }
}
