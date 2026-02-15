use crate::codec::{byte_decode_1, byte_encode};
use crate::field::{compress, modq, FieldElement};
use crate::params::{DU, DV, N};

#[derive(Clone, Copy, Debug)]
pub struct RingElement {
    c: [i32; N],
}

impl Default for RingElement {
    fn default() -> Self {
        Self { c: [0; N] }
    }
}

impl From<&[FieldElement; N]> for RingElement {
    fn from(farr: &[FieldElement; N]) -> Self {
        Self {
            c: farr.map(|e| i32::from(e)),
        }
    }
}

impl From<&[u16; N]> for RingElement {
    fn from(farr: &[u16; N]) -> Self {
        Self {
            c: farr.map(|e| i32::from(e)),
        }
    }
}

impl Into<[u16; N]> for &RingElement {
    fn into(self) -> [u16; N] {
        self.c.map(|e| e as u16)
    }
}

impl From<&[i32; N]> for RingElement {
    fn from(farr: &[i32; N]) -> Self {
        Self {
            c: farr.clone(),
        }
    }
}

impl Into<[i32; N]> for &RingElement {
    fn into(self) -> [i32; N] {
        self.c
    }
}

impl RingElement {
    pub fn get(&self, i: usize) -> i32 {
        assert!(i > 0 && i < N);
        self.c[i]
    }

    pub fn byte_encode_du(&self) -> [u8; 32*DU as usize] {
        let mut arr = [0; 32*DU as usize];
        byte_encode(self.into(), &mut arr, DU);
        arr
    }

    pub fn byte_encode_dv(&self) -> [u8; 32*DV as usize] {
        let mut arr = [0; 32*DV as usize];
        byte_encode(self.into(), &mut arr, DV);
        arr
    }

    pub fn byte_decode_1(b: &[u8; 32*DU as usize]) -> Self {
        let mut r: [u16; N] = [0u16; N];
        byte_decode_1(b, &mut r);
        (&r).into()  // Self { c: r.into() }
    }
}


pub type Poly = RingElement;
impl Poly {
    pub fn coefficients(&mut self) -> &mut [i32] {
        self.c.as_mut()
    }

    pub fn coeff(&self) -> [i32; N] {
        self.c
    }

    pub fn add(&self, t: &Poly) -> Poly {
        Poly {
            c: std::array::from_fn(|i| modq(self.c[i] + t.c[i]))
        }
    }

    pub fn compress<const D: u8>(&self) -> Poly {
        Poly::from(&self.c.map(|c| compress::<D>(c as u16)))
    }

    pub(crate) fn sample_poly_cbd_eta_2(&mut self, b: &[u8; 64*2]) {
        for i in 0..N/2 {
            let w = b[i] & 0xF;
            let x = (w & 1) + ((w >> 1) & 1);
            let y = ((w >> 2) & 1) + ((w >> 3) & 1);
            self.c[i*2] = FieldElement::reduce_once(x as i32 - y as i32);
            let w = b[i] & 0xF0;
            let x = ((w >> 4) & 1) + ((w >> 5) & 1);
            let y = ((w >> 6) & 1) + ((w >> 7) & 1);
            self.c[i*2+1] = FieldElement::reduce_once(x as i32 - y as i32);
        }
    }

    #[cfg(feature="ML_KEM_512")]
    pub fn sample_poly_cbd_eta_3(&mut self, b: &[u8; 64*3]) {
        for i in 0..N/4usize {
            // read 24 bits
            let mut w: u32 = b[i*3+0] as u32 | ((b[i*3+1] as u32) << 8) | ((b[i*3+2] as u32) << 16);
            // use 6 bits for each coefficient
            for j in 0..4 {
                let x = ((w >> 0) & 1) + ((w >> 1) & 1) + ((w >> 2) & 1);
                let y = ((w >> 3) & 1) + ((w >> 4) & 1) + ((w >> 5) & 1);
                self.c[i * 4 + j] = FieldElement::sub(&FieldElement::from(x as i32), &FieldElement::from(y as i32)).into();
                w >>= 6;
            }
        }
    }
}

