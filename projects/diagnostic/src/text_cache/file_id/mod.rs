use super::*;

mod ser_der;
mod try_from;

#[derive(Clone, Default, Eq)]
pub struct FileID(Box<str>);

impl AsRef<str> for FileID {
    fn as_ref(&self) -> &str {
        &self.0
    }
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
        f.debug_tuple("FileID").field(&self.0).finish()
    }
}

impl Display for FileID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl Hash for FileID {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(self.0.as_bytes())
    }
}

impl PartialEq for FileID {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl PartialOrd for FileID {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for FileID {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}
