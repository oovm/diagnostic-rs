use super::*;

impl Display for SourcePath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SourcePath::Anonymous => f.write_str("<anonymous>"),
            SourcePath::Snippet(s) => f.write_str(s),
            #[cfg(not(feature = "url"))]
            SourcePath::Path(s) => write!(f, "{:?}", s),
            #[cfg(feature = "url")]
            SourcePath::Local(s) => f.write_str(s.as_str()),
            #[cfg(feature = "url")]
            SourcePath::Remote(s) => f.write_str(s.as_str()),
        }
    }
}

impl Display for Source {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for c in self.lines() {
            f.write_str(&c.chars)?;
            f.write_char('\n')?;
        }
        Ok(())
    }
}
