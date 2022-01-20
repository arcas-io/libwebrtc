#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("include/ice_transport.h");

        #[namespace = "webrtc"]
        type IceTransportFactory;

        fn create_arcas_cxx_ice_transport_factory() -> UniquePtr<IceTransportFactory>;
    }
}
