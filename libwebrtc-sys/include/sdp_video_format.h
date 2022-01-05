#pragma once
#include "api/video_codecs/sdp_video_format.h"
#include "libwebrtc-sys/include/rust_shared.h"
#include "rust/cxx.h"

rust::Vec<ArcasRustDict> sdp_video_format_get_parameters(const webrtc::SdpVideoFormat& format);
const std::string&       sdp_video_format_get_name(const webrtc::SdpVideoFormat& format);
rust::String             sdp_video_format_to_string(const webrtc::SdpVideoFormat& format);

// Helper utilities to return C++ types back into rust with shared structs.

// This is a generic helper method but was specific created to support:
//
// VideoEncoderFactory::GetSupportedFormats
//
std::unique_ptr<std::vector<webrtc::SdpVideoFormat>>
create_sdp_video_format_list(ArcasSdpVideoFormatVecInit list);
std::unique_ptr<webrtc::SdpVideoFormat> create_sdp_video_format(ArcasSdpVideoFormatInit init);
