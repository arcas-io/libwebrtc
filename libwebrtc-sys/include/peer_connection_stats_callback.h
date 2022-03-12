#pragma once
#include "api/ref_counted_base.h"
#include "api/stats/rtc_stats_collector_callback.h"
#include "rust/cxx.h"
#include "rust_shared.h"

class ArcasRTCStatsCollectorCallback : public webrtc::RTCStatsCollectorCallback, public rtc::RefCountedBase
{
public:
    using underlying_t = rust::Box<ArcasRustRTCStatsCollectorCallback>;
    ArcasRTCStatsCollectorCallback(underlying_t cb, short expected_calls = 1)
    : expected_calls_back{expected_calls}
    , cb(std::move(cb))
    {
    }

    virtual ~ArcasRTCStatsCollectorCallback() {}

    void OnStatsDelivered(const rtc::scoped_refptr<const webrtc::RTCStatsReport>&) override;

    void AddRef() const override
    {
        rtc::RefCountedBase::AddRef();
    }

    rtc::RefCountReleaseStatus Release() const override
    {
        return rtc::RefCountedBase::Release();
    }

private:
    short expected_calls_back = 1;
    std::vector<rtc::scoped_refptr<const webrtc::RTCStatsReport>> stat_reports;
    underlying_t cb;

    void finish();
};
