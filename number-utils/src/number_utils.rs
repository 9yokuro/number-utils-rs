use crate::impl_number_utils_for;
use std::marker::Sized;

impl_number_utils_for!(u32, u64, u128, usize);

pub trait NumberUtils {
    /// Calculates factorial.
    ///
    /// # Examples
    /// Basic usage:
    /// ```
    /// # use number_utils::NumberUtils;
    /// # fn main() {
    /// let n: u32 = 6;
    /// assert_eq!(n.factorial(), 720);
    /// # }
    /// ```
    fn factorial(&self) -> Self;

    /// Calculates k-permutations.
    ///
    /// # Examples
    /// Basic usage:
    /// ```
    /// # use number_utils::NumberUtils;
    /// # fn main() {
    /// let n: u32 = 6;
    /// assert_eq!(n.permutation(3), 120);
    /// # }
    /// ```
    fn permutation(&self, k: u32) -> Self;

    /// Calculates k-combination.
    ///
    /// # Examples
    /// Basic usage:
    /// ```
    /// # use number_utils::NumberUtils;
    /// # fn main() {
    /// let n: u32 = 6;
    /// assert_eq!(n.combination(3), 20);
    /// # }
    /// ```
    fn combination(&self, k: u32) -> Self;

    /// Calculates greatest common divisor of two integers.
    ///
    /// # Examples
    /// Basic usage:
    /// ```
    /// # use number_utils::NumberUtils;
    /// # fn main() {
    /// let n: u32 = 54;
    /// assert_eq!(n.gcd(24), 6);
    /// # }
    /// ```
    fn gcd(&self, m: Self) -> Self;

    /// Calculates least common multiple.
    ///
    /// # Examples
    /// Basic usage:
    /// ```
    /// # use number_utils::NumberUtils;
    /// # fn main() {
    /// let n: u32 = 72;
    /// assert_eq!(n.lcm(10), 360);
    /// # }
    fn lcm(&self, m: Self) -> Self;

    /// Calculates integer square root.
    ///
    /// # Examples
    /// Basic usage:
    /// ```
    /// # use number_utils::NumberUtils;
    /// # fn main() {
    /// let n: u32 = 122;
    /// assert_eq!(n.isqrt(), 11);
    /// # }
    /// ```
    fn isqrt(&self) -> Self;

    /// Calculates factorial, returning None if overflow occured.
    ///
    /// # Examples
    /// Basic usage:
    /// ```
    /// # use number_utils::NumberUtils;
    /// # fn main() {
    /// let n: u32 = 5;
    /// assert_eq!(n.checked_factorial(), Some(120));
    /// assert_eq!(u32::MAX.checked_factorial(), None);
    /// # }
    /// ```
    fn checked_factorial(&self) -> Option<Self>
    where
        Self: Sized;

    /// Calculates k-permutations, returning None if overflow occured.
    ///
    /// # Examples
    /// Basic usage:
    /// ```
    /// # use number_utils::NumberUtils;
    /// # fn main() {
    /// let n: u32 = 6;
    /// assert_eq!(n.checked_permutation(3), Some(120));
    /// assert_eq!(u32::MAX.checked_permutation(3), None);
    /// # }
    /// ```
    fn checked_permutation(&self, k: u32) -> Option<Self>
    where
        Self: Sized;

    /// Calculates k-combination, returning None if overflow occured.
    ///
    /// # Examples
    /// Basic usage:
    /// ```
    /// # use number_utils::NumberUtils;
    /// # fn main() {
    /// let n: u32 = 6;
    /// assert_eq!(n.checked_combination(3), Some(20));
    /// assert_eq!(u32::MAX.checked_combination(1200), None);
    /// # }
    /// ```
    fn checked_combination(&self, k: u32) -> Option<Self>
    where
        Self: Sized;

    /// Calculates number of digits.
    ///
    /// # Examples
    /// ```
    /// # use number_utils::NumberUtils;
    /// # fn main() {
    /// let n: u32 = 1234567890;
    /// assert_eq!(n.digits(), 10);
    /// # }
    /// ```
    fn digits(&self) -> Self;
}

/// Implementation of the Pascal's triangle.
///
/// # Examples
/// ```
/// # use number_utils::pascals_triangle;
/// # fn main() {
/// assert_eq!(pascals_triangle(3), vec![1, 1, 1, 1, 2, 1]);
/// # }
/// ```
pub fn pascals_triangle(n: usize) -> Vec<usize> {
    let mut result = vec![1];
    for i in 1..n {
        let r = result.len() - i;
        result.push(1);
        for j in 1..i {
            result.push(result[r + j - 1] + result[r + j]);
        }
        result.push(1);
    }
    result
}
