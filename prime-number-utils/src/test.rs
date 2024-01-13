#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_sieve_of_eratosthenes() {
        let mut sieve_of_eratosthenes = SieveOfEratosthenes::new();
        assert_eq!(
            sieve_of_eratosthenes.gen_range(0..1_000_000_000).len(),
            50_847_534
        );
    }

    #[test]
    fn test_sieve_of_sundaram() {
        let mut sieve_of_sundaram = SieveOfSundaram::new();
        assert_eq!(
            sieve_of_sundaram.gen_range(0..1_000_000_000).len(),
            50_847_534
        );
    }

    #[test]
    fn test_sieve_of_atkin() {
        let mut sieve_of_atkin = SieveOfAtkin::new();
        assert_eq!(sieve_of_atkin.gen_range(0..1_000_000_000).len(), 50_847_534);
    }

    #[test]
    fn test_bitwise_sieve() {
        let mut bitwise_sieve = BitwiseSieve::new();
        assert_eq!(bitwise_sieve.gen_range(0..1_000_000_000).len(), 50_847_534);
    }

    #[test]
    fn test_segmented_sieve() {
        assert_eq!(
            SegmentedSieve::new().gen_range(0..1_000_000_000).len(),
            50_847_534
        );
    }

    #[test]
    fn test_linear_sieve() {
        let mut linear_sieve = LinearSieve::new();
        assert_eq!(linear_sieve.gen_range(0..1_000_000_000).len(), 50_847_534);
        assert_eq!(LinearSieve::prime_factorization(48), vec![2, 2, 2, 2, 3]);
    }

    #[test]
    fn test_baillie_psw() {
        let mut sieve_of_eratosthenes = SieveOfEratosthenes::new();
        for i in sieve_of_eratosthenes.gen_range(0..100_000_000) {
            assert_eq!(baillie_psw(i), true);
        }
    }

    #[test]
    fn test_lucas_lehmer() {
        let mut mersenne_primes = vec![];
        for i in 0..1000 {
            if lucas_lehmer(i).unwrap() {
                mersenne_primes.push(i);
            }
        }
        assert_eq!(
            mersenne_primes,
            vec![2, 3, 5, 7, 13, 17, 19, 31, 61, 89, 107, 127, 521, 607]
        );
    }
}
