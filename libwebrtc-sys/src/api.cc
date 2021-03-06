#include "libwebrtc-sys/include/api.h"
#include "libwebrtc-sys/src/api.rs.h"
#include "libwebrtc-sys/src/shared_bridge.rs.h"

std::unique_ptr<ArcasAPI> create_arcas_api()
{
    return std::make_unique<ArcasAPI>();
}
