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
    let mut files = TextCache::new();

    let file_id0 = files.add("0.greeting", "hello world!").unwrap();
    let file_id1 = files.add("1.greeting", "bye world").unwrap();

    let messages = vec![
        Message::UnwantedGreetings { greetings: vec![(file_id0, 0..5), (file_id1, 0..3)] },
        Message::OverTheTopExclamations { exclamations: vec![(file_id0, 11..12)] },
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
    pub fn add(&mut self, name: String, path: Option<PathBuf>) -> DiagnosticResult {
        let mut file = FileCache { name, line_starts: vec![], source: String::new(), path };
        file.update()?;

        match &path {
            Some(s) => {
                let line_starts = line_starts(&read_to_string(s)?).collect();
                self.files.insert(name.clone(), FileCache { name, line_starts, source, path });
            }
            None => {
                self.files.insert(name.clone(), file);
            }
        }
        Ok(())
    }
    pub fn update(&mut self, name: &str) -> DiagnosticResult {
        match self.files.get_mut(name) {
            Some(s) => {
                s.update()
            }
            None => { Ok(()) }
        }
    }

    /// Get the file corresponding to the given id.
    fn get(&self, file: &str) -> Result<&FileCache, DiagnosticError> {
        // match self.files.get(file) {
        //     None => {}
        //     Some(_) => {}
        // }
        self.files.get(file).ok_or(DiagnosticError::FileMissing)
    }

    fn name(&self, file_id: &str) -> Result<&str, DiagnosticError> {
        Ok(self.get(file_id)?.name.as_ref())
    }

    fn source(&self, file_id: &str) -> Result<&str, DiagnosticError> {
        Ok(&self.get(file_id)?.source)
    }

    fn line_index(&self, file_id: &str, byte_index: usize) -> Result<usize, DiagnosticError> {
        self.get(file_id)?.line_starts.binary_search(&byte_index).or_else(|next_line| Ok(next_line - 1))
    }

    fn line_range(&self, file_id: &str, line_index: usize) -> Result<Range<usize>, DiagnosticError> {
        let file = self.get(file_id)?;
        let line_start = file.line_start(line_index)?;
        let next_line_start = file.line_start(line_index + 1)?;

        Ok(line_start..next_line_start)
    }
}

/// A Diagnostic message.
enum Message {
    UnwantedGreetings { greetings: Vec<(String, Range<usize>)> },
    OverTheTopExclamations { exclamations: Vec<(String, Range<usize>)> },
}

impl Message {
    fn to_diagnostic(&self) -> Diagnostic<String> {
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
