use sha3::digest::{ExtendableOutput, Update, XofReader};
use sha3::{Digest, Sha3_512, Shake256};
use crate::params::ETA1;
use crate::sampler::XOF256;

#[allow(unused)]
pub fn sha3_512(data: &[u8], hash: &mut [u8; 64]) {
    let mut g = Sha3_512::new();
    Digest::update(&mut g, data);
    hash.copy_from_slice(&g.finalize());
}


#[allow(unused)]
pub fn prf_eta1(s: &[u8], b: u8, hash: &mut [u8; (64 * ETA1) as usize]) {
    assert_eq!(s.len(), 32);
    assert!(b <= 5);
    let mut d = [0u8; 33];
    d[0..32].copy_from_slice(s);
    d[32] = b;
    let mut xof = XOF256::absorb_finalize(&d);
    xof.squeeze(hash);
    {
        let _hash_= &mut[0u8; (64 * ETA1) as usize];
        let mut hasher = Shake256::default();
        hasher.update(s);
        hasher.update(&[b]);
        let mut r = hasher.finalize_xof();
        r.read(_hash_);
        assert_eq!(hash, _hash_);
    }
}
