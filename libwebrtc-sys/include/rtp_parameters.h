#pragma once
#include "api/rtp_parameters.h"
#include "libwebrtc-sys/include/alias.h"
#include "rust/cxx.h"

class ArcasRTPCodecCapability
{
private:
    webrtc::RtpCodecCapability api;

public:
    ArcasRTPCodecCapability(webrtc::RtpCodecCapability api) : api(api) {}

    webrtc::RtpCodecCapability get() const { return this->api; }

    rust::String mime_type() const { return rust::String(api.mime_type().c_str()); }
    rust::String name() const { return rust::String(api.name.c_str()); }
    ArcasMediaType kind() const { return api.kind; }
};

class ArcasRTPHeaderExtensionCapability
{
private:
    webrtc::RtpHeaderExtensionCapability api;

public:
    ArcasRTPHeaderExtensionCapability(webrtc::RtpHeaderExtensionCapability api) : api(api) {}
    webrtc::RtpHeaderExtensionCapability get() const { return this->api; }

    rust::String get_uri() const { return rust::String(api.uri.c_str()); }
};

std::unique_ptr<std::vector<ArcasRTPHeaderExtensionCapability>> gen_unique_vector_rtp_header_extension_capabilities();
std::unique_ptr<std::vector<ArcasRTPCodecCapability>> gen_unique_vector_rtp_codec_capabilities();
