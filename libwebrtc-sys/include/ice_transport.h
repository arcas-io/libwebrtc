#pragma once
#include "api/ice_transport_interface.h"
#include "p2p/base/default_ice_transport_factory.h"

std::unique_ptr<webrtc::IceTransportFactory> create_arcas_cxx_ice_transport_factory();
