#pragma once
#include "libwebrtc-sys/include/webrtc_api.h"

// Utility class to implement "ref counting" for an object that's held by a
// unique ptr.  This is glue code that shouldn't be used unless the object
// implementing it is carefully managed.
class ArcasRefCounted : public rtc::RefCountInterface
{

    void AddRef() const {}
    rtc::RefCountReleaseStatus Release()
    {
        return rtc::RefCountReleaseStatus::kDroppedLastRef;
    }
};