#include "libwebrtc-sys/include/color_space.h"

std::unique_ptr<ArcasColorSpace> create_arcas_color_space()
{
    return std::make_unique<ArcasColorSpace>();
}
