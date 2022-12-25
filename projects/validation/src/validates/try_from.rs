// use std::{
//     convert::Infallible,
//     num::ParseIntError,
//     ops::{ControlFlow, FromResidual, Try},
//     str::FromStr,
// };
//
// use crate::DiagnosticError;
//
// use super::*;
//
// #[test]
// fn test2() {
//     test().unwrap();
// }
//
// fn test() -> Validation<u32, DiagnosticError> {
//     let out = u32::from_str("a")?;
//     Validation::Success { value: out, diagnostics: vec![] }
// }
//
// impl<T, F, E> FromResidual<Result<T, E>> for Validation<T, F>
// where
//     F: From<E>,
// {
//     fn from_residual(residual: Result<T, E>) -> Self {
//         match residual {
//             Ok(o) => Validation::Success { value: o, diagnostics: vec![] },
//             Err(e) => Validation::Failure { fatal: e.into(), diagnostics: vec![] },
//         }
//     }
// }
//
// impl<T, F> FromResidual for Validation<T, F> {
//     fn from_residual(residual: <Self as Try>::Residual) -> Self {
//         todo!()
//     }
// }
//
// impl<T, F, E> Try for Validation<T, F> {
//     type Output = T;
//     type Residual = Validation<T, F>;
//
//     fn from_output(output: Self::Output) -> Self {
//         todo!()
//     }
//
//     fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
//         match self {
//             Validation::Success { value, diagnostics: _ } => ControlFlow::Continue(value),
//             Validation::Failure { fatal, diagnostics } => {
//                 ControlFlow::Break(Validation::Failure { fatal, diagnostics })
//             }
//         }
//     }
// }
//
// impl From<ParseIntError> for DiagnosticError {
//     fn from(_: ParseIntError) -> Self {
//         todo!()
//     }
// }
