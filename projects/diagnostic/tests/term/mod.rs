use super::*;

#[test]
fn unsized_emit() {
    let mut files = TextStorage::default();
    let id1 = files.anonymous("text text");
    let mut writer = termcolor::NoColor::new(Vec::<u8>::new());
    let diagnostic = Diagnostic::new(DiagnosticLevel::Fatal).with_primary(&id1, 0..0, "primear");

    emit(&mut writer, &TerminalConfig::default(), &files, &diagnostic).unwrap();
}
