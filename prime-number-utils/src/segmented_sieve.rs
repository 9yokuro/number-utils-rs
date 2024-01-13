use crate::{impl_gen_range, BitwiseSieve, GenPrime};

/// Implementation of the Segmented sieve.
///
/// # Examples
/// ```
/// # use prime_number_utils::{GenPrime, SegmentedSieve};
/// # fn main() {
/// let mut segmented_sieve = SegmentedSieve::new();
/// assert_eq!(segmented_sieve.gen_range(0..10), vec![2, 3, 5, 7]);
/// # }
/// ```
#[derive(Default, Debug, Clone, Eq, PartialEq, Hash)]
pub struct SegmentedSieve {
    max: usize,
}

impl SegmentedSieve {
    /// Creates a new sieve.
    pub fn new() -> Self {
        Self::default()
    }
}

impl GenPrime for SegmentedSieve {
    fn gen(&mut self) -> Vec<usize> {
        if self.max < 2 {
            return vec![];
        }
        let n = (self.max as f64).sqrt() as usize;
        let mut primes = BitwiseSieve::new().gen_range(0..n + 1);
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
