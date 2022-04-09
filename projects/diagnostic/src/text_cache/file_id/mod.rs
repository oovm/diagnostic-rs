use std::{
    collections::hash_map::DefaultHasher,
    fmt::{Debug, Display, Formatter},
    hash::{Hash, Hasher},
    path::PathBuf,
};

pub struct FileID {
    pub(crate) inner: String,
}

impl FileID {
    pub fn new<E>(source: E) -> Result<Self, <E as TryInto<Self>>::Error>
    where
        E: TryInto<Self>,
    {
        source.try_into()
    }
}

impl TryFrom<PathBuf> for FileID {
    type Error = std::io::Error;

    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        let path = value.to_string_lossy();
        if cfg!(windows) {
            let path = &path[4..path.len()];
            Ok(Self { inner: path.to_string() })
        }
        else {
            Ok(Self { inner: path.to_string() })
        }
    }
}

impl From<String> for FileID {
    fn from(value: String) -> Self {
        Self::from(value.as_str())
    }
}

impl From<&str> for FileID {
    fn from(value: &str) -> Self {
        let mut hasher = DefaultHasher::new();
        hasher.write_str(value);
        let id = hasher.finish();
        Self { inner: format!("<anonymous:{:0x}>", id) }
    }
}

impl Debug for FileID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("FileID").field(&self.inner).finish()
    }
}

impl Display for FileID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.inner)
    }
}

impl Hash for FileID {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(self.inner.as_bytes())
    }
}
