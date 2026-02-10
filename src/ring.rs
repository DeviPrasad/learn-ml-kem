use crate::codec::{byte_decode_1, byte_encode};
use crate::field;
use crate::field::FieldElement;
use crate::params::{DU, N};

#[allow(dead_code)]
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
            c: farr.map(|e| i32::from(e)),
        }
    }
}

impl Into<[i32; N]> for &RingElement {
    fn into(self) -> [i32; N] {
        self.c.clone()
    }
}

#[allow(dead_code)]
impl RingElement {
    pub fn get(&self, i: usize) -> i32 {
        assert!(i > 0 && i < N);
        self.c[i]
    }

    pub fn compress(&self) -> RingElement {
        let mut r: [u16; N] = [0u16; N];
        for i in 0..N {
            #[cfg(any(feature = "ML_KEM_512", feature = "ML_KEM_768"))]
            {
                r[i] = field::compress::<10>(self.get(i) as u16);
            }
            #[cfg(feature = "ML_KEM_1024")]
            {
                r[i] = field::compress::<11>(self.get(i) as u16);
            }
        }
        RingElement::from(&r)
    }

    #[cfg(any(feature = "ML_KEM_512", feature = "ML_KEM_768"))]
    pub fn byte_encode(&self) -> [u8; 32 * 10] {
        let mut arr = [0; 320];
        byte_encode(self.into(), &mut arr, 10);
        arr
    }

    #[cfg(feature = "ML_KEM_1024")]
    pub fn byte_encode(&self) -> [u8; 32 * 11] {
        let mut arr = [0; 352];
        byte_encode(self.into(), &mut arr, 11);
        arr
    }

    pub fn byte_decode_1(b: &[u8; 32 * DU as usize]) -> Self {
        let mut r: [u16; N] = [0u16; N];
        byte_decode_1(b, &mut r);
        (&r).into()  // Self { c: r.into() }
    }
}


#[allow(dead_code)]
pub type Poly = RingElement;
impl Poly {
    pub fn coefficients(&self) -> [i32; N] {
        self.c.clone()
    }
}

