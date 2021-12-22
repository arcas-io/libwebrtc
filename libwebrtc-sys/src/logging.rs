#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("include/logging.h");

        #[namespace = "rtc"]
        type LoggingSeverity = crate::shared_bridge::ffi::LoggingSeverity;

        // Logging
        fn set_arcas_log_to_stderr(log: bool);
        fn set_arcas_log_level(level: LoggingSeverity);
    }
}
