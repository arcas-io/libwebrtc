#pragma once
#include "rust/cxx.h"
#include "libwebrtc-sys/include/alias.h"
#include "libwebrtc-sys/include/webrtc_api.h"

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