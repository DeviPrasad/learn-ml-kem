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
                out_byte |= ((element & MASKS[chunk_bits as usize - 1] as u16) << out_byte_bits) as u8;
                enc[j] = out_byte;
                j += 1;
                out_byte_bits = 0;
                out_byte = 0;
            } else {
                out_byte |= ((element & MASKS[chunk_bits as usize - 1] as u16) << out_byte_bits) as u8;
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
        while element_bits_done < bits {
            if in_byte_bits_left == 0 {
                in_byte = enc[j];
                j += 1;
                in_byte_bits_left = 8;
            }
            let mut chunk_bits = bits - element_bits_done;
            if chunk_bits > in_byte_bits_left {
                chunk_bits = in_byte_bits_left;
            }
            element |= ((in_byte & MASKS[chunk_bits as usize - 1]) as u16) << element_bits_done;
            in_byte_bits_left -= chunk_bits;
            assert!(chunk_bits <= 8);
            // condirional shift right; avoid "attempt to shift right with overflow" error
            if chunk_bits < 8 {
                in_byte >>= chunk_bits;
            } else {
                in_byte = 0;
            }

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
fn byte_encode_11(f: [u16; N], enc: &mut [u8]) {
    for i in 0..N/8 {
        let mut x: u128 = 0;
        x |= (f[i*8] & 0x7FF) as u128;
        x |= ((f[i*8+1] & 0x7FF) as u128) << 11;
        x |= ((f[i*8+2] & 0x7FF) as u128) << 22;
        x |= ((f[i*8+3] & 0x7FF) as u128) << 33;
        x |= ((f[i*8+4] & 0x7FF) as u128) << 44;
        x |= ((f[i*8+5] & 0x7FF) as u128) << 55;
        x |= ((f[i*8+6] & 0x7FF) as u128) << 66;
        x |= ((f[i*8+7] & 0x7FF) as u128) << 77;

        enc[i*11+0] = x as u8;
        enc[i*11+1] = (x >> 8) as u8;
        enc[i*11+2] = (x >> 16) as u8;
        enc[i*11+3] = (x >> 24) as u8;
        enc[i*11+4] = (x >> 32) as u8;
        enc[i*11+5] = (x >> 40) as u8;
        enc[i*11+6] = (x >> 48) as u8;
        enc[i*11+7] = (x >> 56) as u8;
        enc[i*11+8] = (x >> 64) as u8;
        enc[i*11+9] = (x >> 72) as u8;
        enc[i*11+10] = (x >> 80) as u8;
    }
}

#[allow(dead_code)]
fn byte_encode_10(f: [u16; N], enc: &mut [u8]) {
    for i in 0..N/4 {
        let mut x: u64 = 0;
        x |= (f[i*4] & 0x3FF) as u64;
        x |= ((f[i*4+1] & 0x3FF) as u64) << 10;
        x |= ((f[i*4+2] & 0x3FF) as u64) << 20;
        x |= ((f[i*4+3] & 0x3FF) as u64) << 30;

        enc[i*5+0] = x as u8;
        enc[i*5+1] = (x >> 8) as u8;
        enc[i*5+2] = (x >> 16) as u8;
        enc[i*5+3] = (x >> 24) as u8;
        enc[i*5+4] = (x >> 32) as u8;
    }
}

#[allow(dead_code)]
fn byte_encode_5(f: [u16; N], enc: &mut [u8]) {
    assert!(enc.len() >= 32*5);
    for i in 0..N/8 {
        let mut x: u64 = 0;
        x |= f[i*8+0] as u64 & 0x1F;
        x |= (f[i*8+1] as u64 & 0x1F) << 5;
        x |= (f[i*8+2] as u64 & 0x1F) << 10;
        x |= (f[i*8+3] as u64 & 0x1F) << 15;
        x |= (f[i*8+4] as u64 & 0x1F) << 20;
        x |= (f[i*8+5] as u64 & 0x1F) << 25;
        x |= (f[i*8+6] as u64 & 0x1F) << 30;
        x |= (f[i*8+7] as u64 & 0x1F) << 35;

        enc[i*5+0] = x as u8;
        enc[i*5+1] = (x >> 8) as u8;
        enc[i*5+2] = (x >> 16) as u8;
        enc[i*5+3] = (x >> 24) as u8;
        enc[i*5+4] = (x >> 32) as u8;
    }
}

/*
    The bit-decoding in one iteration.
                 encoded byte sequence
        4        3        2        1        0
    <------><---------><-----><---------><------>
    <[5],[3><2],[5],[1><4],[4><1],[5],[2><3],[5]>
      7     6    5     4      3    2     1    0
        decoded field elements - u16 elements
*/
#[allow(dead_code)]
fn byte_decode_5(b: &[u8], f: &mut [u16; N]) {
    assert!(b.len() >= 32 * 5);
    for i in 0..N/8 {
        f[i*8+0] = (b[i*5+0] & 0b0001_1111) as u16;
        f[i*8+1] = ((b[i*5+0] >> 5) & 0b0000_0111) as u16 | ((b[i*5+1] & 0b0000_0011) << 3) as u16;
        f[i*8+2] = ((b[i*5+1] >> 2) & 0b0001_1111) as u16;
        f[i*8+3] = ((b[i*5+1] >> 7) & 0b0000_0001) as u16 | ((b[i*5+2] & 0b0000_1111) << 1) as u16;
        f[i*8+4] = ((b[i*5+2] >> 4) & 0b0000_1111) as u16 | ((b[i*5+3] & 0b0000_0001) << 4) as u16;
        f[i*8+5] = ((b[i*5+3] >> 1) & 0b0001_1111) as u16;
        f[i*8+6] = ((b[i*5+3] >> 6) & 0b0000_0011) as u16 | ((b[i*5+4] & 0b0000_0111) << 2) as u16;
        f[i*8+7] = ((b[i*5+4]  >> 3) & 0b0001_1111) as u16;
    }
}

#[allow(dead_code)]
fn byte_encode_4(f: [u16; N], enc: &mut [u8]) {
    assert!(enc.len() >= 32*4);
    for i in 0..N/2 {
        enc[i] = (f[i*2+0] as u8 & 0xF) | ((f[i*2+1] as u8 & 0xF) << 4);
    }
}

#[allow(dead_code)]
fn byte_decode_4(b: &[u8], f: &mut [u16; N]) {
    assert!(b.len() >= 32*4);
    for i in 0..N/2 {
        f[i*2+0] |= b[i] as u16 & 0x000F;
        f[i*2+1] |= (b[i] as u16 & 0x00F0) >> 4;
    }
}

#[allow(dead_code)]
fn byte_encode_1(f: [u16; N], enc: &mut [u8]) {
    assert!(enc.len() >= 32*1);
    for i in 0..N/8 {
        let mut x = 0u8;
        x |= (f[i*8+0] & 1) as u8;
        x |= (f[i*8+1] & 1) as u8;
        x |= (f[i*8+2] & 1) as u8;
        x |= (f[i*8+3] & 1) as u8;
        x |= (f[i*8+4] & 1) as u8;
        x |= (f[i*8+5] & 1) as u8;
        x |= (f[i*8+6] & 1) as u8;
        x |= (f[i*8+7] & 1) as u8;

        enc[i] = x;
    }
}

#[allow(dead_code)]
pub fn byte_decode_1(b: &[u8], f: &mut [u16; N]) {
    for i in 0..N {
        f[i] = (b[i / 8] >> (i % 8) & 1) as u16;
    }
}



#[allow(dead_code)]
pub fn byte_encode(f: [u16; N], enc: &mut [u8], d: u8) {
    assert!(enc.len() >= 32 * d as usize);
    assert!([10, 11, 12].contains(&d));
    if d == 10 {
        byte_encode_10(f, enc);
    } else if d == 11 {
        byte_encode_11(f, enc);
    } else if d == 4 {
        byte_encode_4(f, enc);
    }
}

#[allow(dead_code)]
pub fn byte_decode(b: &[u8], dec: &mut [u16; N], d: u8) {
    assert!(b.len() >= 32 * d as usize);
    assert!([10, 11, 12].contains(&d));
    let mut bidx: u8 = 0; // # bits filled in 'b' [0..8)
    let mut j = 0;
    for i in 0..N {
        let mut c: u16 = 0; // output byte
        let mut cidx: u8 = 0;
        while cidx < d {
            c |= ((b[j] >> bidx) as u16) << cidx;
            c &= (1 << d) - 1;
            let bits = min(8-bidx, d-cidx);
            bidx += bits;
            assert!(bidx <= 8);
            cidx += bits;
            if bidx == 8 {
                j += 1;
                bidx = 0;
            }
        }
        dec[i] = c;
    }
    assert_eq!(j, b.len());
    assert_eq!(bidx, 0);
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
    use crate::ring::{_byte_decode_bssl_, _byte_encode_bssl_, byte_decode, byte_decode_1, byte_decode_4, byte_decode_5, byte_encode, byte_encode_4, byte_encode_5};

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
    fn test_byte_encode_q_minus_1() {
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

    /**
    16 bit 3328 - 0000110100000000
    10 LSB bits - 0100000000
    Repeated 4 times = 40 bits - 0100000000010000000001000000000100000000
    Encoded in 5 bytes - 01000000 00010000 00000100 00000001 00000000
    values in decimal -   64         16         4          1        0
    */
    #[test]
    fn test_byte_encode_decode_q_minus_1() {
        // let f = [1024u16; N];
        let f = [(Q-1) as u16; N];

        let mut enc = [0u8; 32 * DU as usize];
        let mut enc_bssl = [0u8; 32 * DU as usize];
        byte_encode(f, &mut enc, DU);
        _byte_encode_bssl_(f, &mut enc_bssl, DU);
        // assert_eq!(enc, enc_bssl);
        #[cfg(any(feature = "ML_KEM_512", feature = "ML_KEM_768"))]
        assert_eq!(enc, [0u8, 1, 4, 16, 64].repeat(64).as_slice());
        #[cfg(feature = "ML_KEM_1024")]
        assert_eq!(enc, [0u8, 5, 40, 64, 1, 10, 80, 128, 2, 20, 160].repeat(32).as_slice());

        let mut dec = [0; N];
        let mut dec_bssl = [0; N];
        byte_decode(&enc, &mut dec, DU);
        let ok = _byte_decode_bssl_(&enc, &mut dec_bssl, DU);
        assert!(ok);
        assert_eq!(dec, dec_bssl);
    }

    #[test]
    fn test_byte_encode_decode_random_arr() {
        #[cfg(any(feature = "ML_KEM_512", feature = "ML_KEM_768"))]
        let f = gen_random_u16_array(0x3FF);
        #[cfg(feature = "ML_KEM_1024")]
        let f = gen_random_u16_array(0x7FF);

        let mut enc = [0u8; 32 * DU as usize];
        let mut enc_bssl = [0u8; 32 * DU as usize];
        byte_encode(f, &mut enc, DU);
        _byte_encode_bssl_(f, &mut enc_bssl, DU);
        assert_eq!(enc, enc_bssl);

        let mut dec = [0u16; N];
        byte_decode(&enc, &mut dec, DU);
        assert_eq!(dec, f);
        let mut dec_bssl = [0u16; N];
        let ok = _byte_decode_bssl_(&enc, &mut dec_bssl, DU);
        assert!(ok);
        assert_eq!(dec, dec_bssl);
    }

    #[test]
    fn test_byte_encode_decode_4() {
        let f = gen_random_u16_array(0xFFFF);
        let mut enc = [0u8; 32 * 4usize];
        let mut enc_bssl = [0u8; 32 * 4usize];
        byte_encode_4(f, &mut enc);
        _byte_encode_bssl_(f, &mut enc_bssl, 4);
        assert_eq!(enc, enc_bssl);

        let mut dec = [0u16; N];
        byte_decode_4(&enc, &mut dec);
        let mut dec_bssl = [0u16; N];
        let ok = _byte_decode_bssl_(&enc, &mut dec_bssl, 4);
        assert!(ok);
        assert_eq!(dec, dec_bssl);
    }

    #[test]
    fn test_byte_encode_decode_5() {
        let f = gen_random_u16_array(0xFFF);
        let mut enc = [0u8; 32 * 5usize];
        let mut enc_bssl = [0u8; 32 * 5usize];
        byte_encode_5(f, &mut enc);
        _byte_encode_bssl_(f, &mut enc_bssl, 5);
        assert_eq!(enc, enc_bssl);

        let mut dec = [0u16; N];
        byte_decode_5(&enc, &mut dec);
        let mut dec_bssl = [0u16; N];
        let ok = _byte_decode_bssl_(&enc, &mut dec_bssl, 5);
        assert!(ok);
        assert_eq!(dec, dec_bssl);
    }
}
