use proc_macro2::TokenStream;

use diagnostic::{
    term::{
        emit,
        termcolor::{ColorChoice, StandardStream},
        Config,
    },
    DiagnosticLevel, TextStorage,
};

pub fn my_macro(input: TokenStream) {
    let mut store = TextStorage::default();
    store.anonymous("projects\\diagnostic-proc\\src\\diag\\mod.rs", include_str!("mod.rs"));
    store.anonymous("projects/diagnostic-proc/src/diag2/mod.rs", include_str!("mod.rs"));
    let raw = diagnostic::Diagnostic::new(DiagnosticLevel::Error)
        .with_message("unknown builtin: 1")
        .with_primary("projects\\diagnostic-proc\\src\\diag\\mod.rs", 96..102, "unknown builtin")
        .with_secondary("projects/diagnostic-proc/src/diag2/mod.rs", 96..102, "unknown builtin")
        .with_note("there is a builtin with a similar name: `NATURAL`");

    let writer = StandardStream::stderr(ColorChoice::Always);
    let config = Config::default();
    for diagnostic in &vec![raw.clone()] {
        emit(&mut writer.lock(), &config, &store, diagnostic).unwrap();
    }

    //  Err(input.span().error("there's a problem here..."))
}
