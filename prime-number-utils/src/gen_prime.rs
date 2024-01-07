use std::ops::RangeBounds;

pub trait GenPrime<T = usize> {
    /// Finds all prime numbers in the given range.
    ///
    /// # Examples
    /// ```
    /// # use prime_number_utils::{SieveOfEratosthenes, GenPrime};
    /// # fn main() {
    /// let mut sieve_of_eratosthenes = SieveOfEratosthenes::new();
    ///
    /// // Exclusive range
    /// assert_eq!(sieve_of_eratosthenes.gen_range(0..11), vec![2, 3, 5, 7]);
    ///
    /// // Inclusive range
    /// assert_eq!(sieve_of_eratosthenes.gen_range(0..=11), vec![2, 3, 5, 7, 11]);
    /// # }
    /// ```
    fn gen_range<R: RangeBounds<T>>(&mut self, range: R) -> Vec<T>;
    #[doc(hidden)]
    fn gen(&mut self) -> Vec<T>;
}

#[doc(hidden)]
#[macro_export]
macro_rules! impl_gen_range {
    () => {
        fn gen_range<R: RangeBounds<usize>>(&mut self, range: R) -> Vec<usize> {
            self.min = match range.start_bound() {
                Unbounded => 0,
                Excluded(&n) => n + 1,
                Included(&n) => n,
            };
            self.max = match range.end_bound() {
                Unbounded => panic!("Incorrect RangeBound"),
                Excluded(&n) => n,
                Included(&n) => n + 1,
            };
            let mut primes = self.gen();
            primes.retain(|&x| self.min <= x);
            primes
        }
    };
}
