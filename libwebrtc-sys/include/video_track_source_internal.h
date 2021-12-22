#pragma once
#include "libwebrtc-sys/include/video_frame_internal.h"
#include "media/base/video_broadcaster.h"
#include "pc/video_track_source.h"
#include "api/video/i420_buffer.h"
#include <chrono>

class ArcasVideoTrackSourceInternal : public rtc::RefCountedBase, public webrtc::VideoTrackSource
{
private:
    rtc::VideoBroadcaster broadcaster;

protected:
    rtc::VideoSourceInterface<webrtc::VideoFrame> *source() override
    {
        return nullptr;
    }

public:
    /* remote=true this was picked at random */
    ArcasVideoTrackSourceInternal() : webrtc::VideoTrackSource(false)
    {
        SetState(webrtc::MediaSourceInterface::kLive);
    };
    ~ArcasVideoTrackSourceInternal()
    {
        RTC_LOG(LS_VERBOSE) << "~ArcasVideoTrackSourceInternal";
    };

    webrtc::MediaSourceInterface::SourceState state() const override
    {
        return webrtc::MediaSourceInterface::kLive;
    }

    void AddRef() const override
    {

        rtc::RefCountedBase::AddRef();
    }

    rtc::RefCountReleaseStatus Release() const override
    {
        return rtc::RefCountedBase::Release();
    }

    void RemoveSink(
        rtc::VideoSinkInterface<webrtc::VideoFrame> *sink) override
    {
        RTC_LOG(LS_VERBOSE) << "RemoveSink for track source internal";
        broadcaster.RemoveSink(sink);
    }

    void AddOrUpdateSink(
        rtc::VideoSinkInterface<webrtc::VideoFrame> *sink,
        const rtc::VideoSinkWants &wants) override
    {
        RTC_LOG(LS_VERBOSE) << "AddOrUpdateSink for track source internal";
        broadcaster.AddOrUpdateSink(sink, wants);
    }

    void push_frame(const webrtc::VideoFrame &frame)
    {
        broadcaster.OnFrame(frame);
    }
};
