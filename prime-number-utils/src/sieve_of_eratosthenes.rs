use crate::{impl_gen_range, GenPrime};

#[derive(Default, Debug, Clone, Copy, Eq, PartialEq, Hash)]
/// Implementation of the Sieve of Eratosthenes.
///
/// # Examples
/// ```
/// # use prime_number_utils::{SieveOfEratosthenes, GenPrime};
/// # fn main() {
/// let mut sieve_of_eratosthenes = SieveOfEratosthenes::new();
/// assert_eq!(sieve_of_eratosthenes.gen_range(0..10), vec![2, 3, 5, 7]);
/// # }
/// ```
pub struct SieveOfEratosthenes {
    max: usize,
}

impl SieveOfEratosthenes {
    /// Creates a new sieve.
    pub fn new() -> Self {
        Self::default()
    }
}

impl GenPrime for SieveOfEratosthenes {
    fn gen(&mut self) -> Vec<usize> {
        if self.max < 2 {
            return vec![];
        }
        let mut sieve = vec![0; self.max / 2];
        let mut i: usize = 3;
        while i.pow(2) < self.max {
            if sieve[i / 2] == 0 {
                for j in (i.pow(2)..self.max).step_by(i * 2) {
                    sieve[j / 2] = 1;
                }
            }
            i += 2;
        }
        let mut primes = vec![2];
        for i in (3..self.max).step_by(2) {
            if sieve[i / 2] == 0 {
                primes.push(i);
            }
        }
        primes
    }

    impl_gen_range!();
}
