use std::time::{SystemTime, UNIX_EPOCH};

/// A byte array of length 32
pub type Bytes = [u8; 32];
/// A timestamp in microseconds
pub type Timestamp = u64;

/// Generate a random bytes array of length 32
pub fn bytes() -> Bytes {
    let mut bytes = [0; 32];
    getrandom::fill(&mut bytes).unwrap();
    bytes
}

pub fn time() -> Timestamp {
    let now = SystemTime::now();
    now.duration_since(UNIX_EPOCH).unwrap().as_micros() as Timestamp
}
