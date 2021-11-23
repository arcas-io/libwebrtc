use std::time::SystemTime;

use crate::error::{Result, WebRTCError};

pub fn now() -> Result<u64> {
    let time_ms = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map_err(|_| WebRTCError::FailedToGetSystemTime)?;
    Ok(time_ms.as_secs() * 1000 + time_ms.subsec_nanos() as u64 / 1_000_000)
}
