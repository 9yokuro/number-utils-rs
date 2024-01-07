//! # number-utils
//! A calculation crate.
//!
//! # Examples
//! ```
//! use number_utils::NumberUtils;
//!
//! # fn main() {
//! let n: u32 = 6;
//!
//! assert_eq!(n.factorial(), 720);
//! assert_eq!(n.permutation(3), 120);
//! assert_eq!(n.combination(3), 20);
//!
//! assert_eq!(10_u32.factorial().digits(), 7);
//! # }
//! ```
#[cfg(feature = "num-bigint")]
pub mod bigint;
mod macros;
mod number_utils;
mod test;

pub use crate::{
    bigint::*,
    number_utils::{pascals_triangle, NumberUtils},
};
