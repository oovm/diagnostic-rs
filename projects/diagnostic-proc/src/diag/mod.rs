use proc_macro2::TokenStream;
use proc_macro2_diagnostics::{Diagnostic, Level};

use diagnostic::DiagnosticLevel;
use proc_macro_error::abort;

fn level(raw: DiagnosticLevel) -> Level {
    match raw {
        DiagnosticLevel::Help => Level::Help,
        DiagnosticLevel::Note => Level::Help,
        DiagnosticLevel::Warning => Level::Warning,
        DiagnosticLevel::Error => Level::Error,
        DiagnosticLevel::Bug => Level::Error,
    }
}

pub fn my_macro(input: TokenStream) -> Result<TokenStream, Diagnostic> {
    let raw = diagnostic::Diagnostic::new(DiagnosticLevel::Error)
        .with_message("unknown builtin: `NATRAL`")
        .with_primary("GGG", 96..102, "unknown builtin")
        .with_note("there is a builtin with a similar name: `NATURAL`");
    let diagnostic::Diagnostic { severity, code, message, labels, notes } = raw;

    abort!(
        span, message; // <--- attachments start with `;` (semicolon)

        help = "format {} {}", "arg1", "arg2"; // <--- every attachment ends with `;`,
                                               //      maybe except the last one

        note = "to_string"; // <--- one arg uses `.to_string()` instead of `format!()`

        yay = "I see what {} did here", "you"; // <--- "help =" and "hint =" are mapped
                                               // to Diagnostic::help,
                                               // anything else is Diagnostic::note

    );

    Err(diag)

    //  Err(input.span().error("there's a problem here..."))
}
