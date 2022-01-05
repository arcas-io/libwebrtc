#pragma once
#include "api/video/color_space.h"

class ArcasColorSpace
{
private:
    webrtc::ColorSpace color_space_;

public:
    ArcasColorSpace()
    : color_space_(webrtc::ColorSpace::PrimaryID::kBT709,
                   webrtc::ColorSpace::TransferID::kBT709,
                   webrtc::ColorSpace::MatrixID::kBT709,
                   webrtc::ColorSpace::RangeID::kFull)
    {
    }
    const webrtc::ColorSpace* as_ptr() const
    {
        return &color_space_;
    }
};

std::unique_ptr<ArcasColorSpace> create_arcas_color_space();
