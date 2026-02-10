use crate::ntt::NTT;
use crate::params::{ETA1, RANK};
use crate::ring::Poly;
use crate::{prf, sampler};
use crate::codec::byte_encode;

#[allow(unused)]
#[derive(Clone, Debug, PartialEq)]
pub struct EncryptionKey {
    key: [u8; 384 * RANK + 32],
    seed: [u8; 32],
}

#[allow(unused)]
impl EncryptionKey {
    pub fn new(key: [u8; 384 * RANK + 32], seed: [u8; 32]) -> Self {
        Self { key, seed }
    }
}

#[allow(unused)]
pub struct DecryptionKey {
    key: [u8; 384 * RANK],
}

#[allow(unused)]
impl DecryptionKey {
    pub fn new(key: [u8; 384 * RANK]) -> Self {
        DecryptionKey { key }
    }
}

#[allow(unused)]
fn gen_rho_sigma(sr: [u8; 33], rho: &mut [u8; 32], sigma: &mut [u8; 32]) {
    let mut pr = [0u8; 64];
    prf::sha3_512(&sr, &mut pr);
    rho.copy_from_slice(&pr[0..32]);
    sigma.copy_from_slice(&pr[32..]);
}

#[allow(unused)]
pub fn key_gen(d: [u8; 32]) -> (EncryptionKey, DecryptionKey) {
    let sr = {
        let mut sr = [0u8; 33];
        sr[0..32].copy_from_slice(&d);
        sr[32] = RANK as u8;
        sr
    };

    let (rho, sigma) = {
        let mut rho = [0u8; 32];
        let mut sigma = [0u8; 32];
        gen_rho_sigma(sr, &mut rho, &mut sigma);
        (rho, sigma)
    };

    let ah: [[NTT; RANK]; RANK] = {
        let mut rho_j_i = [0u8; 34];
        let mut ah: [[NTT; RANK]; RANK] = [[NTT::default(); RANK]; RANK];
        rho_j_i.copy_from_slice(&rho[0..32]);
        for i in 0..RANK {
            rho_j_i[33] = i as u8;
            for j in 0..RANK {
                rho_j_i[32] = j as u8;
                NTT::sample_ntt(&rho_j_i, &mut ah[i][j]);
            }
        }
        ah
    };

    let (s, n) = {
        let mut n = 0;
        let mut s = [Poly::default(); RANK];
        for i in 0..RANK {
            let mut prd = [0u8; 64 * ETA1 as usize];
            prf::prf_eta1(&rho, n, &mut prd);
            sampler::sample_poly_cbd_eta1(&prd, &mut s[i]);
            n += 1;
        }
        (s, n)
    };

    let e = {
        let mut n = n;
        let mut e = [Poly::default(); RANK];
        for i in 0..RANK {
            let mut prd = [0u8; 64 * ETA1 as usize];
            prf::prf_eta1(&rho, n, &mut prd);
            sampler::sample_poly_cbd_eta1(&prd, &mut e[i]);
            n += 1;
        }
        e
    };

    let sh = {
        let mut sh = [NTT::default(); RANK];
        for i in 0..RANK {
            sh[i] = NTT::from_poly(&s[i]);
        }
        sh
    };

    let eh = {
        let mut eh = [NTT::default(); RANK];
        for i in 0..RANK {
            eh[i] = NTT::from_poly(&e[i]);
        }
        eh
    };

    let ph = {
        let mut ph = [NTT::default(); RANK];
        for i in 0..RANK {
            for j in 0..RANK {
                ph[i] = ph[i].add(&ah[i][j].mul(&sh[j]));
            }
        }
        ph
    };

    let th = {
        let mut th = [NTT::default(); RANK];
        for i in 0..RANK {
            th[i] = ph[i].add(&eh[i]);
        }
        th
    };

    let mut pke_ek = [0u8; 384 * RANK + 32];
    // byte_encode(&th, &mut pke_ek, 12);

    let pk = EncryptionKey::new([0u8; 384 * RANK + 32], [0u8; 32]);
    let sk = DecryptionKey::new([0u8; 384 * RANK]);
    (pk, sk)
}
