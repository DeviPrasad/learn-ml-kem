use crate::params::RANK;

#[allow(unused)]
pub struct DecryptionKey {
    key: [u8; 384*RANK],
}

#[allow(unused)]
impl DecryptionKey {
    pub fn new(key: [u8; 384*RANK]) -> Self {
        DecryptionKey { key }
    }

    pub(crate) fn key_bytes(&self) -> (&[u8]) {
        &self.key
    }
}
