use crate::params::{DU, DV, RANK};

mod codec;
mod decrypt;
mod encrypt;
mod field;
mod keygen;
mod ntt;
mod params;
mod prf;
mod ring;
mod sampler;

fn main() {
    let (pk, dk, _) = keygen::key_gen([0u8; 32]);
    let c: [u8; 32 * (DU * RANK as u8 + DV) as usize] = pk.encrypt([128u8; 32], [1u8; 32]);
    let m = dk.decrypt(c);
    assert_eq!(m, [128u8; 32]);
}

#[cfg(test)]
mod tests {
    use crate::encrypt::EncryptionKey;
    use crate::field::compress_1;
    use crate::params::{DU, DV, HALF_Q_UP, HALF_QU16, Q, RANK};
    use crate::ring::Poly;
    use crate::{keygen, prf, ring};
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use std::path::Path;

    #[cfg(test)]
    pub fn _read_kat_file_(
        path: &Path,
    ) -> std::io::Result<(
        Vec<Vec<u8>>,
        Vec<Vec<u8>>,
        Vec<Vec<u8>>,
        Vec<Vec<u8>>,
        Vec<Vec<u8>>,
        Vec<Vec<u8>>,
    )> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut kat_d: Vec<Vec<u8>> = Vec::new();
        let mut kat_pk: Vec<Vec<u8>> = Vec::new();
        let mut kat_sk: Vec<Vec<u8>> = Vec::new();
        let mut kat_m: Vec<Vec<u8>> = Vec::new();
        let mut kat_ct: Vec<Vec<u8>> = Vec::new();
        let mut kat_ss: Vec<Vec<u8>> = Vec::new();
        for line in reader.lines() {
            let line = line?;
            let line = line.trim();

            if line.is_empty() {
                continue;
            }

            let (key, value) = line.split_once('=').unwrap();
            let key = key.trim().to_string();
            let value = value.trim().to_string();

            match key.as_str() {
                "d" => kat_d.push(hex::decode(value).unwrap()),
                "pk" => kat_pk.push(hex::decode(value).unwrap()),
                "sk" => kat_sk.push(hex::decode(value).unwrap()),
                "m" => kat_m.push(hex::decode(value).unwrap()),
                "ct" => kat_ct.push(hex::decode(value).unwrap()),
                "ss" => kat_ss.push(hex::decode(value).unwrap()),
                _ => {}
            }
        }
        Ok((kat_d, kat_pk, kat_sk, kat_m, kat_ct, kat_ss))
    }

    #[cfg(test)]
    fn _run_kats_(kat_file: &Path) {
        let (d, kat_pk, kat_sk, kat_m, kat_ct, kat_ss) = _read_kat_file_(kat_file).unwrap();

        for i in 0..d.len() {
            let dv: [u8; 32] = d[i].as_slice().try_into().unwrap();
            let exp_pk = &kat_pk[i];
            let (pk, dk, _) = keygen::key_gen(dv);
            assert_eq!(pk.key_bytes(), exp_pk);
            let exp_sk = &kat_sk[i];
            assert_eq!(dk.key_bytes(), &exp_sk[0..384 * RANK]);

            let mut hash_ek = [0u8; 32];
            prf::sha3_256(&pk.key_bytes(), &mut hash_ek); // H(ek)

            let mut m_h_ek: [u8; 64] = [0u8; 64];
            let m: [u8; 32] = kat_m[i].as_slice().try_into().unwrap();
            m_h_ek[0..32].copy_from_slice(&m);
            m_h_ek[32..].copy_from_slice(&hash_ek);

            let mut hash = [0u8; 64];
            prf::sha3_512(&m_h_ek, &mut hash); // G(m||H(ek))
            let r: [u8; 32] = hash[32..].try_into().unwrap();

            let ct = pk.encrypt(m.into(), r);
            let exp_ct: [u8; 32usize * (DU as usize * RANK + DV as usize)] =
                kat_ct[i].as_slice().try_into().unwrap();
            assert_eq!(ct, exp_ct);

            let ss: [u8; 32] = hash[0..32].try_into().unwrap();
            let exp_ss: [u8; 32] = kat_ss[i].as_slice().try_into().unwrap();
            assert_eq!(ss, exp_ss);

            let md = dk.decrypt(exp_ct);
            assert_eq!(md, m);
            let ct_dash = pk.encrypt(md.into(), r);
            assert_eq!(ct_dash.len(), ct.len());
            assert_eq!(ct_dash, ct);
        }
    }

    #[cfg(feature = "ML_KEM_512")]
    #[test]
    fn mlkem512_run_nist_kats() {
        _run_kats_("nist-kats/ml_kem_512.kat".as_ref())
    }

    #[cfg(feature = "ML_KEM_1024")]
    #[test]
    fn mlkem1024_run_nist_kats() {
        _run_kats_("nist-kats/ml_kem_1024.kat".as_ref())
    }

    #[cfg(feature = "ML_KEM_768")]
    #[test]
    fn mlkem768_run_nist_kats() {
        _run_kats_("nist-kats/ml_kem_768.kat".as_ref())
    }

    #[test]
    fn test_generic_main() {
        for _ in 0..128 {
            let mut d: [u8; 32] = [0u8; 32];
            getrandom::fill(&mut d).expect("random bytes");
            let (pk, dk, _) = keygen::key_gen(d);
            let mut r: [u8; 32] = [0u8; 32];
            getrandom::fill(&mut r).expect("random bytes");
            let mut m: [u8; 32] = [0u8; 32];
            getrandom::fill(&mut m).expect("random bytes");
            let c: [u8; 32 * (DU * RANK as u8 + DV) as usize] = pk.encrypt(m, r);
            let md = dk.decrypt(c);
            assert_eq!(md, m);
        }
    }

    #[test]
    fn test_message_reconstruction() {
        for _i in 0..2048 {
            let mut d: [u8; 32] = [0u8; 32];
            getrandom::fill(&mut d).expect("random bytes");
            let (_rho, sigma) = keygen::sample_rho_sigma(d);
            let (s, n) = keygen::sample_s(sigma);
            let (e, _) = keygen::sample_e(sigma, n);

            let mut r: [u8; 32] = [0u8; 32];
            getrandom::fill(&mut r).expect("random bytes");

            let (y, n) = EncryptionKey::sample_y(r);
            let (e1, n) = EncryptionKey::sample_e1(r, n);
            let e2 = EncryptionKey::sample_e2(r, n);

            let mut m: [u8; 32] = [0u8; 32];
            getrandom::fill(&mut m).expect("random bytes");
            let mu: Poly = EncryptionKey::decode_then_decompress_message(m);

            let se1: Poly = ring::inner_product(s, e1);
            let ey: Poly = ring::inner_product(e, y);
            let msg_with_err: Poly = e2.add(&mu).add(&se1).add(&ey);
            let md: [u8; 32] = msg_with_err.compress_1().byte_encode_1();
            assert_eq!(m, md);
        }
    }

    #[test]
    fn test_one_bit_reconstruction() {
        let _compressed_hq_ = compress_1(HALF_Q_UP);
        assert_eq!(1, _compressed_hq_);
        for err_x in 0..Q as u16 / 4 {
            let y = compress_1(HALF_Q_UP + err_x);
            assert_eq!(_compressed_hq_, y);
        }
        let y = compress_1(HALF_Q_UP + Q as u16 / 4 + 1);
        assert_eq!(0, y);
        assert_ne!(_compressed_hq_, y);
        for err_x in Q as u16 / 4 + 1..HALF_QU16 as u16 {
            let y = compress_1(HALF_Q_UP + err_x);
            assert_ne!(_compressed_hq_, y);
        }
    }
}
