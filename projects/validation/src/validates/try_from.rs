use super::*;
use std::convert::Infallible;

use std::ops::{ControlFlow, FromResidual, Try};

impl<T, F, E> FromResidual<Result<Infallible, E>> for Validation<T, F>
where
    F: From<E>,
{
    fn from_residual(residual: Result<Infallible, E>) -> Self {
        match residual {
            Ok(o) => unreachable!(),
            Err(e) => Failure { fatal: e.into(), diagnostics: vec![] },
        }
    }
}

impl<T, F> FromResidual for Validation<T, F> {
    fn from_residual(residual: <Self as Try>::Residual) -> Self {
        todo!()
    }
}

impl<T, F> Try for Validation<T, F> {
    type Output = T;
    type Residual = Validation<T, F>;

    fn from_output(output: Self::Output) -> Self {
        todo!()
    }

    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match self {
            Validation::Success { value, diagnostics: _ } => ControlFlow::Continue(value),
            Validation::Failure { fatal, diagnostics } => ControlFlow::Break(Validation::Failure { fatal, diagnostics }),
        }
    }
}
