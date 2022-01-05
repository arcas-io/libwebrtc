#pragma once
#include "api/rtc_error.h"
#include "rust/cxx.h"

class ArcasRTCError
{
private:
    webrtc::RTCError error_;
    rust::String     message_;

public:
    ArcasRTCError(webrtc::RTCError error)
    : error_(error)
    , message_(error.message())
    {
    }

    bool ok() const
    {
        return error_.ok();
    }

    webrtc::RTCErrorType kind() const
    {
        return error_.type();
    }

    webrtc::RTCErrorDetailType detailed_type() const
    {
        return error_.error_detail();
    }

    rust::String message() const
    {
        return message_;
    }
};

std::unique_ptr<ArcasRTCError> gen_unique_ptr_error();
