use super::*;

impl Display for SourceText {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for c in self.lines() {
            f.write_str(&c.text)?;
            f.write_char('\n')?;
        }
        Ok(())
    }
}

impl<S: AsRef<str>> From<S> for SourceText {
    /// Generate a [`SourceText`] from the given [`str`].
    ///
    /// Note that this function can be expensive for long strings. Use an implementor of [`Cache`] where possible.
    fn from(s: S) -> Self {
        let mut offset = 0;
        // (Last line, last line ends with CR)
        let mut last_line: Option<(SourceLine, bool)> = None;
        let mut lines: Vec<SourceLine> = s
            .as_ref()
            .split_inclusive([
                '\r',       // Carriage return
                '\n',       // Line feed
                '\x0B',     // Vertical tab
                '\x0C',     // Form feed
                '\u{0085}', // Next line
                '\u{2028}', // Line separator
                '\u{2029}', // Paragraph separator
            ])
            .flat_map(|line| {
                // Returns last line and set `last_line` to current `line`
                // A hack that makes `flat_map` deals with consecutive lines

                if let Some((last, ends_with_cr)) = last_line.as_mut() {
                    if *ends_with_cr && line == "\n" {
                        last.length += 1;
                        offset += 1;
                        return replace(&mut last_line, None).map(|(l, _)| l);
                    }
                }

                let len = line.len();
                let ends_with_cr = line.ends_with('\r');
                let line = SourceLine { offset, length: len as u32, text: line.trim_end().to_owned() };
                offset += line.length;
                replace(&mut last_line, Some((line, ends_with_cr))).map(|(l, _)| l)
            })
            .collect();

        if let Some((l, _)) = last_line {
            lines.push(l);
        }

        Self { path: SourcePath::Anonymous, lines, length: offset }
    }
}
