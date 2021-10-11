use std::fmt;

use cxx::CxxString;
use cxx::UniquePtr;
#[cxx::bridge]
mod ffi {

    struct FactoryConfig {
        name: String,
    }

    unsafe extern "C++" {
        include!("libwebrtc-sys/include/webrtc.h");

        type ArcasWebRTC;
        type ArcasPeerConnectionFactory;

        fn createFactory(self: &ArcasWebRTC) -> UniquePtr<ArcasPeerConnectionFactory>;

        fn createWebRTC() -> UniquePtr<ArcasWebRTC>;
    }
}

impl fmt::Debug for ffi::ArcasWebRTC {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        /// foo
        f.debug_struct("ArcasWebRTC").finish()
    }
}

#[cfg(test)]
mod tests {
    use crate::ffi;

    #[test]
    fn test_ffi() {
        let cfg = ffi::FactoryConfig {
            name: "nutbar".to_owned(),
        };

        {
            let webrtc = ffi::createWebRTC();
            let fac = webrtc.createFactory();
        }
    }
}
