use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
};

use serde::{Deserialize, Serialize};

use crate::{Diagnostic, DiagnosticLevel};

#[cfg(feature = "nightly")]
mod try_from;

/// A validation result with multiple diagnostics.
#[derive(Debug, Serialize, Deserialize)]
pub enum Validation<T, E> {
    /// Verification process complete
    Success {
        /// The final product after successful verification
        value: T,
        /// Some diagnostics that does not stop the analysis
        diagnostics: Vec<E>,
    },
    /// Verification process interrupted
    Failure {
        /// A fatal problem prevents the analysis from continuing
        fatal: E,
        /// Some diagnostics that does not stop the analysis
        diagnostics: Vec<E>,
    },
}

impl<T, E> Display for Validation<T, E>
where
    T: Debug,
    E: Error,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl<T, E> Error for Validation<T, E>
where
    T: Debug,
    E: Error,
{
}

impl<T, E> Validation<T, E> {
    /// Check if the validate result is success
    pub fn is_success(&self) -> bool {
        matches!(self, Validation::Success { .. })
    }
    /// Check if the validate result is failure
    pub fn is_failure(&self) -> bool {
        matches!(self, Validation::Failure { .. })
    }
    /// Check if the validate result has no problem
    pub fn no_problem(&self) -> bool {
        match self {
            Validation::Success { diagnostics, .. } => diagnostics.is_empty(),
            Validation::Failure { .. } => false,
        }
    }
    /// Returns the contained [`Validation::Success`] value, consuming the `self` value.
    pub fn unwrap(self) -> T
    where
        E: Display,
    {
        match self {
            Validation::Success { value, diagnostics: _ } => value,
            Validation::Failure { fatal, diagnostics: _ } => panic!("{}", fatal),
        }
    }
    /// Returns the contained [`Validation::Success`] value, consuming the `self` value.
    pub fn unwrap_or_default(self) -> T
    where
        T: Default,
    {
        match self {
            Validation::Success { value, .. } => value,
            Validation::Failure { .. } => T::default(),
        }
    }
}

impl<T, E> Validation<T, E> {
    /// A fatal error occurred
    pub fn fatal<I>(error: I, diagnostics: Vec<E>) -> Self
    where
        I: Into<E>,
    {
        Validation::Failure { fatal: error.into(), diagnostics }
    }
    /// Collect all diagnostics, with final fatal error if exists
    pub fn collect_diagnostics<'s>(&'s self) -> Vec<Diagnostic>
    where
        Diagnostic: From<&'s E>,
    {
        let mut out = vec![];
        match self {
            Validation::Success { value: _, diagnostics } => {
                for diagnostic in diagnostics {
                    out.push(diagnostic.into())
                }
            }
            Validation::Failure { fatal, diagnostics } => {
                for diagnostic in diagnostics {
                    out.push(diagnostic.into())
                }
                let mut last = Diagnostic::from(fatal);
                last.severity = DiagnosticLevel::Fatal;
                out.push(last)
            }
        }
        out
    }
}
