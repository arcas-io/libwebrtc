#include "libwebrtc-sys/include/stun.h"

std::unique_ptr<ArcasICEMessage> create_arcas_ice_message()
{
    return std::make_unique<ArcasICEMessage>();
}
