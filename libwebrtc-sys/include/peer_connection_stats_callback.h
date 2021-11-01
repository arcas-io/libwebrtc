#pragma once
#include "rust/cxx.h"
#include "libwebrtc-sys/include/rust_shared.h"
#include "api/stats/rtc_stats_collector_callback.h"
#include "api/ref_counted_base.h"

class ArcasRTCStatsCollectorCallback: public webrtc::RTCStatsCollectorCallback, public rtc::RefCountedBase {
    private:
    rust::Box<ArcasRustRTCStatsCollectorCallback> cb;

    public:
    ArcasRTCStatsCollectorCallback(rust::Box<ArcasRustRTCStatsCollectorCallback>);
    void OnStatsDelivered(const rtc::scoped_refptr<const webrtc::RTCStatsReport>&) override;

    void AddRef() const override {
        rtc::RefCountedBase::AddRef();
    }

    rtc::RefCountReleaseStatus Release() const override {
        return rtc::RefCountedBase::Release();
    }
};

