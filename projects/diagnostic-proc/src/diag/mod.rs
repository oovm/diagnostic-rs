use proc_macro::Span;
use proc_macro2::{Literal, TokenStream, TokenTree};
use std::path::PathBuf;

use diagnostic::{
    term::{
        emit,
        termcolor::{ColorChoice, StandardStream},
        Config,
    },
    DiagnosticLevel, TextStorage,
};

pub fn my_macro(input: TokenStream) {
    let call_file = Span::call_site().source_file().path();

    match input.into_iter().next() {
        None => {}
        Some(TokenTree::Literal(s)) => {
            eprintln!("{:?}", s)
        }
        _ => {}
    }

    let mut store = TextStorage::default();
    // let id1 = store.file(call_file).unwrap();
    let id1 = store.anonymous(call_file.to_string_lossy(), include_str!("mod.rs"));
    let id2 = store.anonymous("projects\\diagnostic-proc\\src\\diag2\\mod.rs", include_str!("mod.rs"));
    let raw = diagnostic::Diagnostic::new(DiagnosticLevel::Error)
        .with_message("unknown builtin: 1")
        .with_primary(id1, 96..102, "unknown builtin")
        .with_secondary(id2, 96..102, "unknown builtin")
        .with_note("there is a builtin with a similar name: `NATURAL`");

    let writer = StandardStream::stderr(ColorChoice::Always);
    let config = Config::default();
    for diagnostic in &vec![raw.clone()] {
        emit(&mut writer.lock(), &config, &store, diagnostic).unwrap();
    }

    //  Err(input.span().error("there's a problem here..."))
}
