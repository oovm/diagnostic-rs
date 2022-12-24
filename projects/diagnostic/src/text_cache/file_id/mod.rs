use std::rc::Rc;

use super::*;

mod ser_der;
mod try_from;

/// A file identifier, pointing to actual file in [`TextStorage`].
///
/// # Arguments
///
/// * `source`:
///
/// returns: Result<FileID, <E as TryInto<FileID>>::Error>
///
/// # Examples
///
/// ```
/// use diagnostic::FileID;
/// FileID::file_id_from_file("file_id.rs").unwrap();
/// ```
#[derive(Eq, Clone)]
pub struct FileID {
    hash: Rc<str>,
}

impl Default for FileID {
    fn default() -> Self {
        Self { hash: Rc::from(String::new()) }
    }
}

impl AsRef<str> for FileID {
    fn as_ref(&self) -> &str {
        &self.hash
    }
}

impl AsRef<Path> for FileID {
    fn as_ref(&self) -> &Path {
        Path::new(AsRef::<str>::as_ref(self))
    }
}

impl Debug for FileID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("FileID").field(&self.hash).finish()
    }
}

impl Display for FileID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.hash)
    }
}

impl Hash for FileID {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(self.hash.as_bytes())
    }
}

impl PartialEq for FileID {
    fn eq(&self, other: &Self) -> bool {
        self.hash.eq(&other.hash)
    }
}

impl PartialOrd for FileID {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.hash.partial_cmp(&other.hash)
    }
}

impl Ord for FileID {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hash.cmp(&other.hash)
    }
}
