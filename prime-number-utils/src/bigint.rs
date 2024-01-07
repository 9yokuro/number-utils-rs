use crate::baillie_psw;
use num_bigint::ToBigUint;
use num_iter::range;

/// Lucas-Lehmer test, a primality test for Mersenne numbers.
///
/// # Examples
/// ```
/// # use prime_number_utils::lucas_lehmer;
/// # fn calc() -> Option<()> {
/// assert_eq!(lucas_lehmer(13)?, true);
/// assert_eq!(lucas_lehmer(11)?, false);
/// # Some(())
/// # }
/// # fn main() {
/// # calc().unwrap();
/// # }
/// ```
pub fn lucas_lehmer(n: usize) -> Option<bool> {
    if n == 2 {
        return Some(true);
    }
    if !baillie_psw(n) {
        return Some(false);
    }
    let mut s = 4.to_biguint()?;
    let m = 2.to_biguint()?.pow(n as u32) - 1.to_biguint()?;
    for _ in range(2, n) {
        let ss = s.pow(2);
        s = (&ss & &m) + (ss >> n);
        if s >= m {
            s -= &m;
        }
        s -= 2.to_biguint()?;
    }
    Some(s == 0.to_biguint()?)
}
