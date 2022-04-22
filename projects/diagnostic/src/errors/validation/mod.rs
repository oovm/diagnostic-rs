use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
    mem::take,
};

use serde::{Deserialize, Serialize};

use crate::{Diagnostic, DiagnosticLevel, Failure, Success};

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
            Success { diagnostics, .. } => diagnostics.is_empty(),
            Failure { .. } => false,
        }
    }
    /// Returns the contained [`Validation::Success`] value, consuming the `self` value.
    pub fn unwrap(self) -> T
    where
        E: Display,
    {
        match self {
            Success { value, diagnostics: _ } => value,
            Failure { fatal, diagnostics: _ } => panic!("{}", fatal),
        }
    }
    /// Returns the contained [`Validation::Success`] value, consuming the `self` value.
    pub fn unwrap_or_default(self) -> T
    where
        T: Default,
    {
        match self {
            Success { value, .. } => value,
            Failure { .. } => T::default(),
        }
    }
    /// Maps a `Result<T, E>` to `Result<U, E>` by applying a function to a
    /// contained [`Ok`] value, leaving an [`Err`] value untouched.
    ///
    /// This function can be used to compose the results of two functions.
    ///
    /// # Examples
    ///
    /// Print the numbers on each line of a string multiplied by two.
    ///
    /// ```
    /// let line = "1\n2\n3\n4\n";
    ///
    /// for num in line.lines() {
    ///     match num.parse::<i32>().map(|i| i * 2) {
    ///         Ok(n) => println!("{n}"),
    ///         Err(..) => {}
    ///     }
    /// }
    /// ```
    pub fn map<U, F>(self, f: F) -> Validation<U, E>
    where
        F: FnOnce(T) -> U,
    {
        // let a = Ok(1);
        // a.map_or();

        match self {
            Success { value, diagnostics } => Success { value: f(value), diagnostics },
            Failure { fatal, diagnostics } => Failure { fatal, diagnostics },
        }
    }
    /// Calls a closure on each element of an iterator.
    ///
    /// This is equivalent to using a [`for`] loop on the iterator, although
    /// `break` and `continue` are not possible from a closure. It's generally
    /// more idiomatic to use a `for` loop, but `for_each` may be more legible
    /// when processing items at the end of longer iterator chains. In some
    /// cases `for_each` may also be faster than a loop, because it will use
    /// internal iteration on adapters like `Chain`.
    ///
    /// [`for`]: ../../book/ch03-05-control-flow.html#looping-through-a-collection-with-for
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use std::sync::mpsc::channel;
    ///
    /// let (tx, rx) = channel();
    /// (0..5).map(|x| x * 2 + 1).for_each(move |x| tx.send(x).unwrap());
    ///
    /// let v: Vec<_> = rx.iter().collect();
    /// assert_eq!(v, vec![1, 3, 5, 7, 9]);
    /// ```
    ///
    /// For such a small example, a `for` loop may be cleaner, but `for_each`
    /// might be preferable to keep a functional style with longer iterators:
    ///
    /// ```
    /// (0..5)
    ///     .flat_map(|x| x * 100..x * 110)
    ///     .enumerate()
    ///     .filter(|&(i, x)| (i + x) % 3 == 0)
    ///     .for_each(|(i, x)| println!("{i}:{x}"));
    /// ```
    pub fn take_diagnostics<U>(&mut self, out: &mut Vec<U>)
    where
        U: From<E>,
    {
        let diagnostics = match self {
            Success { diagnostics, .. } => take(diagnostics),
            Failure { diagnostics, .. } => take(diagnostics),
        };
        out.extend(diagnostics.into_iter().map(U::from))
    }
    ///
    pub fn omit(self) {}
    /// Returns the provided default (if [`Err`]), or
    /// applies a function to the contained value (if [`Ok`]),
    ///
    /// Arguments passed to `map_or` are eagerly evaluated; if you are passing
    /// the result of a function call, it is recommended to use [`map_or_else`],
    /// which is lazily evaluated.
    ///
    /// [`map_or_else`]: Result::map_or_else
    ///
    /// # Examples
    ///
    /// ```
    /// let x: Result<_, &str> = Ok("foo");
    /// assert_eq!(x.map_or(42, |v| v.len()), 3);
    ///
    /// let x: Result<&str, _> = Err("bar");
    /// assert_eq!(x.map_or(42, |v| v.len()), 42);
    /// ```
    pub fn map_or<U, F>(self, default: U, f: F) -> U
    where
        F: FnOnce(T) -> U,
    {
        match self {
            Success { value, .. } => f(value),
            Failure { .. } => default,
        }
    }
}

impl<T, E> Validation<T, E> {
    /// A fatal error occurred
    pub fn fatal<I>(error: I, diagnostics: Vec<E>) -> Self
    where
        I: Into<E>,
    {
        Failure { fatal: error.into(), diagnostics }
    }
    /// Turn a [`Validation`] into a [`Result`]
    pub fn result<F>(self, f: F) -> Result<T, E>
    where
        F: FnMut(E) -> (),
    {
        match self {
            Success { value, diagnostics } => {
                diagnostics.into_iter().for_each(f);
                Ok(value)
            }
            Failure { fatal, diagnostics } => {
                diagnostics.into_iter().for_each(f);
                Err(fatal)
            }
        }
    }
    /// Turn a [`Validation`] into a [`Option`]
    pub fn option<F>(self, f: F) -> Option<T>
    where
        F: FnMut(E) -> (),
    {
        match self {
            Success { value, diagnostics } => {
                diagnostics.into_iter().for_each(f);
                Some(value)
            }
            Failure { fatal: _, diagnostics } => {
                diagnostics.into_iter().for_each(f);
                None
            }
        }
    }
    /// Collect all diagnostics, with final fatal error if exists
    pub fn collect_diagnostics<'s>(&'s self) -> Vec<Diagnostic>
    where
        Diagnostic: From<&'s E>,
    {
        let mut out = vec![];
        match self {
            Success { value: _, diagnostics } => {
                for diagnostic in diagnostics {
                    out.push(diagnostic.into())
                }
            }
            Failure { fatal, diagnostics } => {
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
