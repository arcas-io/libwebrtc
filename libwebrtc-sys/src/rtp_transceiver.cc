#include "libwebrtc-sys/include/rtp_transceiver.h"

ArcasRTPTransceiver::ArcasRTPTransceiver(
    rtc::scoped_refptr<webrtc::RtpTransceiverInterface> api) : api(api){};