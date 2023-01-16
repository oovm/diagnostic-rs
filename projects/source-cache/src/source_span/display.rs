use super::*;

impl Debug for SourceSpan {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FileSpan").field("start", &self.start).field("end", &self.end).field("file", &self.file).finish()
    }
}

impl Display for SourceSpan {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "FileSpan(0x{:X}, {}..{})", self.file.hash, self.start, self.end)
    }
}
