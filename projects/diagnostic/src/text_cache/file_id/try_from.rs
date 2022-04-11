use super::*;

impl From<&FileID> for FileID {
    fn from(value: &FileID) -> Self {
        value.clone()
    }
}

impl TryFrom<&Path> for FileID {
    type Error = std::io::Error;

    fn try_from(value: &Path) -> Result<Self, Self::Error> {
        let path = value.canonicalize()?;
        let path = path.to_string_lossy();
        if cfg!(windows) {
            let path = &path[4..path.len()];
            Ok(Self { inner: path.to_string() })
        }
        else {
            Ok(Self { inner: path.to_string() })
        }
    }
}

impl TryFrom<&PathBuf> for FileID {
    type Error = std::io::Error;
    fn try_from(value: &PathBuf) -> Result<Self, Self::Error> {
        Self::try_from(value.as_path())
    }
}

impl TryFrom<PathBuf> for FileID {
    type Error = std::io::Error;
    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        Self::try_from(value.as_path())
    }
}

impl From<String> for FileID {
    fn from(value: String) -> Self {
        Self::from(value.as_str())
    }
}

impl From<&String> for FileID {
    fn from(value: &String) -> Self {
        Self::from(value.as_str())
    }
}

impl From<&str> for FileID {
    fn from(value: &str) -> Self {
        let mut hasher = DefaultHasher::new();
        hasher.write(value.as_bytes());
        let id = hasher.finish();
        Self { inner: format!("<anonymous:{:0x}>", id) }
    }
}
