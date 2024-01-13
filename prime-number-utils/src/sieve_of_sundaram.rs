use crate::{impl_gen_range, GenPrime};

#[derive(Default, Debug, Clone, Copy, Eq, PartialEq, Hash)]
/// Implementation of the Sieve of Sundaram.
///
/// # Examples
/// ```
/// # use prime_number_utils::{GenPrime, SieveOfSundaram};
/// # fn main() {
/// let mut sieve_of_sundaram = SieveOfSundaram::new();
/// assert_eq!(sieve_of_sundaram.gen_range(0..10), vec![2, 3, 5, 7]);
/// # }
/// ```
pub struct SieveOfSundaram {
    max: usize,
}

impl SieveOfSundaram {
    /// Creates a new sieve.
    pub fn new() -> Self {
        Self::default()
    }
}

impl GenPrime for SieveOfSundaram {
    fn gen(&mut self) -> Vec<usize> {
        if self.max < 2 {
            return vec![];
        }
        let limit = (self.max - 1) / 2;
        let mut sieve = vec![0; limit + 1];
        for i in 1..=limit {
            let mut j = i;
            while i + j + 2 * i * j <= limit {
                sieve[i + j + 2 * i * j] = 1;
                j += 1;
            }
        }
        let mut primes = vec![2];
        for (i, &item) in sieve.iter().enumerate().take(limit + 1).skip(1) {
            if item == 0 {
                primes.push(2 * i + 1);
            }
        }
        primes
    }

    impl_gen_range!();
}
