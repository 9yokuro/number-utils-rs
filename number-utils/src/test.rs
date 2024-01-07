#[cfg(test)]
mod tests {
    use crate::*;
    use num_bigint::ToBigUint;

    #[test]
    fn test_factorial() {
        assert_eq!(6_u32.factorial(), 720);
        assert_eq!(6_u64.factorial(), 720);
        assert_eq!(6_u128.factorial(), 720);
        assert_eq!(6_usize.factorial(), 720);
        assert_eq!(6_u32.checked_factorial(), Some(720));
        assert_eq!(6_u64.checked_factorial(), Some(720));
        assert_eq!(6_u128.checked_factorial(), Some(720));
        assert_eq!(6_usize.checked_factorial(), Some(720));
        assert_eq!(factorial_big(6), 720.to_biguint());
    }

    #[test]
    fn test_permutation() {
        assert_eq!(6_u32.permutation(3), 120);
        assert_eq!(6_u64.permutation(3), 120);
        assert_eq!(6_u128.permutation(3), 120);
        assert_eq!(6_usize.permutation(3), 120);
        assert_eq!(permutation_big(6, 3), 120.to_biguint());
    }

    #[test]
    fn test_combination() {
        assert_eq!(6_u32.combination(3), 20);
        assert_eq!(6_u64.combination(3), 20);
        assert_eq!(6_u128.combination(3), 20);
        assert_eq!(6_usize.combination(3), 20);
        assert_eq!(combination_big(6, 3), 20.to_biguint());
    }

    #[test]
    fn test_gcd() {
        assert_eq!(54_u32.gcd(24), 6);
        assert_eq!(54_u64.gcd(24), 6);
        assert_eq!(54_u128.gcd(24), 6);
        assert_eq!(54_usize.gcd(24), 6);
        assert_eq!(gcd_big(54, 24), 6.to_biguint());
    }

    #[test]
    fn test_lcm() {
        assert_eq!(72_u32.lcm(10), 360);
        assert_eq!(72_u64.lcm(10), 360);
        assert_eq!(72_u128.lcm(10), 360);
        assert_eq!(72_usize.lcm(10), 360);
        assert_eq!(lcm_big(72, 10), 360.to_biguint());
    }

    #[test]
    fn test_isqrt() {
        assert_eq!(122_u32.isqrt(), 11);
        assert_eq!(122_u64.isqrt(), 11);
        assert_eq!(122_u128.isqrt(), 11);
        assert_eq!(122_usize.isqrt(), 11);
    }

    #[test]
    fn test_pascals_triangle() {
        assert_eq!(
            pascals_triangle(6),
            vec![1, 1, 1, 1, 2, 1, 1, 3, 3, 1, 1, 4, 6, 4, 1, 1, 5, 10, 10, 5, 1]
        );
    }
}
