use super::*;

impl Display for Source {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for c in self.lines() {
            f.write_str(&c.chars)?;
            f.write_char('\n')?;
        }
        Ok(())
    }
}
