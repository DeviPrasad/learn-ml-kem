use sha3::digest::{ExtendableOutput, Update, XofReader};
use sha3::{Shake256, Shake256Reader};
use crate::field::FieldElement;
use crate::ring::Poly;

pub struct XOF {
    r: Shake256Reader,
}

impl XOF {
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
pub fn sample_poly_cbd_eta1(b: &[u8; 64*3], f: &mut Poly) {
    sample_poly_cbd_eta_3(b, f)
}

#[cfg(any(feature="ML_KEM_768", feature="ML_KEM_1024"))]
pub fn sample_poly_cbd_eta1(b: &[u8; 64*2], f: &mut Poly) {
    sample_poly_cbd_eta_2(b, f)
}

pub fn sample_poly_cbd_eta2(b: &[u8; 64*2], f: &mut Poly) {
    sample_poly_cbd_eta_2(b, f)
}


#[allow(dead_code)]
fn sample_poly_cbd_eta_3(b: &[u8; 64*3], f: &mut Poly) {
    let mut _f = f.coefficients();
    for i in 0..256/4usize {
        // read 24 bits
        let mut w: u32 = b[i*3+0] as u32 | ((b[i*3+1] as u32) << 8) | ((b[i*3+2] as u32) << 16);
        // use 6 bits for each coefficient
        for j in 0..4 {
            let x = ((w >> 0) & 1) + ((w >> 1) & 1) + ((w >> 2) & 1);
            let y = ((w >> 3) & 1) + ((w >> 4) & 1) + ((w >> 5) & 1);
            _f[i * 4 + j] = FieldElement::sub(&FieldElement::from(x as i32), &FieldElement::from(y as i32)).into();
            w >>= 6;
        }
    }
}

#[allow(dead_code)]
fn sample_poly_cbd_eta_2(b: &[u8; 64*2], f: &mut Poly) {
    // let mut f = [0u16; 256];
    let mut _f = f.coefficients();
    for i in 0..256/2usize {
        let w = b[i*2];
        let x = ((w >> 0) & 1) + ((w >> 1) & 1);
        let y = ((w >> 2) & 1) + ((w >> 3) & 1);
        _f[i] = FieldElement::sub(&FieldElement::from(x as u16), &FieldElement::from(y as u16)).into();
        let x = ((w >> 4) & 1) + ((w >> 5) & 1);
        let y = ((w >> 6) & 1) + ((w >> 7) & 1);
        _f[i+1] = FieldElement::sub(&FieldElement::from(x as u16), &FieldElement::from(y as u16)).into();
    }
}