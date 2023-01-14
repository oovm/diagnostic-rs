use std::borrow::Cow;
use url::Url;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum SourcePath {
    Anonymous,
    Snippet(Cow<'static, str>),
    Local(Url),
    Remote(Url),
}

impl SourcePath {
    /// Calculate the file from the source
    pub fn get_file_id(&self) -> FileID {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        unsafe { FileID::new(hasher.finish()) }
    }
    pub fn get_local(&self) -> Option<PathBuf> {
        match self {
            SourcePath::Anonymous => {}
            SourcePath::Snippet(_) => {}
            SourcePath::Local(s) => s.to_file_path().ok(),
            _ => None,
        }
    }
}

impl Default for SourcePath {
    fn default() -> Self {
        Self::Anonymous
    }
}
