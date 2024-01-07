#[doc(hidden)]
#[macro_export]
macro_rules! impl_number_utils_for {
    ($type: ty) => {
        impl NumberUtils for $type {
            fn factorial(&self) -> Self {
                if self <= &1 {
                    1
                } else if self == &2 {
                    2
                } else {
                    (3..=*self).fold(2, |x, y| x * y)
                }
            }

            fn permutation(&self, k: u32) -> Self {
                let k = k as $type;
                if self < &k {
                    0
                } else if self == &0 || k == 0 {
                    1
                } else {
                    let self_k_1 = self - k + &1;
                    (self_k_1 + &1..=*self).fold(self_k_1, |x, y| x * y)
                }
            }

            fn combination(&self, k: u32) -> Self {
                let k = k as $type;
                if self < &k {
                    0
                } else if self == &0 || k == 0 || self == &k {
                    1
                } else if k == 1 {
                    *self
                } else {
                    self.permutation(k as u32) / k.factorial()
                }
            }

            fn gcd(&self, mut m: Self) -> Self {
                let mut n = *self;
                if n == 0 {
                    return m;
                }
                if m == 0 {
                    return n;
                }
                let gcd_exponent_on_two = (n | m).trailing_zeros();
                n >>= n.trailing_zeros();
                m >>= m.trailing_zeros();
                while n != m {
                    if n < m {
                        std::mem::swap(&mut n, &mut m);
                    }
                    n -= m;
                    n >>= n.trailing_zeros();
                }
                n << gcd_exponent_on_two
            }

            fn lcm(&self, m: Self) -> Self {
                self / self.gcd(m) * m
            }

            fn isqrt(&self) -> Self {
                if self < &2 {
                    return *self;
                }
                let mut shift = 2;
                while self >> shift != 0 {
                    shift += 2;
                }
                let mut result = 0;
                while shift >= 0 {
                    result <<= 1;
                    let large_cand = result + 1;
                    if large_cand * large_cand <= self >> shift {
                        result = large_cand;
                    }
                    shift -= 2;
                }
                result
            }

            fn checked_factorial(&self) -> Option<Self> {
                if self <= &1 {
                    Some(1)
                } else if self == &2 {
                    Some(2)
                } else {
                    let mut result: $type = 2;
                    for i in 3..=*self {
                        result = result.checked_mul(i)?;
                    }
                    Some(result)
                }
            }

            fn checked_permutation(&self, k: u32) -> Option<Self> {
                let k = k as $type;
                if self < &k {
                    Some(0)
                } else if self == &0 || k == 0 {
                    Some(1)
                } else {
                    let self_k_1 = self.checked_sub(k)?.checked_add(1)?;
                    let mut result = self_k_1;
                    for i in self_k_1 + 1..=*self {
                        result = result.checked_mul(i)?;
                    }
                    Some(result)
                }
            }

            fn checked_combination(&self, k: u32) -> Option<Self> {
                let k = k as $type;
                if self < &k {
                    Some(0)
                } else if self == &0 || k == 0 || self == &k {
                    Some(1)
                } else if k == 1 {
                    Some(*self)
                } else {
                    self.checked_permutation(k as u32)?.checked_div(k.checked_factorial()?)
                }
            }

            fn digits(&self) -> Self {
                if self == &0 {
                    1
                } else {
                    (self.ilog10() + 1) as $type
                }
            }
        }
    };
    ( $($type: ty),* ) => {
        $( impl_number_utils_for!($type); )*
    }
}
