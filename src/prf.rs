use sha3::digest::{ExtendableOutput, Update, XofReader};
use sha3::{Digest, Sha3_512, Shake256};
use sha3::digest::Reset;
use crate::params::ETA1;

pub fn sha3_512(data: &[u8], hash: &mut [u8; 64]) {
    let mut g = Sha3_512::new();
    Digest::update(&mut g, data);
    hash.copy_from_slice(&g.finalize());
}

pub fn prf_eta1(s: &[u8], b: u8, hash: &mut [u8; (64 * ETA1) as usize]) {
    let mut hasher = Shake256::default();
    hasher.update(s);
    hasher.update(&[b]);
    let mut r = hasher.finalize_xof();
    r.read(hash)
}
