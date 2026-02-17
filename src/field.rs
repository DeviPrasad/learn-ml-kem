use crate::params::{BARRETT_MULTIPLIER, BARRETT_SHIFT, HALF_Q, Q};

#[inline(always)]
pub fn _modq_(x: i32) -> u16 {
    let x = x as i64;
    let mut t = (x - (((x * 5039) >> 24) * 3329)) as i32;

    // Ensure t is in [0, 2*Q)
    t += (t >> 31) & 3329;
    // Reduce from [0, 2*Q) to [0, Q)
    t -= ((t >= 3329) as i32) * 3329;
    t as u16
}

#[inline(always)]
pub fn modq(x: i32) -> i32 {
    let _expected = _modq_(x);
    let x = x as i64;
    let t = (x - (((x * 5039) >> 24) * 3329)) as i32;

    let t = if t < 0 {
        (t + 3329) as u16
    } else if t >= 3329 {
        (t - 3329) as u16
    } else {
        t as u16
    };

    assert_eq!(t, _expected);
    t as i32
}

#[inline(always)]
pub fn modq_i64(x: i64) -> i32 {
    // Barrett reduction with k = 24
    // t = x - floor(x * 5039 / 2^24) * 3329
    let t = x - (((x * 5039) >> 24) * 3329);

    // Final conditional subtraction
    let mut r = t as i32;
    if r < 0 {
        r += 3329;
    } else if r >= 3329 {
        r -= 3329;
    }
    r
}

#[derive(Clone, Copy, Default)]
pub struct FieldElement {
    v: i32,
}

impl From<i32> for FieldElement {
    fn from(x: i32) -> Self {
        let v = modq(x);
        Self { v }
    }
}

impl From<u16> for FieldElement {
    fn from(x: u16) -> Self {
        Self::from(x as i32)
    }
}

impl From<FieldElement> for i32 {
    fn from(fe: FieldElement) -> i32 {
        fe.v
    }
}

impl From<FieldElement> for u16 {
    fn from(fe: FieldElement) -> u16 {
        fe.v as u16
    }
}

impl FieldElement {
    pub fn reduce_once(a: i32) -> i32 {
        assert_eq!((((a >> 31) & 1) * Q as i32) + a, modq(a));
        (((a >> 31) & 1) * Q as i32) + a
    }

    pub fn add(a: &Self, b: &Self) -> Self {
        Self::reduce_once(a.v + b.v).into()
    }

    pub fn sub(a: &Self, b: &Self) -> Self {
        Self::reduce_once(a.v - b.v).into()
    }
}

// maps a field element uniformly to the range 0 to 2ᵈ-1 per FIPS 203, Def 4.7.
pub fn compress<const D: u8>(x: u16) -> u16 {
    let x: u32 = x as u32;
    assert!(x < Q);
    let dividend = x << D;
    let quotient = (((dividend as u64) * BARRETT_MULTIPLIER) >> BARRETT_SHIFT) as u32;
    let rem = dividend - (quotient * Q);

    // If x < q, the remainder is in the range [0, q+q/2), not [0, q).
    // [ q/2, q+q/2 ) -> round to 1
    assert!(rem < HALF_Q || rem < Q + HALF_Q);
    let f = (rem >= HALF_Q) as u32;
    assert!(quotient <= (1 << D) - 1); // [0, 2^d-1]
    let t = quotient + f;
    assert!(t <= (1 << D)); // [0, 2^d]
    // when when D = 10, after rouding up 't', maintain it in [0, 2^d-1]
    (t & ((1 << D) - 1)) as u16
}

#[allow(dead_code)]
pub fn decompress<const D: u8>(y: u16) -> u16 {
    assert!(y < (1 << D));
    let dividend = y as u32 * Q;
    let quotient = dividend >> D;
    // round up to next higher value.
    assert!(((quotient + ((dividend >> (D - 1)) & 1)) as u16) < (Q as u16));
    (quotient + ((dividend >> (D - 1)) & 1)) as u16
}

#[allow(dead_code)]
pub fn decompress_1(y: u16) -> u16 {
    const HALF_Q_UP: u16 = ((Q + 1) / 2) as u16;
    HALF_Q_UP * y
}

#[allow(dead_code)]
pub fn compress_1(x: u16) -> u16 {
    assert_eq!(
        compress::<1>(x),
        ((((x as u32 * 2) + HALF_Q) / Q) & 1) as u16
    );
    compress::<1>(x)
}

impl FieldElement {
    // maps a field element uniformly to the range 0 to 2ᵈ-1 per FIPS 203, Def 4.7.
    pub fn compress<const D: u8>(&self) -> u16 {
        compress::<D>(self.v as u16)
    }
    pub fn decompress<const D: u8>(y: u16) -> FieldElement {
        FieldElement::from(decompress::<D>(y))
    }
}

#[cfg(test)]
mod modq_tests {
    use crate::field::modq;
    use crate::params::Q;

    #[test]
    fn test_modq() {
        for x in 0..0x0FFFFFFi32 {
            assert_eq!(x % Q as i32, modq(x));
        }
    }
}

#[cfg(test)]
mod compress_tests {
    use crate::field::FieldElement;
    use crate::params::DU;

    #[test]
    fn test_u16_range() {
        for x in 0u16..65535 {
            let t = FieldElement::from(x).compress::<DU>();
            let v = u16::from(t);
            assert!(v <= (1 << DU) - 1);
        }
    }
}

#[cfg(test)]
mod decompress_tests {
    use crate::field::{decompress, FieldElement};
    use crate::params::{DU, Q};

    // for all y in Z_q and d < 12, compress(decompress(y)) = y
    #[test]
    fn test_decompress_then_compress() {
        for x in 0u16..65535 {
            let y = x & ((1 << DU) - 1); // y in [0, 2^d] where d in {1, 10, 11}
            let t = FieldElement::decompress::<DU>(y).compress::<DU>();
            assert_eq!(y, t);
        }
    }

    // if d is large (i.e., close to 12), |x - decompress(compress(x))| <= 2
    #[test]
    fn test_compress_then_decompress() {
        #[cfg(any(
            feature = "ML_KEM_512",
            feature = "ML_KEM_768",
            feature = "ML_KEM_1024"
        ))]
        {
            for x in 0..Q {
                let t: u32 =
                    decompress::<DU>(FieldElement::from(x as i32).compress::<DU>().into()).into();
                // abs_diff = 3328, y = 3328, t = 0
                assert!(x.abs_diff(t) <= 2 || Q.abs_diff(x.abs_diff(t)) <= 1);
            }
        }
    }
}
