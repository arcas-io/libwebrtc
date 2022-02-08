use log::error;

use crate::error::WebRTCError;

pub fn log_webrtc_error(err: WebRTCError) {
    error!("Unhandled WebRTCError: {}", err.to_string());
}

#[macro_export]
macro_rules! cxx_ref {
    ($result:expr) => {
        match $result.as_ref() {
            Some(value) => Ok(value),
            None => Err(crate::error::WebRTCError::CXXUnwrapError(String::from(
                stringify!($result),
            ))),
        }
    };
}

#[macro_export]
macro_rules! cxx_get_mut {
    ($result:expr) => {
        // XXX: Actually check for null.
        Ok($result.pin_mut().get_unchecked_mut())
    };
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

#[macro_export]
macro_rules! take_or_err {
    ($result:expr) => {
        $result
            .take()
            .ok_or_else(|| WebRTCError::TakeError(String::from(stringify!($result))))
    };
}

#[macro_export]
macro_rules! rx_recv_or_err {
    ($result:expr) => {
        match $result.recv() {
            Some(value) => Ok(value),
            None => Err(WebRTCError::RecvError(String::from(stringify!($result)))),
        }
    };
}

#[macro_export]
macro_rules! rx_recv_async_or_err {
    ($result:expr) => {
        match $result.recv().await {
            Some(value) => Ok(value),
            None => Err(WebRTCError::RecvError(String::from(stringify!($result)))),
        }
    };
}

/// Helper for the optional senders blocking send.
#[macro_export]
macro_rules! send_event {
    ($sender:expr, $val:expr) => {
        match &$sender {
            Some(sender) => crate::ok_or_return!(sender.blocking_send($val)),
            None => {}
        }
    };
}
