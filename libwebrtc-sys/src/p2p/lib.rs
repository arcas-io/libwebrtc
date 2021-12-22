#[cxx::bridge]
pub mod api {
    unsafe extern "C++" {
        include!("libwebrtc-sys/include/p2p/ice_transport_internal.h");

        type ArcasP2PCandidate;
        type ArcasP2PIceTransportInternal;
    }
}
