use cxx::UniquePtr;
use libwebrtc_sys::ice_transport::ffi::create_arcas_cxx_ice_transport_factory;

pub struct IceTransportFactory {
    pub(crate) inner: UniquePtr<libwebrtc_sys::ice_transport::ffi::IceTransportFactory>,
}

impl Default for IceTransportFactory {
    fn default() -> Self {
        IceTransportFactory {
            inner: create_arcas_cxx_ice_transport_factory(),
        }
    }
}
