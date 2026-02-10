use sha3::digest::{ExtendableOutput, Update, XofReader};
use std::cell::RefCell;
use sha3::{Shake256, Shake256Reader};
pub fn xof_init() -> Shake256 {
    Shake256::default()
}

pub fn xof_absorb<'a>(x: &'a mut Shake256, b: &[u8]) -> &'a Shake256 {
    x.update(b);
    x
}

pub fn xof_finalize(x: Shake256) -> Shake256Reader {
    x.finalize_xof()
}

pub fn xof_squeeze<'a>(xr: &'a mut Shake256Reader, mut buf: &mut [u8]) -> &'a mut Shake256Reader {
    xr.read(&mut buf);
    xr
}

// Algorithm 7. SamplerNTT(B)
#[allow(dead_code)]
pub fn sample_ntt(b: [u8; 34]) {
    let mut x = xof_init();
    xof_absorb(&mut x, &b);
    let mut xr = xof_finalize(x);
    let mut j = 0;
    while j < 256 {
        let mut c = [0u8; 3];
        xof_squeeze(&mut xr, &mut c);
        let d1 = c[0] as u16 + (c[1] as u16 & 0xF) << 8;
        let d2 = c[1] as u16 >> 4 + (c[2] as u16) << 4;
    }
}