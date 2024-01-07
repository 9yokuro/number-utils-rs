use crate::{impl_gen_range, BitwiseSieve, GenPrime};
use std::ops::{Bound::*, RangeBounds};

/// Implementation of the Segmented sieve.
///
/// # Examples
/// ```
/// # use prime_number_utils::{GenPrime, SegmentedSieve, BitwiseSieve};
/// # fn main() {
/// let mut segmented_sieve = SegmentedSieve::new(BitwiseSieve::new());
/// assert_eq!(segmented_sieve.gen_range(0..10), vec![2, 3, 5, 7]);
/// # }
/// ```
#[derive(Default, Debug, Clone, Eq, PartialEq, Hash)]
pub struct SegmentedSieve<G: GenPrime = BitwiseSieve> {
    max: usize,
    min: usize,
    gen_prime: G,
}

impl<G: GenPrime> SegmentedSieve<G> {
    /// Creates a new sieve.
    pub fn new(gen_prime: G) -> Self {
        Self {
            max: 0,
            min: 0,
            gen_prime,
        }
    }
}

impl<G: GenPrime> GenPrime for SegmentedSieve<G> {
    fn gen(&mut self) -> Vec<usize> {
        if self.max < 2 {
            return vec![];
        }
        let n = (self.max as f64).sqrt() as usize;
        let mut primes = self.gen_prime.gen_range(0..n);
        let mut low = n;
        let mut high = 2 * n;
        let mut result = vec![];
        while low < self.max {
            if high >= self.max {
                high = self.max;
            }
            let mut sieve = vec![0; n];
            for i in &primes {
                let mut lolim = low / i * i;
                if lolim < low {
                    lolim += i;
                }
                for j in (lolim..high).step_by(*i) {
                    sieve[j - low] = 1;
                }
            }
            for i in low..high {
                if sieve[i - low] == 0 {
                    result.push(i);
                }
            }
            low += n;
            high += n;
        }
        primes.append(&mut result);
        primes
    }

    impl_gen_range!();
}
