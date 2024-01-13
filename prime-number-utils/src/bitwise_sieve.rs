use crate::{impl_gen_range, GenPrime};

#[derive(Default, Debug, Clone, Copy, Eq, PartialEq, Hash)]
/// Generates prime numbers using bitwise.
///
/// # Examples
/// ```
/// # use prime_number_utils::{GenPrime, BitwiseSieve};
/// # fn main() {
/// let mut bitwise_sieve = BitwiseSieve::new();
/// assert_eq!(bitwise_sieve.gen_range(0..10), vec![2, 3, 5, 7]);
/// # }
/// ```
pub struct BitwiseSieve {
    max: usize,
}

impl BitwiseSieve {
    /// Creates a new sieve.
    pub fn new() -> Self {
        Self::default()
    }
}

#[inline]
fn not_prime(sieve: &[usize], n: usize) -> usize {
    sieve[n / 64] & (1 << ((n >> 1) & 31))
}

impl GenPrime for BitwiseSieve {
    fn gen(&mut self) -> Vec<usize> {
        if self.max < 2 {
            return vec![];
        }
        let mut sieve = vec![0; self.max / 64 + 1];
        for i in (3..=(self.max as f64).sqrt() as usize).step_by(2) {
            if not_prime(&sieve, i) == 0 {
                let k = i << 1;
                for j in (i.pow(2)..self.max).step_by(k) {
                    sieve[j / 64] |= 1 << ((j >> 1) & 31);
                }
            }
        }
        let mut primes = vec![2];
        for i in (3..=self.max).step_by(2) {
            if not_prime(&sieve, i) == 0 {
                primes.push(i);
            }
        }
        primes
    }

    impl_gen_range!();
}
