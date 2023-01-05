use super::*;

#[test]
fn main() {
    let mut files = FileCache::default();
    let sample = files.load_text(include_str!("sample.tao"), "sample.tao");

    Diagnostic::new(ReportKind::Blame)
        .with_location(sample, Some(12))
        .with_message("Incompatible types")
        .with_code(12)
        .with_label(Label::new(sample.with_range(32..33)).with_message("This is of type Nat"))
        .with_label(Label::new(sample.with_range(42..45)).with_message("This is of type Str"))
        .finish()
        .print(files)
        .unwrap();
}
