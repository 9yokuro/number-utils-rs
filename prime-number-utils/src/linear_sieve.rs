use crate::{impl_gen_range, GenPrime};

#[derive(Default, Debug, Clone, Eq, PartialEq, Hash)]
/// Implementation of the Linear Sieve.
///
/// # Examples
/// ```
/// # use prime_number_utils::{GenPrime, LinearSieve};
/// # fn main() {
/// let mut linear_sieve = LinearSieve::new();
/// assert_eq!(linear_sieve.gen_range(0..10), vec![2, 3, 5, 7]);
///
/// assert_eq!(LinearSieve::prime_factorization(48), vec![2, 2, 2, 2, 3]);
/// # }
/// ```
pub struct LinearSieve {
    max: usize,
    smallest_prime_factors: Vec<usize>,
}

impl LinearSieve {
    /// Creates a new sieve.
    pub fn new() -> Self {
        Self::default()
    }

    /// Gets smallest prime factors.
    ///
    /// # Examples
    /// ```
    /// # use prime_number_utils::{GenPrime, LinearSieve};
    /// # fn main() {
    /// let mut linear_sieve = LinearSieve::new();
    /// linear_sieve.gen_range(0..10);
    /// assert_eq!(linear_sieve.smallest_prime_factors(), &mut vec![0, 0, 2, 3, 2, 5, 2, 7, 2, 3]);
    /// # }
    /// ```
    pub fn smallest_prime_factors(&mut self) -> &mut Vec<usize> {
        &mut self.smallest_prime_factors
    }

    /// Calculates prime factorization.
    ///
    /// # Examples
    /// ```
    /// # use prime_number_utils::{GenPrime, LinearSieve};
    /// # fn main() {
    /// assert_eq!(LinearSieve::prime_factorization(48), vec![2, 2, 2, 2, 3]);
    /// # }
    /// ```
    pub fn prime_factorization(mut n: usize) -> Vec<usize> {
        let mut prime_factors = vec![];
        if n < 2 {
            return prime_factors;
        }
        let mut linear_sieve = LinearSieve::new();
        linear_sieve.gen_range(0..n + 1);
        let smallest_prime_factors = linear_sieve.smallest_prime_factors();
        while n != 1 {
            let smallest_prime_factors_n = smallest_prime_factors[n];
            prime_factors.push(smallest_prime_factors_n);
            n /= smallest_prime_factors_n;
        }
        prime_factors
    }
}

impl GenPrime for LinearSieve {
    fn gen(&mut self) -> Vec<usize> {
        if self.max < 2 {
            return vec![];
        }
        if self.max == 2 {
            return vec![2];
        }
        self.smallest_prime_factors = vec![2; self.max + 1];
        self.smallest_prime_factors[0] = 0;
        self.smallest_prime_factors[1] = 0;
        let mut primes = vec![2];
        for i in (3..=self.max).step_by(2) {
            if self.smallest_prime_factors[i] == 2 {
                self.smallest_prime_factors[i] = i;
                primes.push(i);
            }
            let mut j = 0;
            while i * primes[j] <= self.max {
                self.smallest_prime_factors[i * primes[j]] = primes[j];
                if primes[j] == self.smallest_prime_factors[i] {
                    break;
                }
                j += 1;
            }
        }
        self.smallest_prime_factors.pop();
        primes
    }

    impl_gen_range!();
}
