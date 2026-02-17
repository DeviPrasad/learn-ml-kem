use crate::field::{modq, modq_i64};
use crate::params::{N, Q, RANK};
use crate::ring::Poly;
use crate::sampler::XOF128;

#[allow(unused)]
// [pow(17, bitreverse(i), p) for i in range(128)]
const NTT_ROOTS: [u16; 128] = [
    1, 1729, 2580, 3289, 2642, 630, 1897, 848, 1062, 1919, 193, 797, 2786, 3260, 569, 1746, 296,
    2447, 1339, 1476, 3046, 56, 2240, 1333, 1426, 2094, 535, 2882, 2393, 2879, 1974, 821, 289, 331,
    3253, 1756, 1197, 2304, 2277, 2055, 650, 1977, 2513, 632, 2865, 33, 1320, 1915, 2319, 1435,
    807, 452, 1438, 2868, 1534, 2402, 2647, 2617, 1481, 648, 2474, 3110, 1227, 910, 17, 2761, 583,
    2649, 1637, 723, 2288, 1100, 1409, 2662, 3281, 233, 756, 2156, 3015, 3050, 1703, 1651, 2789,
    1789, 1847, 952, 1461, 2687, 939, 2308, 2437, 2388, 733, 2337, 268, 641, 1584, 2298, 2037,
    3220, 375, 2549, 2090, 1645, 1063, 319, 2773, 757, 2099, 561, 2466, 2594, 2804, 1092, 403,
    1026, 1143, 2150, 2775, 886, 1722, 1212, 1874, 1029, 2110, 2935, 885, 2154,
];

#[allow(unused)]
// [pow(17, 2*bitreverse(i) + 1, p) for i in range(128)]
const MOD_ROOTS: [u16; 128] = [
    17, 3312, 2761, 568, 583, 2746, 2649, 680, 1637, 1692, 723, 2606, 2288, 1041, 1100, 2229, 1409,
    1920, 2662, 667, 3281, 48, 233, 3096, 756, 2573, 2156, 1173, 3015, 314, 3050, 279, 1703, 1626,
    1651, 1678, 2789, 540, 1789, 1540, 1847, 1482, 952, 2377, 1461, 1868, 2687, 642, 939, 2390,
    2308, 1021, 2437, 892, 2388, 941, 733, 2596, 2337, 992, 268, 3061, 641, 2688, 1584, 1745, 2298,
    1031, 2037, 1292, 3220, 109, 375, 2954, 2549, 780, 2090, 1239, 1645, 1684, 1063, 2266, 319,
    3010, 2773, 556, 757, 2572, 2099, 1230, 561, 2768, 2466, 863, 2594, 735, 2804, 525, 1092, 2237,
    403, 2926, 1026, 2303, 1143, 2186, 2150, 1179, 2775, 554, 886, 2443, 1722, 1607, 1212, 2117,
    1874, 1455, 1029, 2300, 2110, 1219, 2935, 394, 885, 2444, 2154, 1175,
];

#[allow(unused)]
pub fn bit_rev_7(x: u8) -> usize {
    assert!(x <= 127);
    let mut x = x;
    let mut val: u8 = 0;
    for _ in 0..7 {
        let bit = x & 1;
        val <<= 1;
        val |= bit;
        x >>= 1;
    }
    val as usize
}

#[allow(unused)]
#[derive(Clone, Debug, Copy, PartialEq)]
pub struct NTT {
    c: [i32; N],
}

impl Default for NTT {
    fn default() -> Self {
        NTT::from_poly(&Poly::default())
    }
}

impl Into<[u16; N]> for NTT {
    fn into(self) -> [u16; N] {
        self.c.map(|x| x as u16)
    }
}

impl From<[u16; N]> for NTT {
    fn from(value: [u16; N]) -> Self {
        let _ = value.map(|v| assert!(v < Q as u16));
        Self {
            c: value.map(|x| x as i32),
        }
    }
}

#[allow(unused)]
fn pow_mod_q(n: i32, exp: u8) -> i32 {
    const Q32: i32 = Q as i32;
    let mut result = 1i32;
    let mut exp = exp;
    let mut num = n;
    while exp > 0 {
        if exp & 1 == 1 {
            let _dbg_res = (result * num) % Q32;
            result = modq(result * num);
            assert_eq!(_dbg_res, result);
        }
        exp >>= 1;
        let _dbg_res = (num * num) % Q32;
        num = modq(num * num);
        assert_eq!(_dbg_res, num);
    }
    result
}
#[allow(unused)]
impl NTT {
    pub fn from_poly(f: &Poly) -> Self {
        let mut fh = f.coeff();
        let mut i = 1u8;
        let mut len = 128usize;
        while len >= 2 {
            let mut start = 0_usize;
            while start < N {
                let zeta = NTT_ROOTS[i as usize] as i32;
                i += 1;
                for j in start..start + len {
                    let t = modq((zeta * fh[j + len]));
                    fh[j + len] = modq(fh[j] - t);
                    fh[j] = modq(fh[j] + t);
                }
                start += (2 * len);
            }
            len /= 2;
        }
        Self { c: fh }
    }

    pub fn inv(&self) -> Poly {
        let mut f: [i32; N] = self.c.into();
        let mut i = 127;
        let mut len = 2usize;
        while len <= 128 {
            let mut start = 0_usize;
            while start < N {
                let zeta = NTT_ROOTS[i] as i32;
                i -= 1;
                for j in start..start + len {
                    let t = f[j];
                    f[j] = modq(t + f[j + len]);
                    f[j + len] = modq(zeta * (f[j + len] - t));
                }
                start += (len * 2);
            }
            len *= 2;
        }

        Poly::from(&f.map(|x| modq(x * 3303)))
    }

    pub fn add(&self, that: &NTT) -> Self {
        let fh = self.c;
        let th = that.c;
        NTT {
            c: std::array::from_fn(|i| modq((fh[i] + th[i]))),
        }
    }

    pub fn sub(&self, that: &NTT) -> Self {
        let fh = self.c;
        let th = that.c;
        NTT {
            c: std::array::from_fn(|i| modq((fh[i] - th[i]))),
        }
    }

    pub fn mul(&self, that: &NTT) -> Self {
        let fh = self.c;
        let th = that.c;
        let mut rh: [i32; N] = [0i32; N];

        for i in 0..N / 2 {
            (rh[2 * i], rh[2 * i + 1]) = Self::base_case_multiply(
                fh[2 * i],
                fh[2 * i + 1],
                th[2 * i],
                th[2 * i + 1],
                MOD_ROOTS[i] as i32,
            );
        }
        NTT { c: rh }
    }

    pub fn base_case_multiply(a0: i32, a1: i32, b0: i32, b1: i32, gamma: i32) -> (i32, i32) {
        let c0 = modq_i64((a0 as i64 * b0 as i64 + a1 as i64 * b1 as i64 * gamma as i64));
        let c1 = modq_i64((a0 as i64 * b1 as i64 + a1 as i64 * b0 as i64));
        (c0, c1)
    }

    pub fn coefficients(&self) -> [i32; N] {
        self.c
    }
}

#[allow(unused)]
impl NTT {
    pub fn sample_ntt_matrix(rho: &[u8; 32], mut ah: &mut [[NTT; RANK]; RANK]) {
        let mut rho_j_i = [0u8; 34];
        rho_j_i[0..32].copy_from_slice(rho);
        for i in 0..RANK {
            rho_j_i[33] = i as u8;
            for j in 0..RANK {
                rho_j_i[32] = j as u8;
                NTT::sample_ntt(&rho_j_i, &mut ah[i][j]);
            }
        }
    }

    // Algorithm 7. SamplerNTT(B)
    #[allow(dead_code)]
    pub fn sample_ntt(b: &[u8; 34], ah: &mut NTT) {
        let mut xr = XOF128::absorb_finalize(b);
        let mut j = 0;
        while j < 256 {
            let mut c = [0u8; 3];
            xr.squeeze(&mut c);
            let d1 = (c[0] as u16) + (((c[1] & 0xF) as u16) << 8);
            let d2 = ((c[1] >> 4) as u16) + ((c[2] as u16) << 4);
            assert!(d1 < (1 << 12));
            assert!(d2 < (1 << 12));
            if d1 < Q as u16 {
                ah.c[j] = d1 as i32;
                j += 1;
            }
            if d2 < Q as u16 && j < 256 {
                ah.c[j] = d2 as i32;
                j += 1;
            }
        }
    }
}

#[cfg(test)]
mod ntt_tests {
    use crate::field::modq;
    use crate::ntt::{bit_rev_7, pow_mod_q, MOD_ROOTS, NTT, NTT_ROOTS};
    use crate::params::{N, Q};
    use crate::ring::Poly;
    #[test]
    fn test_ntt_00() {
        let n1 = NTT::default();
        let n2 = NTT::default();
        let r: [i32; N] = n1.add(&n2).coefficients();
        assert_eq!(r, [0; N]);
        let r: [i32; N] = n1.mul(&n2).coefficients();
        assert_eq!(r, [0; N]);

        let n3 = NTT::from_poly(&Poly::from(&[1u16; N]));
        let r: [i32; N] = n1.mul(&n3).coefficients();
        assert_eq!(r, [0; N]);
    }

    #[test]
    fn test_ntt_01() {
        let n1 = NTT::from_poly(&Poly::from(&[0u16; N]));
        let n2 = NTT::from_poly(&Poly::from(&[1u16; N]));
        let r = n1.add(&n2).inv();
        assert_eq!(r.coeff(), [1; N]);
    }

    #[test]
    fn test_ntt_add_01() {
        let a: [i32; N] = std::array::from_fn(|_| modq(getrandom::u32().unwrap() as i32));
        let b: [i32; N] = std::array::from_fn(|_| modq(getrandom::u32().unwrap() as i32));
        let n1 = NTT::from_poly(&Poly::from(&a));
        let n2 = NTT::from_poly(&Poly::from(&b));
        let r0 = n1.add(&n2).inv();
        let r1 = std::array::from_fn(|i| modq(a[i] + b[i]));
        assert_eq!(r0.coeff(), r1);
    }

    #[test]
    fn test_ntt_conv_01() {
        // 1 + 200x + 300x^2 + 0^254 + 0^255;
        let mut w = [0i32; N];
        w[0] = modq(Q as i32 - 1);
        w[1] = 200i32;
        w[2] = 300i32;
        w[3] = modq(Q as i32);
        for i in 4..253 {
            // w[i] = modq(modq(getrandom::u32().unwrap() as i32));
            w[i] = modq((getrandom::u32().unwrap() & 0xFFFFFF) as i32);
        }
        w[253] = modq(-1);
        w[254] = 254;
        w[255] = Q as i32 - 2;

        let wh = NTT::from_poly(&Poly::from(&w));
        let _w = wh.inv();
        assert_eq!(w, _w.coeff());
    }

    #[test]
    fn test_bit_rev7_1() {
        assert_eq!(bit_rev_7(0), 0);
        assert_eq!(bit_rev_7(1), 64);
        assert_eq!(bit_rev_7(64), 1);
        assert_eq!(bit_rev_7(2), 32);
        assert_eq!(bit_rev_7(32), 2);
        assert_eq!(bit_rev_7(3), 96);
        assert_eq!(bit_rev_7(96), 3);
        assert_eq!(bit_rev_7(7), 112);
        assert_eq!(bit_rev_7(112), 7);
        assert_eq!(bit_rev_7(63), 126);
        assert_eq!(bit_rev_7(99), 99);
        assert_eq!(bit_rev_7(107), 107);
        assert_eq!(bit_rev_7(119), 119);
        assert_eq!(bit_rev_7(126), 63);
        assert_eq!(bit_rev_7(127), 127);
        assert_eq!(bit_rev_7(98), 35);
    }

    #[test]
    fn test_bit_rev7_2() {
        let mut perm = [0usize; 128];
        let mut mask = [0u8; 128];
        for x in 0..128 {
            let v = bit_rev_7(x);
            assert_eq!(perm[x as usize], 0);
            assert_eq!(mask[v], 0);
            perm[x as usize] = v;
            mask[v] += 1;
        }
        let mut total = 0;
        for i in 0..128 {
            assert_eq!(mask[i], 1);
            total += perm[i]
        }
        assert_eq!(total, 8128); // sum(list(range(0, 128)))
    }

    #[test]
    fn test_mod_roots() {
        for i in 0..128u8 {
            let r = pow_mod_q(17, 2 * bit_rev_7(i) as u8 + 1);
            assert_eq!(r, MOD_ROOTS[i as usize] as i32);
        }
    }

    #[test]
    fn test_ntt_roots() {
        for i in 0..128u8 {
            let r = pow_mod_q(17, bit_rev_7(i) as u8);
            assert_eq!(r, NTT_ROOTS[i as usize] as i32);
        }
    }
}
