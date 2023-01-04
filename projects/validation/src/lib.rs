#![cfg_attr(feature = "nightly", feature(try_trait_v2))]
#![doc = include_str!("../readme.md")]

mod validates;

pub use crate::validates::{convert::Validate, Validation};
