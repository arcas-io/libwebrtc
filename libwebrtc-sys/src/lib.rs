use std::fmt;

use cxx::CxxString;
use cxx::UniquePtr;
#[cxx::bridge]
mod ffi {

    unsafe extern "C++" {
        include!("libwebrtc-sys/include/peer_connection_factory.h");
        include!("libwebrtc-sys/include/peer_connection.h");

        type ArcasPeerConnectionFactory;

        fn createFactory() -> UniquePtr<ArcasPeerConnectionFactory>;
    }
}

#[cfg(test)]
mod tests {
    use crate::ffi;

    #[test]
    fn test_ffi() {
        let factory = ffi::createFactory();
        let factory2 = ffi::createFactory();
    }
}
