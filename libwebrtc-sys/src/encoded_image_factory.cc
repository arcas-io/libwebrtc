#include "libwebrtc-sys/include/encoded_image_factory.h"

std::unique_ptr<ArcasEncodedImageFactory> create_arcas_encoded_image_factory()
{
    return std::make_unique<ArcasEncodedImageFactory>();
}