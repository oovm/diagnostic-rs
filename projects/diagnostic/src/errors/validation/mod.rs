use std::{error::Error, fmt::Debug};

use crate::{Diagnostic, DiagnosticLevel};

#[derive(Debug)]
pub enum Validation<T, E> {
    Success { value: T, diagnostics: Vec<E> },
    Failure { fatal: E, diagnostics: Vec<E> },
}

impl<T, E> Validation<T, E> {
    pub fn unwrap(self) -> T
    where
        E: Error,
    {
        match self {
            Validation::Success { value, diagnostics: _ } => value,
            Validation::Failure { fatal, diagnostics: _ } => panic!("{}", fatal),
        }
    }
    pub fn is_success(&self) -> bool {
        matches!(self, Validation::Success { .. })
    }
    pub fn is_failure(&self) -> bool {
        matches!(self, Validation::Failure { .. })
    }
}

impl<T, E> Validation<T, E> {
    pub fn no_problem(&self) -> bool {
        match self {
            Validation::Success { diagnostics, .. } => diagnostics.is_empty(),
            Validation::Failure { .. } => false,
        }
    }
    pub fn collect_diagnostics<'s>(&'s self) -> Vec<Diagnostic>
    where
        E: Error,
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
