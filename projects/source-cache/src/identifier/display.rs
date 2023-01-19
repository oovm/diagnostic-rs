use super::*;

impl Display for SourcePath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Anonymous => f.write_str("<anonymous>"),
            Self::Snippet(s) => f.write_str(s),
            Self::Local(s) => match Url::from_file_path(s) {
                Ok(s) => f.write_str(s.as_str()),
                Err(_) => f.write_str(&s.to_string_lossy()),
            },
            Self::Remote(s) => f.write_str(s.as_str()),
        }
    }
}

impl Debug for SourceID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "FileID(0x{:X})", self.hash)
    }
}

impl Display for SourceID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}
