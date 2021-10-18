#include "iostream"
#include "rust/cxx.h"
#include "libwebrtc-sys/include/utils.h"
#include "libwebrtc-sys/src/lib.rs.h"

rust::String session_description_to_string(const webrtc::SessionDescriptionInterface &sdp)
{
    std::string output;
    sdp.ToString(&output);
    // XXX: For some reason we must cast to c_str first.
    rust::String rust_str(output.c_str());
    return rust_str;
}