#![cfg_attr(feature = "nightly", feature(try_trait_v2))]
#![doc = include_str!("../readme.md")]

mod validates;

pub use self::validates::Validation;
