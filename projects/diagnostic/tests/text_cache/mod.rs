use super::*;

const TEST_SOURCE: &str = "foo\nbar\r\n\nbaz";

#[test]
fn line_starts() {
    let file = TextCache::anonymous(TEST_SOURCE.to_string());
    assert_eq!(
        file.line_starts,
        [
            0,  // "foo\n"
            4,  // "bar\r\n"
            9,  // ""
            10, // "baz"
        ],
    );
}

#[test]
fn line_span_sources() {
    let file = TextCache::anonymous(TEST_SOURCE.to_string());
    let line_sources = (0..4)
        .map(|line| {
            let line_range = file.line_range(line).unwrap();
            &file.text[line_range]
        })
        .collect::<Vec<_>>();
    assert_eq!(line_sources, ["foo\n", "bar\r\n", "\n", "baz"]);
}
