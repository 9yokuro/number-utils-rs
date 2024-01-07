//! # prime-number-utils
//! 'prime-number-utils' is a collection of utilities to generate prime numbers and to determine
//! whether a number is prime efficiently.
//!
//! # Examples
//! ```
//! use prime_number_utils::{GenPrime, SieveOfEratosthenes, baillie_psw};
//! # fn main() {
//! let mut sieve_of_eratosthenes = SieveOfEratosthenes::new();
//!
//! let mut primes = sieve_of_eratosthenes.gen_range(0..20);
//!
//! assert_eq!(&primes, &vec![2, 3, 5, 7, 11, 13, 17, 19]);
//!
//! assert!(primes.iter().all(|&n| baillie_psw(n)));
//! # }
//! ```
mod baillie_psw;
#[cfg(feature = "num-bigint")]
pub mod bigint;
mod bitwise_sieve;
mod gen_prime;
mod linear_sieve;
mod segmented_sieve;
mod sieve_of_atkin;
mod sieve_of_eratosthenes;
mod sieve_of_sundaram;
mod test;

pub use crate::{
    baillie_psw::*, bigint::*, bitwise_sieve::*, gen_prime::GenPrime, linear_sieve::*,
    segmented_sieve::*, sieve_of_atkin::*, sieve_of_eratosthenes::*, sieve_of_sundaram::*,
};
