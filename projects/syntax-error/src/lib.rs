use std::{
    error::Error,
    fmt::{Display, Formatter},
    num::{ParseFloatError, ParseIntError},
    ops::Range,
    str::ParseBoolError,
};
use syntax_error::{Color, Label, Report, ReportKind};

use crate::{FileID, FileSpan, ValkyrieError, ValkyrieErrorType};

mod errors;
mod for_std;
mod third_party;

#[cfg(feature = "dashu")]
mod for_dashu;
#[cfg(feature = "json5")]
mod for_json5;
#[cfg(feature = "num")]
mod for_num;
#[cfg(feature = "peginator")]
mod for_peginator;
mod for_std;
#[cfg(feature = "toml")]
mod for_toml;

#[cfg(feature = "pratt")]
mod for_pratt;

#[derive(Clone, Debug)]
pub struct SyntaxError {
    pub info: String,
    pub span: FileSpan,
    pub level: ReportKind,
}

impl Error for SyntaxError {}

impl Display for SyntaxError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str(&self.info)
    }
}

impl SyntaxError {
    pub fn new(file: FileID) -> Self {
        Self { span: file.with_range(0..0), info: String::new(), level: ReportKind::Error }
    }
    pub fn with_message<T>(mut self, info: T) -> Self
        where
            T: ToString,
    {
        self.info = info.to_string();
        self
    }
    pub fn with_range(mut self, range: Range<usize>) -> Self {
        self.span.set_range(range);
        self
    }
    pub fn with_span(mut self, span: FileSpan) -> Self {
        self.span = span;
        self
    }
    pub fn with_level(mut self, level: ReportKind) -> Self {
        self.level = level;
        self
    }
    pub fn as_report(&self) -> Report {
        let mut report = Report::new(self.level, self.span.get_file(), self.span.get_range().start);
        report.set_message(self.to_string());
        let label = Label::new(self.span);
        report.add_label(label);
        report.finish()
    }
}

// macro_rules! wrap_parse_error {
//     ($($type:ty),*) => {
//         $(
//             impl From<$type> for ValkyrieError {
//                 fn from(value: $type) -> Self {
//                     SyntaxError::new(value.to_string()).into()
//                 }
//             }
//         )*
//     };
// }
//
// wrap_parse_error!(ParseIntError, ParseFloatError, ParseBoolError, ParseCharError, url::ParseError);
//
// #[cfg(feature = "num")]
// wrap_parse_error!(num::bigint::ParseBigIntError);
//
// #[cfg(feature = "dashu")]
// wrap_parse_error!(dashu::base::ParseError);
