#include "libwebrtc-sys/include/p2p/ice_transport_internal.h"

std::unique_ptr<ArcasP2PIceConfig> create_arcas_p2p_ice_config()
{
    return std::make_unique<ArcasP2PIceConfig>();
}
