use std::{error::Error, fmt::Debug};

#[derive(Debug)]
pub enum Validation<T, E> {
    Success { value: T, diagnostics: Vec<E> },
    Failure { fatal: E, diagnostics: Vec<E> },
}

impl<T, E> Validation<T, E> {
    pub fn no_problem(&self) -> bool {
        match self {
            Validation::Success { diagnostics, .. } => diagnostics.is_empty(),
            Validation::Failure { .. } => false,
        }
    }
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
