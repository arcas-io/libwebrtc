#include "libwebrtc-sys/include/webrtc_api.h"
#include "libwebrtc-sys/include/video_track_source.h"
#include "libwebrtc-sys/include/video_track_source_internal.h"

std::unique_ptr<ArcasVideoTrackSource> create_arcas_video_track_source()
{
    auto ref = rtc::make_ref_counted<ArcasVideoTrackSourceInternal>();
    return std::make_unique<ArcasVideoTrackSource>(ref);
}

std::unique_ptr<ArcasVideoFrameEncodedImageData> extract_arcas_video_frame_to_raw_frame_buffer(const webrtc::VideoFrame &frame)
{
    auto ptr = frame.video_frame_buffer();
    return std::make_unique<ArcasVideoFrameEncodedImageData>(ptr);
}

std::shared_ptr<ArcasVideoFrameEncodedImageData> create_arcas_encoded_image(
    const webrtc::EncodedImage &encoded_image, const webrtc::CodecSpecificInfo &codec_specific_info)
{
    auto internal = rtc::make_ref_counted<ArcasVideoFrameEncodedImageDataInternal>(
        encoded_image,
        codec_specific_info);

    return std::make_shared<ArcasVideoFrameEncodedImageData>(internal);
}
