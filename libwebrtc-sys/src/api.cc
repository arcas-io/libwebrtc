#include "libwebrtc-sys/include/api.h"

std::unique_ptr<ArcasAPI> create_arcas_api()
{
    return std::make_unique<ArcasAPI>();
}