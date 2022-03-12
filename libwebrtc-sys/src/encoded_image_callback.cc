#include "encoded_image_callback.h"
#include "libwebrtc-sys/src/shared_bridge.rs.h"

ArcasEncodedImageCallbackResult ArcasEncodedImageCallback::on_encoded_image(const webrtc::EncodedImage& image,
                                                                            const ArcasCodecSpecificInfo* codec_specific_info) const
{
    auto result = api->OnEncodedImage(image, codec_specific_info->as_ptr());
    return ArcasEncodedImageCallbackResult{
        .error = result.error,
        .frame_id = result.frame_id,
        .drop_next_frame = result.drop_next_frame,
    };
}
