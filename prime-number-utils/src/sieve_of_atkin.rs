use crate::{impl_gen_range, GenPrime};

#[derive(Default, Debug, Clone, Copy, Eq, PartialEq, Hash)]
/// Implementation of the Sieve of Atkin.
///
/// # Examples
/// ```
/// # use prime_number_utils::{GenPrime, SieveOfAtkin};
/// # fn main() {
/// let mut sieve_of_atkin = SieveOfAtkin::new();
/// assert_eq!(sieve_of_atkin.gen_range(0..10), vec![2, 3, 5, 7]);
/// # }
/// ```
pub struct SieveOfAtkin {
    max: usize,
}

impl SieveOfAtkin {
    /// Creates a new sieve.
    pub fn new() -> Self {
        Self::default()
    }
}

impl GenPrime for SieveOfAtkin {
    fn gen(&mut self) -> Vec<usize> {
        if self.max < 2 {
            return vec![];
        }
        if self.max == 2 {
            return vec![2];
        }
        if self.max <= 4 {
            return vec![2, 3];
        }
        let mut sieve = vec![0; self.max + 1];
        let sqrt = (self.max as f64).sqrt() as usize;
        for x in 1..=sqrt {
            for y in 1..=sqrt {
                let xx = x.pow(2);
                let yy = y.pow(2);
                let xx3 = 3 * xx;
                let xx3yy = xx3 + yy;
                let mut n = xx3yy + xx;
                if n <= self.max && (n % 12 == 1 || n % 12 == 5) {
                    sieve[n] ^= 1;
                }
                n = xx3yy;
                if n <= self.max && n % 12 == 7 {
                    sieve[n] ^= 1;
                }
                n = (xx3 as isize - yy as isize) as usize;
                if x > y && n <= self.max && n % 12 == 11 {
                    sieve[n] ^= 1;
                }
            }
        }
        for x in 5..sqrt {
            if sieve[x] == 1 {
                let xx = x.pow(2);
                for y in (xx..=self.max).step_by(xx) {
                    sieve[y] = 0;
                }
            }
        }
        let mut primes = vec![2, 3];
        for i in (5..self.max).step_by(2) {
            if sieve[i] == 1 {
                primes.push(i);
            }
        }
        primes
    }

    impl_gen_range!();
}
