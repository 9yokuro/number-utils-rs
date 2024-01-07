use num_bigint::{BigUint, ToBigUint};
use num_iter::range_inclusive;

/// Calculates factorial.
///
/// # Examples
/// ```
/// # use number_utils::factorial_big;
/// # use num_bigint::ToBigUint;
/// # fn calc() -> Option<()> {
/// assert_eq!(factorial_big(30)?, 265_252_859_812_191_058_636_308_480_000_000_u128.to_biguint()?);
/// # Some(())
/// # }
/// # fn main() {
/// # calc().unwrap();
/// # }
/// ```
pub fn factorial_big<T: ToBigUint>(n: T) -> Option<BigUint> {
    let n = n.to_biguint()?;
    let biguint_1 = 1.to_biguint()?;
    let biguint_2 = 2.to_biguint()?;
    if n <= biguint_1 {
        Some(biguint_1)
    } else if n == biguint_2 {
        Some(biguint_2)
    } else {
        let mut factorial_big = biguint_2;
        for i in range_inclusive(3.to_biguint()?, n) {
            factorial_big *= i;
        }
        Some(factorial_big)
    }
}

/// Calculates k-permutations.
///
/// # Examples
/// ```
/// # use number_utils::permutation_big;
/// # use num_bigint::ToBigUint;
/// # fn calc() -> Option<()> {
/// assert_eq!(permutation_big(30, 15)?, 202_843_204_931_727_360_000_u128.to_biguint()?);
/// # Some(())
/// # }
/// # fn main() {
/// # calc();
/// # }
/// ```
pub fn permutation_big<T: ToBigUint>(n: T, k: T) -> Option<BigUint> {
    let n = n.to_biguint()?;
    let k = k.to_biguint()?;
    let biguint_0 = 0.to_biguint()?;
    let biguint_1 = 1.to_biguint()?;
    if n < k {
        Some(biguint_0)
    } else if n == biguint_0 || k == biguint_0 {
        Some(biguint_1)
    } else {
        let mut permutation_big = &n - &k + &biguint_1;
        for i in range_inclusive(&n - &k + 2.to_biguint()?, n) {
            permutation_big *= i;
        }
        Some(permutation_big)
    }
}

/// Calculates k-combination.
///
/// # Examples
/// ```
/// # use number_utils::combination_big;
/// # use num_bigint::ToBigUint;
/// # fn calc() -> Option<()> {
/// assert_eq!(combination_big(30, 15)?, 155_117_520_u32.to_biguint()?);
/// # Some(())
/// # }
/// # fn main() {
/// # calc();
/// # }
/// ```
pub fn combination_big<T: ToBigUint>(n: T, k: T) -> Option<BigUint> {
    let n = n.to_biguint()?;
    let k = k.to_biguint()?;
    let biguint_0 = 0.to_biguint()?;
    let biguint_1 = 1.to_biguint()?;
    if n < k {
        Some(biguint_0)
    } else if n == biguint_0 || k == biguint_0 || n == k {
        Some(biguint_1)
    } else if k == biguint_1 {
        Some(n)
    } else {
        Some(permutation_big(n, k.clone())? / factorial_big(k)?)
    }
}

/// Calculates greatest common divisor of two integers.
///
/// # Examples
/// ```
/// # use number_utils::gcd_big;
/// # use num_bigint::ToBigUint;
/// # fn calc() -> Option<()> {
/// assert_eq!(gcd_big(1247, 1073)?, 29_u32.to_biguint()?);
/// # Some(())
/// # }
/// # fn main() {
/// # calc().unwrap();
/// # }
/// ```
pub fn gcd_big<T: ToBigUint>(n: T, m: T) -> Option<BigUint> {
    let mut n = n.to_biguint()?;
    let mut m = m.to_biguint()?;
    let biguint_0 = 0.to_biguint()?;
    if n == biguint_0 {
        return Some(m);
    }
    if m == biguint_0 {
        return Some(n);
    }
    let gcd_exponent_on_two = (&n | &m).trailing_zeros()?;
    n >>= n.trailing_zeros()?;
    m >>= m.trailing_zeros()?;
    while n != m {
        if n < m {
            std::mem::swap(&mut n, &mut m);
        }
        n -= &m;
        n >>= n.trailing_zeros()?;
    }
    Some(n << gcd_exponent_on_two)
}

/// Calculates least common multiple.
///
/// # Examples
/// ```
/// # use number_utils::lcm_big;
/// # use num_bigint::ToBigUint;
/// # fn calc() -> Option<()> {
/// assert_eq!(lcm_big(120, 50)?, 600_u32.to_biguint()?);
/// # Some(())
/// # }
/// # fn main() {
/// # calc().unwrap();
/// # }
/// ```
pub fn lcm_big<T: ToBigUint>(n: T, m: T) -> Option<BigUint> {
    let biguint_n = n.to_biguint()?;
    let biguint_m = m.to_biguint()?;
    Some(biguint_n / gcd_big(n, m)? * biguint_m)
}
