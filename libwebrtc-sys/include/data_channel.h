#pragma once
#include "api/data_channel_interface.h"

class ArcasDataChannel
{
private:
    rtc::scoped_refptr<webrtc::DataChannelInterface> api;

public:
    ArcasDataChannel(rtc::scoped_refptr<webrtc::DataChannelInterface> api);
};

std::unique_ptr<ArcasDataChannel> gen_unique_data_channel();
