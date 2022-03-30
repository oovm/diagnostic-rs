//! Terminal back-end for emitting diagnostics.

use std::str::FromStr;

use crate::Diagnostic;
pub use termcolor;
use termcolor::{ColorChoice, WriteColor};

use crate::text_cache::TextStorage;

pub use self::config::{Chars, Config, DisplayStyle, Styles};
use self::{
    renderer::Renderer,
    views::{RichDiagnostic, ShortDiagnostic},
};

mod config;
mod renderer;
mod views;

/// A command line argument that configures the coloring of the output.
///
/// This can be used with command line argument parsers like [`clap`] or [`structopt`].
///
/// [`clap`]: https://crates.io/crates/clap
/// [`structopt`]: https://crates.io/crates/structopt
///
/// # Example
///
/// ```rust
/// use codespan_reporting::term::{termcolor::StandardStream, ColorArg};
/// use structopt::StructOpt;
///
/// #[derive(Debug, StructOpt)]
/// #[structopt(name = "groovey-app")]
/// pub struct Opts {
///     /// Configure coloring of output
///     #[structopt(
///         long = "color",
///         default_value = "auto",
///         possible_values = ColorArg::VARIANTS,
///         case_insensitive = true,
///     )]
///     pub color: ColorArg,
/// }
///
/// let opts = Opts::from_args();
/// let writer = StandardStream::stderr(opts.color.into());
/// ```
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ColorArg(pub ColorChoice);

impl ColorArg {
    /// Allowed values the argument.
    ///
    /// This is useful for generating documentation via [`clap`] or `structopt`'s
    /// `possible_values` configuration.
    ///
    /// [`clap`]: https://crates.io/crates/clap
    /// [`structopt`]: https://crates.io/crates/structopt
    pub const VARIANTS: &'static [&'static str] = &["auto", "always", "ansi", "never"];
}

impl FromStr for ColorArg {
    type Err = &'static str;

    fn from_str(src: &str) -> Result<ColorArg, &'static str> {
        match src {
            _ if src.eq_ignore_ascii_case("auto") => Ok(ColorArg(ColorChoice::Auto)),
            _ if src.eq_ignore_ascii_case("always") => Ok(ColorArg(ColorChoice::Always)),
            _ if src.eq_ignore_ascii_case("ansi") => Ok(ColorArg(ColorChoice::AlwaysAnsi)),
            _ if src.eq_ignore_ascii_case("never") => Ok(ColorArg(ColorChoice::Never)),
            _ => Err("valid values: auto, always, ansi, never"),
        }
    }
}

impl From<ColorArg> for ColorChoice {
    fn from(x: ColorArg) -> Self {
        x.0
    }
}

/// Emit a labels using the given writer, context, config, and errors.
///
/// The return value covers all error cases. These error case can arise if:
/// * a file was removed from the file database.
/// * a file was changed so that it is too small to have an index
/// * IO fails
pub fn emit<'files>(
    writer: &mut dyn WriteColor,
    config: &Config,
    files: &'files TextStorage,
    diagnostic: &Diagnostic,
) -> Result<(), super::errors::DiagnosticError> {
    let mut renderer = Renderer::new(writer, config);
    match config.display_style {
        DisplayStyle::Rich => RichDiagnostic::new(diagnostic, config).render(files, &mut renderer),
        DisplayStyle::Medium => ShortDiagnostic::new(diagnostic, true).render(files, &mut renderer),
        DisplayStyle::Short => ShortDiagnostic::new(diagnostic, false).render(files, &mut renderer),
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        diagnostic::{Diagnostic, Label},
        term::{emit, Config},
        text_cache::TextStorage,
    };

    #[test]
    fn unsized_emit() {
        let mut files = TextStorage::default();
        files.anonymous("test", "texttext");
        let mut writer = termcolor::NoColor::new(Vec::<u8>::new());
        let diagnostic = Diagnostic::bug().with_labels(vec![Label::primary("id".to_string(), 0..0)]);

        emit(&mut writer, &Config::default(), &files, &diagnostic).unwrap();
    }
}
