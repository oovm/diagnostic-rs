use super::*;

#[test]
fn source_from() {
    fn test(lines: Vec<&str>) {
        let source: String = lines.iter().map(|s| *s).collect();
        let source = SourceText::from(source);

        assert_eq!(source.lines().len(), lines.len());

        let mut offset = 0;
        for (source_line, raw_line) in zip(source.lines().into_iter(), lines.into_iter()) {
            assert_eq!(source_line.offset as usize, offset);
            assert_eq!(source_line.length as usize, raw_line.len());
            assert_eq!(source_line.text, raw_line.trim_end());
            offset += source_line.length as usize;
        }

        assert_eq!(source.get_length(), offset);
    }

    test(vec![]); // Empty string

    test(vec!["Single line"]);
    test(vec!["Single line with LF\n"]);
    test(vec!["Single line with CRLF\r\n"]);

    test(vec!["Two\r\n", "lines\n"]);
    test(vec!["Some\n", "more\r\n", "lines"]);
    test(vec!["\n", "\r\n", "\n", "Empty Lines"]);

    test(vec!["Trailing spaces  \n", "are trimmed\t"]);

    // Line endings other than LF or CRLF
    test(vec!["CR\r", "VT\x0B", "FF\x0C", "NEL\u{0085}", "LS\u{2028}", "PS\u{2029}"]);
}
