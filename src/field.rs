use crate::params::{
    BARRETT_MULTIPLIER_24, BARRETT_MULTIPLIER_32, BARRETT_SHIFT_24, BARRETT_SHIFT_32, HALF_Q,
    HALF_Q_UP, Q, QI32, QI64,
};

#[inline(always)]
pub fn _modq_(x: i32) -> u16 {
    let x = x as i64;
    let mut t = (x - (((x * BARRETT_MULTIPLIER_24) >> BARRETT_SHIFT_24) * QI64)) as i32;

    // Ensure t is in [0, 2*Q)
    t += (t >> 31) & QI32;
    // Reduce from [0, 2*Q) to [0, Q)
    t -= (t >= QI32) as i32 * QI32;
    assert!(t as u16 <= Q as u16);
    t as u16
}

#[inline(always)]
pub fn modq(x: i32) -> i32 {
    let _expected = _modq_(x);
    let x = x as i64;
    let t = (x - (((x * BARRETT_MULTIPLIER_32) >> BARRETT_SHIFT_32) * QI64)) as i32;

    let t = if t < 0 {
        (t + QI32) as u16
    } else if t >= QI32 {
        (t - QI32) as u16
    } else {
        t as u16
    };

    assert_eq!(t, _expected);
    t as i32
}

#[inline(always)]
pub fn modq_i64(x: i64) -> i32 {
    let qh = (x * BARRETT_MULTIPLIER_32) >> BARRETT_SHIFT_32;
    let mut t = x - qh * QI64;

    // single correction is enough for ML-KEM bounds
    t -= ((t >= QI64) as i64) * QI64;
    t += ((t < 0) as i64) * QI64;

    t as i32
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
        (((a >> 31) & 1) * QI32) + a
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
    assert!(D < 12);
    let x = x as u32;
    assert!(x < Q);

    // a = x*2^d + q/2   (spec rounding)
    let a = (x << D) + (Q >> 1);

    // Barrett approximate division
    let t = ((a as u64 * 1290167) >> 32) as u32;

    // One correction (branchless)
    let r = a - t * Q;
    let t = t + ((r >= Q) as u32);

    assert_eq!(
        t & ((1 << D) - 1),
        (((x << D) + Q / 2) / Q) & ((1 << D) - 1)
    );
    (t & ((1 << D) - 1)) as u16
}

#[allow(unused)]
#[inline(always)]
pub fn decompress<const D: u8>(y: u16) -> u16 {
    assert!(D < 12);
    assert!(y < (1 << D));

    let t = (y as u32) * Q + (1 << (D - 1));
    (t >> D) as u16
}

#[allow(unused)]
#[inline(always)]
pub fn decompress_1(y: u16) -> u16 {
    debug_assert!(y < Q as u16);
    debug_assert!(y < 2);
    HALF_Q_UP * y
}

#[allow(unused)]
#[inline(always)]
pub fn compress_1(x: u16) -> u16 {
    debug_assert_eq!(
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
    use crate::field::{FieldElement, decompress};
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
