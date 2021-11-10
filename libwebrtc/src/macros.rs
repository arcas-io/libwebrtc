use log::error;

use crate::error::WebRTCError;

pub fn log_webrtc_error(err: WebRTCError) {
    error!("Unhandled WebRTCError: {}", err.to_string());
}

#[macro_export]
macro_rules! ok_or_return {
    ($result:expr) => {
        match $result {
            Ok(value) => value,
            Err(error) => {
                crate::macros::log_webrtc_error(error.into());
                return;
            }
        }
    };
}
