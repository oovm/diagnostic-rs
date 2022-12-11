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
    repr: Rc<str>,
}

impl Default for FileID {
    fn default() -> Self {
        Self { repr: Rc::from(String::new()) }
    }
}

impl AsRef<str> for FileID {
    fn as_ref(&self) -> &str {
        &self.repr
    }
}

impl AsRef<Path> for FileID {
    fn as_ref(&self) -> &Path {
        Path::new(AsRef::<str>::as_ref(self))
    }
}

impl Debug for FileID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("FileID").field(&self.repr).finish()
    }
}

impl Display for FileID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.repr)
    }
}

impl Hash for FileID {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(self.repr.as_bytes())
    }
}

impl PartialEq for FileID {
    fn eq(&self, other: &Self) -> bool {
        self.repr.eq(&other.repr)
    }
}

impl PartialOrd for FileID {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.repr.partial_cmp(&other.repr)
    }
}

impl Ord for FileID {
    fn cmp(&self, other: &Self) -> Ordering {
        self.repr.cmp(&other.repr)
    }
}
