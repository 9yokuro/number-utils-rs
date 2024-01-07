use std::mem::swap;

struct MontgomeryMultiplication<BitCountType = u32> {
    n: usize,
    ni: usize,
    nh: usize,
    r: usize,
    rn: usize,
    r2: usize,
    d: usize,
    k: BitCountType,
}

impl MontgomeryMultiplication {
    fn new(n: usize) -> Self {
        let mut ni = n;
        for _ in 0..5 {
            ni = ni.wrapping_mul(2_usize.wrapping_sub(n.wrapping_mul(ni)));
        }
        let nh = (n >> 1) + 1;
        let r = n.wrapping_neg() % n;
        let rn = n - r;
        let r2 = ((n as u128).wrapping_neg() % (n as u128)) as usize;
        let mut d = n - 1;
        let k = d.trailing_zeros();
        d >>= k;
        Self {
            n,
            ni,
            nh,
            r,
            rn,
            r2,
            d,
            k,
        }
    }

    fn add(&self, a: usize, b: usize) -> usize {
        let (t, fa) = a.overflowing_add(b);
        let (u, fs) = t.overflowing_sub(self.n);
        if fa {
            u
        } else {
            #[allow(clippy::collapsible_if)]
            if fs {
                t
            } else {
                u
            }
        }
    }

    fn sub(&self, a: usize, b: usize) -> usize {
        let (t, f) = a.overflowing_sub(b);
        if f {
            t.wrapping_add(self.n)
        } else {
            t
        }
    }

    fn div2(&self, ar: usize) -> usize {
        if (ar & 1) == 0 {
            ar >> 1
        } else {
            (ar >> 1) + self.nh
        }
    }

    fn mrmul(&self, ar: usize, br: usize) -> usize {
        let (n, ni) = (self.n, self.ni);
        let t: u128 = (ar as u128) * (br as u128);
        let (t, f) = ((t >> 64) as usize).overflowing_sub(
            (((((t as usize).wrapping_mul(ni)) as u128) * (n as u128)) >> 64) as usize,
        );
        if f {
            t.wrapping_add(n)
        } else {
            t
        }
    }

    #[allow(dead_code)]
    fn mr(&self, ar: usize) -> usize {
        let (n, ni) = (self.n, self.ni);
        let (t, f) =
            (((((ar.wrapping_mul(ni)) as u128) * (n as u128)) >> 64) as usize).overflowing_neg();
        if f {
            t.wrapping_add(n)
        } else {
            t
        }
    }

    fn ar(&self, a: usize) -> usize {
        self.mrmul(a, self.r2)
    }

    fn powir(&self, mut ar: usize, mut b: usize) -> usize {
        let mut t = if (b & 1) == 0 { self.r } else { ar };
        b >>= 1;
        while b != 0 {
            ar = self.mrmul(ar, ar);
            t = self.mrmul(t, if (b & 1) == 0 { self.r } else { ar });
            b >>= 1;
        }
        t
    }
}

fn isqrt64f(iv: usize) -> (usize, usize) {
    isqrt64i(iv, 0)
}

#[allow(dead_code)]
fn isqrt64d(iv: usize) -> (usize, usize) {
    isqrt64i(iv, iv.leading_zeros())
}

fn isqrt64i(iv: usize, lz: u32) -> (usize, usize) {
    let n = (64 >> 1) - (lz >> 1);
    let s = (lz >> 1) << 1;
    let t = n << 1;
    let (mut a, mut b, c, d, e) = (
        iv as u128,
        0x0000_0000_0000_0000_4000_0000_0000_0000 >> s,
        0xffff_ffff_ffff_fffe_0000_0000_0000_0000 >> s,
        0x0000_0000_0000_0001_0000_0000_0000_0000 >> s,
        0x0000_0000_0000_0000_ffff_ffff_ffff_ffff >> s,
    );
    for _ in 0..n {
        let f = ((b + b) & c) + (b & e);
        if a >= b {
            a -= b;
            b = f + d;
        } else {
            b = f;
        }
        a <<= 2;
    }
    ((b >> t) as usize, (a >> t) as usize)
}

fn jacobi_symbol(a: isize, mut n: usize) -> i32 {
    let (mut a, mut j): (usize, i32) = if a >= 0 {
        (a as usize, 1)
    } else if (n & 3) == 3 {
        ((-a) as usize, -1)
    } else {
        ((-a) as usize, 1)
    };
    while a > 0 {
        let ba = a.trailing_zeros();
        a >>= ba;
        if ((n & 7) == 3 || (n & 7) == 5) && (ba & 1) == 1 {
            j = -j;
        }
        if (a & n & 3) == 3 {
            j = -j;
        }
        swap(&mut a, &mut n);
        a %= n;
        if a > (n >> 1) {
            a = n - a;
            if (n & 3) == 3 {
                j = -j;
            }
        }
    }
    if n == 1 {
        j
    } else {
        0
    }
}

fn miller_rabin_base_2(mont: &MontgomeryMultiplication) -> bool {
    let (r, rn, d, k) = (mont.r, mont.rn, mont.d, mont.k);
    let mut br = mont.powir(mont.add(mont.r, mont.r), d);
    if br == r || br == rn {
        return true;
    }
    for _ in 1..k {
        br = mont.mrmul(br, br);
        if br == rn {
            return true;
        }
    }
    false
}

fn lucas_primality_test(mont: &MontgomeryMultiplication) -> bool {
    let n = mont.n;
    let mut d: isize = 5;
    for i in 0..64 {
        match jacobi_symbol(d, n) {
            -1 => break,
            0 => {
                if d.unsigned_abs() < n {
                    return false;
                }
            }
            _ => {}
        }
        if i == 32 && isqrt64f(n).1 == 0 {
            return false;
        }
        if (i & 1) == 1 {
            d = 2 - d;
        } else {
            d = -(d + 2);
        }
    }
    let qm = mont.ar(if d < 0 {
        ((1 - d) as usize) / 4 % n
    } else {
        n - ((d - 1) as usize) / 4 % n
    });
    let mut k = (n + 1) << (n + 1).leading_zeros();
    let mut um = mont.r;
    let mut vm = mont.r;
    let mut qn = qm;
    let dm: usize = mont.ar(if d < 0 {
        n - (((-d) as usize) % n)
    } else {
        (d as usize) % n
    });
    k <<= 1;
    while k > 0 {
        um = mont.mrmul(um, vm);
        vm = mont.sub(mont.mrmul(vm, vm), mont.add(qn, qn));
        qn = mont.mrmul(qn, qn);
        if (k >> 63) != 0 {
            let mut uu = mont.add(um, vm);
            uu = mont.div2(uu);
            vm = mont.add(mont.mrmul(dm, um), vm);
            vm = mont.div2(vm);
            um = uu;
            qn = mont.mrmul(qn, qm);
        }
        k <<= 1;
    }
    if um == 0 || vm == 0 {
        return true;
    }
    let mut x = (n + 1) & (!n);
    x >>= 1;
    while x > 0 {
        um = mont.mrmul(um, vm);
        vm = mont.sub(mont.mrmul(vm, vm), mont.add(qn, qn));
        if vm == 0 {
            return true;
        }
        qn = mont.mrmul(qn, qn);
        x >>= 1;
    }
    false
}

/// Implementation of Baillie-PSW, a probabilistic primality testing algorithm, using Montgomery
/// modular multiplication.
///
/// # Examples
/// ```
/// # use prime_number_utils::baillie_psw;
/// # fn main() {
/// assert_eq!(baillie_psw(233_419_328), false);
/// assert_eq!(baillie_psw(198_491_317), true);
/// # }
/// ```
pub fn baillie_psw(n: usize) -> bool {
    if n == 2 {
        return true;
    }
    if n == 1 || (n & 1) == 0 {
        return false;
    }
    let mont = MontgomeryMultiplication::new(n);
    miller_rabin_base_2(&mont) && lucas_primality_test(&mont)
}
