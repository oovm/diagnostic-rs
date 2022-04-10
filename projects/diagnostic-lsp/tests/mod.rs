use lsp_types::Position;

use diagnostic::{Location, TextStorage};
use diagnostic_lsp::{byte_index_to_position, position_to_byte_index};

const TEST_TEXT: &str = r#"
let test = 2
let test1 = ""
test
"#;
#[test]
fn position() {
    let mut files = TextStorage::default();
    let file_id = files.anonymous(TEST_TEXT);
    let pos = position_to_byte_index(&files, &file_id, &Position { line: 3, character: 2 }).unwrap();
    assert_eq!(
        Location {
            // One-based
            line_number: 3 + 1,
            column_number: 2 + 1,
        },
        files.location(&file_id, pos).unwrap()
    );
}

// The protocol specifies that each `character` in position is a UTF-16 character.
// This means that `å` and `ä` here counts as 1 while `𐐀` counts as 2.
const UNICODE: &str = "åä t𐐀b";

#[test]
fn unicode_get_byte_index() {
    let mut files = TextStorage::default();
    let file_id = files.anonymous(UNICODE);

    let result = position_to_byte_index(&files, &file_id, &Position { line: 0, character: 3 });
    assert_eq!(result.unwrap(), 5);

    let result = position_to_byte_index(&files, &file_id, &Position { line: 0, character: 6 });
    assert_eq!(result.unwrap(), 10);
}

#[test]
fn unicode_get_position() {
    let mut files = TextStorage::default();
    let file_id = files.anonymous(UNICODE.to_string());
    let file_id2 = files.anonymous("\n".to_string() + UNICODE);

    let result = byte_index_to_position(&files, &file_id, 5);
    assert_eq!(result.unwrap(), Position { line: 0, character: 3 });

    let result = byte_index_to_position(&files, &file_id, 10);
    assert_eq!(result.unwrap(), Position { line: 0, character: 6 });

    let result = byte_index_to_position(&files, &file_id2, 11);
    assert_eq!(result.unwrap(), Position { line: 1, character: 6 });
}
