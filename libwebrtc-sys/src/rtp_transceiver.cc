#include "libwebrtc-sys/include/rtp_transceiver.h"

ArcasRTPTransceiver::ArcasRTPTransceiver(
    rtc::scoped_refptr<webrtc::RtpTransceiverInterface> api) : api(api){};

std::unique_ptr<ArcasRTPVideoTransceiver> video_transceiver_from_base(const ArcasRTPTransceiver& transceiver)
{
    return std::make_unique<ArcasRTPVideoTransceiver>(transceiver.api);
}

std::unique_ptr<ArcasRTPAudioTransceiver> audio_transceiver_from_base(const ArcasRTPTransceiver& transceiver)
{
    return std::make_unique<ArcasRTPAudioTransceiver>(transceiver.api);
}
