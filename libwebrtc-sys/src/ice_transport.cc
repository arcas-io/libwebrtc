#include "libwebrtc-sys/include/ice_transport.h"

std::unique_ptr<webrtc::IceTransportFactory> create_arcas_cxx_ice_transport_factory()
{
    return std::make_unique<webrtc::DefaultIceTransportFactory>();
}
