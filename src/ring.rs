use crate::codec::{byte_decode_1, byte_encode};
use crate::field;
use crate::field::FieldElement;
use crate::params::{DU, N};

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct RingElement {
    c: [u16; N],
}

impl Default for RingElement {
    fn default() -> Self {
        Self { c: [0u16; N] }
    }
}

impl From<&[FieldElement; N]> for RingElement {
    fn from(farr: &[FieldElement; 256]) -> Self {
        Self {
            c: farr.map(|e| u16::from(e)),
        }
    }
}

impl From<&[u16; N]> for RingElement {
    fn from(farr: &[u16; 256]) -> Self {
        Self {
            c: farr.map(|e| u16::from(e)),
        }
    }
}

#[allow(dead_code)]
impl RingElement {
    pub fn get(&self, i: usize) -> u16 {
        assert!(i > 0 && i < N);
        self.c[i]
    }

    pub fn compress(&self) -> RingElement {
        let mut r: [u16; N] = [0u16; N];
        for i in 0..N {
            #[cfg(any(feature = "ML_KEM_512", feature = "ML_KEM_768"))]
            {
                r[i] = field::compress::<10>(self.get(i));
            }
            #[cfg(feature = "ML_KEM_1024")]
            {
                r[i] = field::compress::<11>(self.get(i));
            }
        }
        RingElement::from(&r)
    }

    #[cfg(any(feature = "ML_KEM_512", feature = "ML_KEM_768"))]
    pub fn byte_encode(&self) -> [u8; 32 * 10] {
        let mut arr = [0; 320];
        byte_encode(self.c, &mut arr, 10);
        arr
    }

    #[cfg(feature = "ML_KEM_1024")]
    pub fn byte_encode(&self) -> [u8; 32 * 11] {
        let mut arr = [0; 352];
        byte_encode(self.c, &mut arr, 11);
        arr
    }

    pub fn byte_decode_1(b: &[u8; 32 * DU as usize]) -> Self {
        let mut r: [u16; N] = [0u16; N];
        byte_decode_1(b, &mut r);
        Self { c: r }
    }
}
