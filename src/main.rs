use crate::field::FieldElement;
use crate::params::{DU, Q};

mod codec;
mod decrypt;
mod encrypt;
mod field;
mod ntt;
mod params;
mod pke;
mod prf;
mod ring;
mod sampler;

fn main() {
    assert!(DU < 12);
    for x in 0..Q {
        let t = FieldElement::from(x as i32).compress::<DU>();
        assert!(u16::from(t) <= (1 << DU) - 1)
    }
}

#[cfg(test)]
mod tests {
    use crate::params::{DU, DV, RANK};
    use crate::{pke, prf};
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use std::path::Path;

    #[cfg(test)]
    pub fn read_kat_file<P: AsRef<Path>>(
        path: P,
        kat_d: &mut Vec<Vec<u8>>,
        kat_pk: &mut Vec<Vec<u8>>,
        kat_sk: &mut Vec<Vec<u8>>,
        kat_m: &mut Vec<Vec<u8>>,
        kat_ct: &mut Vec<Vec<u8>>,
        kat_ss: &mut Vec<Vec<u8>>,
    ) -> std::io::Result<()> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

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

        Ok(())
    }

    #[cfg(feature = "ML_KEM_512")]
    #[test]
    fn mlkem512_run_nist_kats() {
        let mut d: Vec<Vec<u8>> = Vec::new();
        let mut pk: Vec<Vec<u8>> = Vec::new();
        let mut sk: Vec<Vec<u8>> = Vec::new();
        let mut kat_m: Vec<Vec<u8>> = Vec::new();
        let mut exp_ct: Vec<Vec<u8>> = Vec::new();
        let mut exp_ss: Vec<Vec<u8>> = Vec::new();
        read_kat_file(
            "nist-kats/ml_kem_512.kat",
            &mut d,
            &mut pk,
            &mut sk,
            &mut kat_m,
            &mut exp_ct,
            &mut exp_ss,
        )
        .unwrap();

        for i in 0..d.len() {
            let dv: [u8; 32] = d[i].as_slice().try_into().unwrap();
            let exp_pk: [u8; 800] = pk[i].as_slice().try_into().unwrap();
            let exp_sk: [u8; 1632] = sk[i].as_slice().try_into().unwrap();
            let (pk, dk, _) = pke::key_gen(dv);
            assert_eq!(pk.key_bytes(), exp_pk);

            assert_eq!(dk.key_bytes(), &exp_sk[0..384 * RANK]);
            let m: [u8; 32] = kat_m[i].as_slice().try_into().unwrap();

            let mut hash_ek = [0u8; 32];
            prf::sha3_256(&pk.key_bytes(), &mut hash_ek); // H(ek)
            let mut m_h_ek: [u8; 64] = [0u8; 64];
            m_h_ek[0..32].copy_from_slice(&m);
            m_h_ek[32..].copy_from_slice(&hash_ek);

            let mut hash = [0u8; 64];
            prf::sha3_512(&m_h_ek, &mut hash); // G(m||H(ek))
            let r: [u8; 32] = hash[32..].try_into().unwrap();

            let ct = pk.encrypt(m.into(), r);
            let _ct: [u8; 32usize * (DU as usize * RANK + DV as usize)] =
                exp_ct[i].as_slice().try_into().unwrap();
            assert_eq!(ct, _ct, "kat index {i}");

            let ss: [u8; 32] = hash[0..32].try_into().unwrap();
            let _ss: [u8; 32] = exp_ss[i].as_slice().try_into().unwrap();
            assert_eq!(ss, _ss);

            let md = dk.decrypt(_ct);
            assert_eq!(md, m);
            let ct_dash = pk.encrypt(md.into(), r);
            assert_eq!(ct_dash.len(), ct.len());
            assert_eq!(ct_dash, ct);
        }
    }

    #[cfg(feature = "ML_KEM_1024")]
    #[test]
    fn mlkem1024_run_nist_kats() {
        let mut d: Vec<Vec<u8>> = Vec::new();
        let mut pk: Vec<Vec<u8>> = Vec::new();
        let mut sk: Vec<Vec<u8>> = Vec::new();
        let mut kat_m: Vec<Vec<u8>> = Vec::new();
        let mut exp_ct: Vec<Vec<u8>> = Vec::new();
        let mut exp_ss: Vec<Vec<u8>> = Vec::new();
        read_kat_file(
            "nist-kats/ml_kem_1024.kat",
            &mut d,
            &mut pk,
            &mut sk,
            &mut kat_m,
            &mut exp_ct,
            &mut exp_ss,
        )
        .unwrap();

        for i in 0..d.len() {
            let dv: [u8; 32] = d[i].as_slice().try_into().unwrap();
            let exp_pk: [u8; 1568] = pk[i].as_slice().try_into().unwrap();
            let exp_sk: [u8; 3168] = sk[i].as_slice().try_into().unwrap();
            let (pk, dk, _) = pke::key_gen(dv);
            assert_eq!(pk.key_bytes(), exp_pk);

            assert_eq!(dk.key_bytes(), &exp_sk[0..384 * RANK]);
            let m: [u8; 32] = kat_m[i].as_slice().try_into().unwrap();

            let mut hash_ek = [0u8; 32];
            prf::sha3_256(&pk.key_bytes(), &mut hash_ek); // H(ek)
            let mut m_h_ek: [u8; 64] = [0u8; 64];
            m_h_ek[0..32].copy_from_slice(&m);
            m_h_ek[32..].copy_from_slice(&hash_ek);

            let mut hash = [0u8; 64];
            prf::sha3_512(&m_h_ek, &mut hash); // G(m||H(ek))
            let r: [u8; 32] = hash[32..].try_into().unwrap();

            let ct = pk.encrypt(m.into(), r);
            let _ct: [u8; 32usize * (DU as usize * RANK + DV as usize)] =
                exp_ct[i].as_slice().try_into().unwrap();
            assert_eq!(ct, _ct);

            let ss: [u8; 32] = hash[0..32].try_into().unwrap();
            let _ss: [u8; 32] = exp_ss[i].as_slice().try_into().unwrap();
            assert_eq!(ss, _ss);

            let md = dk.decrypt(_ct);
            assert_eq!(md, m);
            let ct_dash = pk.encrypt(md.into(), r);
            assert_eq!(ct_dash.len(), ct.len());
            assert_eq!(ct_dash, ct);
        }
    }

    #[cfg(feature = "ML_KEM_768")]
    #[test]
    fn mlkem768_run_nist_kats() {
        let mut d: Vec<Vec<u8>> = Vec::new();
        let mut pk: Vec<Vec<u8>> = Vec::new();
        let mut sk: Vec<Vec<u8>> = Vec::new();
        let mut kat_m: Vec<Vec<u8>> = Vec::new();
        let mut exp_ct: Vec<Vec<u8>> = Vec::new();
        let mut exp_ss: Vec<Vec<u8>> = Vec::new();
        read_kat_file(
            "nist-kats/ml_kem_768.kat",
            &mut d,
            &mut pk,
            &mut sk,
            &mut kat_m,
            &mut exp_ct,
            &mut exp_ss,
        )
        .unwrap();

        for i in 0..d.len() {
            let dv: [u8; 32] = d[i].as_slice().try_into().unwrap();
            let exp_pk: [u8; 1184] = pk[i].as_slice().try_into().unwrap();
            let exp_sk: [u8; 2400] = sk[i].as_slice().try_into().unwrap();
            let (pk, dk, _) = pke::key_gen(dv);
            assert_eq!(pk.key_bytes(), exp_pk);

            assert_eq!(dk.key_bytes(), &exp_sk[0..384 * RANK]);
            let m: [u8; 32] = kat_m[i].as_slice().try_into().unwrap();

            let mut hash_ek = [0u8; 32];
            prf::sha3_256(&pk.key_bytes(), &mut hash_ek); // H(ek)
            let mut m_h_ek: [u8; 64] = [0u8; 64];
            m_h_ek[0..32].copy_from_slice(&m);
            m_h_ek[32..].copy_from_slice(&hash_ek);

            let mut hash = [0u8; 64];
            prf::sha3_512(&m_h_ek, &mut hash); // G(m||H(ek))
            let r: [u8; 32] = hash[32..].try_into().unwrap();

            let ct = pk.encrypt(m.into(), r);
            let _ct: [u8; 32usize * (DU as usize * RANK + DV as usize)] =
                exp_ct[i].as_slice().try_into().unwrap();
            assert_eq!(ct, _ct);

            let ss: [u8; 32] = hash[0..32].try_into().unwrap();
            let _ss: [u8; 32] = exp_ss[i].as_slice().try_into().unwrap();
            assert_eq!(ss, _ss);

            let md = dk.decrypt(_ct);
            assert_eq!(md, m);
            let ct_dash = pk.encrypt(md.into(), r);
            assert_eq!(ct_dash.len(), ct.len());
            assert_eq!(ct_dash, ct);
        }
    }
}
