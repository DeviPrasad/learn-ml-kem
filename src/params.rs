pub const Q: u32 = 3329; // ML-KEM prime. 2^8 * 13 + 1
pub const QI32: i32 = Q as i32;
pub const QU32: u32 = Q as u32;
pub const QI64: i64 = Q as i64;

pub const N: usize = 256; // ML-KEM prime. 2^8 * 13 + 1
pub const HALF_Q: u32 = 1664; // ML-KEM prime. 2^8 * 13 + 1
pub const HALF_Q_UP: u16 = ((Q + 1)/2) as u16;
pub const BARRETT_MULTIPLIER_24: i64 = 5039; // 2^12 * 2^12 / q
pub const BARRETT_MULTIPLIER_32: i64 = 1290167; // 2^32 / q
pub const BARRETT_SHIFT_24: u64 = 24; // log₂(2^24)
pub const BARRETT_SHIFT_32: i32 = 32; // log₂(2^24)

#[cfg(feature = "ML_KEM_512")]
#[allow(unused)]
pub const RANK: usize = 2;
#[cfg(feature = "ML_KEM_512")]
#[allow(unused)]
pub const ETA1: u8 = 3;
#[cfg(feature = "ML_KEM_512")]
#[allow(unused)]
pub const ETA2: u8 = 2;
#[cfg(feature = "ML_KEM_512")]
#[allow(unused)]
pub const DU: u8 = 10;
#[cfg(feature = "ML_KEM_512")]
#[allow(unused)]
pub const DV: u8 = 4;

#[cfg(feature = "ML_KEM_768")]
#[allow(unused)]
pub const RANK: usize = 3;
#[cfg(feature = "ML_KEM_768")]
#[allow(unused)]
pub const ETA1: u8 = 2;
#[cfg(feature = "ML_KEM_768")]
#[allow(unused)]
pub const ETA2: u8 = 2;
#[cfg(feature = "ML_KEM_768")]
#[allow(unused)]
pub const DU: u8 = 10;
#[cfg(feature = "ML_KEM_768")]
#[allow(unused)]
pub const DV: u8 = 4;

#[cfg(feature = "ML_KEM_1024")]
#[allow(unused)]
pub const RANK: usize = 4;
#[cfg(feature = "ML_KEM_1024")]
#[allow(unused)]
pub const ETA1: u8 = 2;
#[cfg(feature = "ML_KEM_1024")]
#[allow(unused)]
pub const ETA2: u8 = 2;
#[cfg(feature = "ML_KEM_1024")]
#[allow(unused)]
pub const DU: u8 = 11;
#[cfg(feature = "ML_KEM_1024")]
#[allow(unused)]
pub const DV: u8 = 5;
