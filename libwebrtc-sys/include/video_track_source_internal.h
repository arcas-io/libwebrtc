#pragma once
#include "libwebrtc-sys/include/webrtc_api.h"
#include "pc/video_track_source.h"
#include "api/video/i420_buffer.h"

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
    ArcasVideoTrackSourceInternal() : webrtc::VideoTrackSource(true)
    {
        SetState(webrtc::MediaSourceInterface::kLive);
    };
    ~ArcasVideoTrackSourceInternal()
    {
        RTC_LOG(LS_VERBOSE) << "~ArcasVideoTrackSourceInternal";
    };

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

    void push_i420_data(int32_t width,
                        int32_t height,
                        int32_t stride_y,
                        int32_t stride_u,
                        int32_t stride_v,
                        const uint8_t *data)
    {
        // RTC_DCHECK(track_source);
        // RTC_LOG(LS_INFO) << "creating video_frame_buffer";
        int y_offset = 0, u_offset = stride_y * height;
        int v_offset = u_offset + stride_u * (height / 2);
        auto frame_buffer = webrtc::I420Buffer::Copy(
            width,
            height,
            data + y_offset,
            stride_y,
            data + u_offset,
            stride_u,
            data + v_offset,
            stride_v);

        auto builder = webrtc::VideoFrame::Builder();
        builder.set_video_frame_buffer(frame_buffer);
        auto frame = builder.build();

        // RTC_LOG(LS_VERBOSE) << "copied video_frame_buffer";
        broadcaster.OnFrame(frame);
    }
};
