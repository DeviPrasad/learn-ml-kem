use crate::field;
use crate::field::FieldElement;
use crate::params::{DU, N, Q};
use std::cmp::min;

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
pub fn _byte_encode_bssl_(f: [u16; N], enc: &mut [u8], bits: u8) {
    const MASKS: [u8; 8] = [0x01, 0x03, 0x07, 0x0f, 0x1f, 0x3f, 0x7f, 0xff];

    let mut out_byte = 0u8;
    let mut out_byte_bits = 0u8;
    let mut j = 0usize;
    for i in 0..N {
        let mut element: u16 = f[i];
        let mut element_bits_done = 0u8;
        while element_bits_done < bits {
            let mut chunk_bits = bits - element_bits_done;
            let out_bits_remaining = 8 - out_byte_bits;
            if chunk_bits >= out_bits_remaining {
                chunk_bits = out_bits_remaining;
                out_byte |= ((element as u8) & MASKS[chunk_bits as usize - 1]) << out_byte_bits;
                enc[j] = out_byte;
                j += 1;
                out_byte_bits = 0;
                out_byte = 0;
            } else {
                out_byte |= ((element as u8) & MASKS[chunk_bits as usize - 1]) << out_byte_bits;
                out_byte_bits += chunk_bits;
            }
            element_bits_done += chunk_bits;
            element >>= chunk_bits;
        }
    }

    if out_byte_bits > 0 {
        enc[j] = out_byte;
    }
}

#[allow(dead_code)]
// parses |DEGREE * bits| bits from |enc| into |DEGREE| values in
// |dec|. It returns one on success and zero if any parsed value is >= |kPrime|.
pub fn _byte_decode_bssl_(enc: &[u8], dec: &mut [u16; N], bits: u8) -> bool {
    assert!(bits > 1 && bits <= 12);
    const MASKS: [u8; 8] = [0x01, 0x03, 0x07, 0x0f, 0x1f, 0x3f, 0x7f, 0xff];

    let mut in_byte = 0u8;
    let mut in_byte_bits_left = 0u8;

    let mut j = 0;
    for i in 0..N {
        let mut element: u16 = 0;
        let mut element_bits_done = 0u8;
        while (element_bits_done < bits) {
            if (in_byte_bits_left == 0) {
                in_byte = enc[j];
                j += 1;
                in_byte_bits_left = 8;
            }
            let mut chunk_bits = bits - element_bits_done;
            if (chunk_bits > in_byte_bits_left) {
                chunk_bits = in_byte_bits_left;
            }
            element |= ((in_byte & MASKS[chunk_bits as usize - 1]) as u16) << element_bits_done as u16;
            in_byte_bits_left -= chunk_bits;
            in_byte >>= chunk_bits;

            element_bits_done += chunk_bits;
        }
        // An element is only out of range in the case of invalid input, in which
        // case it is okay to leak the comparison.
        if element >= Q as u16 {
            return false;
        }
        dec[i] = element;
    }
    true
}

#[allow(dead_code)]
pub fn byte_encode(f: [u16; N], enc: &mut [u8], d: u8) {
    assert!(enc.len() >= 32 * d as usize);
    assert!([10, 11, 12].contains(&d));
    let mut b: u8 = 0; // output byte
    let mut bidx: u8 = 0; // # bits filled in 'b' [0..8)
    let mut j = 0;
    for e in f {
        assert!((e as u32) < Q);
        let mut eidx: u8 = 0; // # bits already taken from the element 'e' [0..d)
        while eidx < d {
            // do till 'd' bits are consumed from the element
            assert!(bidx < 8);
            b |= ((e >> eidx) as u8) << bidx;
            let bits = min(8 - bidx, d - eidx);
            bidx += bits;
            assert!(bidx <= 8);
            eidx += bits;
            assert!(eidx <= d);
            if bidx == 8 {
                // the output byte is full, write it out
                enc[j] = b;
                j += 1;
                b = 0;
                bidx = 0; // output byte cleared
            }
        }
    }
    assert_eq!(j, enc.len());
    assert_eq!(bidx, 0);
    assert_eq!(b, 0);
}

#[allow(dead_code)]
pub fn byte_decode_1(b: &[u8], dec: &mut [u16; N]) {
    for i in 0..N {
        dec[i] = (b[i / 8] >> (i % 8) & 1) as u16;
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

#[cfg(test)]
mod ring_tests {
    use crate::params::{DU, N, Q};
    use crate::ring::{_byte_encode_bssl_, byte_decode_1, byte_encode};

    #[cfg(test)]
    fn gen_random_u16(mask: u16) -> u16 {
        getrandom::u32().unwrap() as u16 & mask
    }

    #[cfg(test)]
    fn gen_random_u16_array(mask: u16) -> [u16; N] {
        let mut r: [u16; N] = [0; N];
        for i in 0..N {
            r[i] = getrandom::u32().unwrap() as u16 & mask;
        }
        r
    }

    #[test]
    fn test_byte_encode_00() {
        let f = [0u16; N];
        let mut enc = [0u8; 32 * DU as usize];
        let mut enc_bssl = [0u8; 32 * DU as usize];
        byte_encode(f, &mut enc, DU);
        _byte_encode_bssl_(f, &mut enc_bssl, DU);
        assert_eq!(enc, enc_bssl);
    }

    #[test]
    fn test_byte_encode_q_mius_1() {
        #[cfg(any(feature = "ML_KEM_512", feature = "ML_KEM_768"))]
        let f = [Q as u16 & 0x3FF; N];
        #[cfg(feature = "ML_KEM_1024")]
        let f = [Q as u16 & 0x7FF; N];

        let mut enc = [0u8; 32 * DU as usize];
        let mut enc_bssl = [0u8; 32 * DU as usize];
        byte_encode(f, &mut enc, DU);
        _byte_encode_bssl_(f, &mut enc_bssl, DU);
        assert_eq!(enc, enc_bssl);
    }

    #[test]
    fn test_byte_encode_random() {
        #[cfg(any(feature = "ML_KEM_512", feature = "ML_KEM_768"))]
        let f = [gen_random_u16(0x3FF); N];
        #[cfg(feature = "ML_KEM_1024")]
        let f = [gen_random_u16(0x7FF); N];

        let mut enc = [0u8; 32 * DU as usize];
        let mut enc_bssl = [0u8; 32 * DU as usize];
        byte_encode(f, &mut enc, DU);
        _byte_encode_bssl_(f, &mut enc_bssl, DU);
        assert_eq!(enc, enc_bssl);
        println!("{:?}", &f[0..16])
    }

    #[test]
    fn test_byte_encode_random_arr() {
        #[cfg(any(feature = "ML_KEM_512", feature = "ML_KEM_768"))]
        let f = gen_random_u16_array(0x3FF);
        #[cfg(feature = "ML_KEM_1024")]
        let f = gen_random_u16_array(0x7FF);

        let mut enc = [0u8; 32 * DU as usize];
        let mut enc_bssl = [0u8; 32 * DU as usize];
        byte_encode(f, &mut enc, DU);
        _byte_encode_bssl_(f, &mut enc_bssl, DU);
        assert_eq!(enc, enc_bssl);
        println!("{:?}", &f[0..16])
    }

    #[test]
    fn test_byte_decode_generic() {
        {
            let b = [0; 32 * DU as usize];
            let mut dec = [0u16; N];
            byte_decode_1(&b, &mut dec);
            assert_eq!(dec, [0u16; N]);
        }
        {
            let b = [1; 32 * DU as usize];
            let mut dec = [0u16; N];
            byte_decode_1(&b, &mut dec);
            for i in 0..N {
                if i % 8 == 0 {
                    assert_eq!(dec[i], 1u16);
                } else {
                    assert_eq!(dec[i], 0u16);
                }
            }
        }
        {
            let b = [0xF0; 32 * DU as usize];
            let mut dec = [0u16; N];
            byte_decode_1(&b, &mut dec);
            for i in 0..N {
                if i % 8 < 4 {
                    assert_eq!(dec[i], 0u16);
                } else {
                    assert_eq!(dec[i], 1u16);
                }
            }
        }
        {
            let b = [0xFF; 32 * DU as usize];
            let mut dec = [0u16; N];
            byte_decode_1(&b, &mut dec);
            assert_eq!(dec, [1u16; N]);
        }
    }
}
