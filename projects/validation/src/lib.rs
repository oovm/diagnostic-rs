#![no_std]
#![cfg_attr(feature = "nightly", feature(try_trait_v2))]
#![cfg_attr(feature = "nightly", feature(error_in_core))]
#![doc = include_str!("../readme.md")]

extern crate alloc;

mod validates;

pub use crate::validates::{convert::Validate, Validation};
