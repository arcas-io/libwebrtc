#include "libwebrtc-sys/include/rtp_receiver.h"

ArcasRTPReceiver::ArcasRTPReceiver(rtc::scoped_refptr<webrtc::RtpReceiverInterface> api) : api(api){};