
#include "libwebrtc-sys/include/data_channel.h"

ArcasDataChannel::ArcasDataChannel(rtc::scoped_refptr<webrtc::DataChannelInterface> api) : api(api) {}