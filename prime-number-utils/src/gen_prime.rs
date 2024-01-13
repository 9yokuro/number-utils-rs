pub trait GenPrime<T = usize> {
    /// Finds all prime numbers in the given range.
    ///
    /// # Examples
    /// ```
    /// # use prime_number_utils::{SieveOfEratosthenes, GenPrime};
    /// # fn main() {
    /// let mut sieve_of_eratosthenes = SieveOfEratosthenes::new();
    ///
    /// assert_eq!(sieve_of_eratosthenes.gen_range(0..10), vec![2, 3, 5, 7]);
    /// # }
    fn gen_range(&mut self, range: std::ops::Range<usize>) -> Vec<T>;
    #[doc(hidden)]
    fn gen(&mut self) -> Vec<T>;
}

#[doc(hidden)]
#[macro_export]
macro_rules! impl_gen_range {
    () => {
        fn gen_range(&mut self, range: std::ops::Range<usize>) -> Vec<usize> {
            let min = range.start;
            self.max = range.end;
            let mut primes = self.gen();
            primes.retain(|&x| min <= x);
            primes
        }
    };
}
