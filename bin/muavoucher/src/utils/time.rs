use std::time::{SystemTime, UNIX_EPOCH};

pub fn now_ms() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).expect("Should ok").as_millis() as u64
}

pub fn now() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).expect("Should ok").as_secs()
}
