use sha3::digest::{ExtendableOutput, Update, XofReader};
use sha3::{Shake128, Shake128Reader, Shake256, Shake256Reader};
use crate::params::{ETA1, ETA2, RANK};
use crate::prf;
use crate::ring::Poly;

pub struct XOF128 {
    r: Shake128Reader,
}

pub struct XOF256 {
    r: Shake256Reader,
}

impl XOF128 {
    pub fn absorb_finalize(b: &[u8]) -> Self {
        let mut x = Shake128::default();
        x.update(b);
        Self {
            r: x.finalize_xof(),
        }
    }

    pub fn squeeze(&mut self, mut buf: &mut [u8]) {
        self.r.read(&mut buf);
    }
}

impl XOF256 {
    pub fn absorb_finalize(b: &[u8]) -> Self {
        let mut x = Shake256::default();
        x.update(b);
        Self {
            r: x.finalize_xof(),
        }
    }


    pub fn squeeze(&mut self, mut buf: &mut [u8]) {
        self.r.read(&mut buf);
    }
}

#[cfg(feature="ML_KEM_512")]
fn sample_poly_cbd_eta1(b: &[u8; 64*3], f: &mut Poly) {
    f.sample_poly_cbd_eta_3(b)
}

#[cfg(any(feature="ML_KEM_768", feature="ML_KEM_1024"))]
pub fn sample_poly_cbd_eta1(b: &[u8; 64*2], f: &mut Poly) {
    f.sample_poly_cbd_eta_2(b)
}

pub fn sample_secret_eta1(sigma: [u8; 32], n: &mut u8, s: &mut [Poly; RANK]) {
    for i in 0..RANK {
        let mut prd = [0u8; 64 * ETA1 as usize];
        prf::prf_eta1(&sigma, *n, &mut prd);
        sample_poly_cbd_eta1(&prd, &mut s[i]);
        *n = *n + 1;
    }
}

pub fn sample_secret_eta2(rnd: [u8; 32], n: u8, s: &mut Poly) {
    let mut prd = [0u8; 64 * ETA2 as usize];
    prf::prf_eta2(&rnd, n, &mut prd);
    s.sample_poly_cbd_eta_2(&prd);
}
