mod serder;
mod try_from;
use super::*;

#[derive(Clone, Eq)]
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

impl PartialEq for FileID {
    fn eq(&self, other: &Self) -> bool {
        self.inner.eq(&other.inner)
    }
}

impl PartialOrd for FileID {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.inner.partial_cmp(&other.inner)
    }
}

impl Ord for FileID {
    fn cmp(&self, other: &Self) -> Ordering {
        self.inner.cmp(&other.inner)
    }
}
