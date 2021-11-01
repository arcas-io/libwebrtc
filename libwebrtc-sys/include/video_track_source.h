#pragma once
#include "libwebrtc-sys/include/webrtc_api.h"
#include "pc/video_track_source.h"
#include "api/video/i420_buffer.h"

class ArcasVideoTrackSource : public rtc::RefCountedBase, public webrtc::Notifier<webrtc::VideoTrackSourceInterface>
{
private:
    rtc::VideoBroadcaster broadcaster;
    webrtc::MediaSourceInterface::SourceState state_;
    const bool remote_;

protected:
    rtc::VideoSourceInterface<webrtc::VideoFrame> *source()
    {
        return nullptr;
    }

public:
    /* remote=true this was picked at random */
    ArcasVideoTrackSource() : remote_(true){};
    ~ArcasVideoTrackSource()
    {
        RTC_LOG(LS_VERBOSE) << "~ArcasVideoTrackSource";
    };

    void AddRef() const override
    {
        // Typically we'd want to use the RefCountedBase::AddRef() method but
        // here we're expecting rust to be the _only_ owner of this object.
    }

    rtc::RefCountReleaseStatus Release() const override
    {
        // Without a call into the super class this simply never frees the memory from this refcount interface.
        return rtc::RefCountReleaseStatus::kDroppedLastRef;
    }

    void SetState(webrtc::MediaSourceInterface::SourceState new_state)
    {
        state_ = new_state;
        // XXX: Do we need an observer from rust to notify?
    }

    webrtc::MediaSourceInterface::SourceState state() const override { return state_; }
    bool remote() const override { return remote_; }

    bool is_screencast() const override { return false; }
    absl::optional<bool> needs_denoising() const override
    {
        return absl::nullopt;
    }

    bool GetStats(Stats *stats) override { return false; }

    bool SupportsEncodedOutput() const override { return false; }
    void GenerateKeyFrame() override {}
    void AddEncodedSink(
        rtc::VideoSinkInterface<webrtc::RecordableEncodedFrame> *sink) override {}
    void RemoveEncodedSink(
        rtc::VideoSinkInterface<webrtc::RecordableEncodedFrame> *sink) override {}

    void RemoveSink(
        rtc::VideoSinkInterface<webrtc::VideoFrame> *sink) override
    {
        broadcaster.RemoveSink(sink);
    }

    void AddOrUpdateSink(
        rtc::VideoSinkInterface<webrtc::VideoFrame> *sink,
        const rtc::VideoSinkWants &wants) override
    {
        broadcaster.AddOrUpdateSink(sink, wants);
    }

    void push_i420_data(int32_t width,
                        int32_t height,
                        int32_t stride_y,
                        int32_t stride_u,
                        int32_t stride_v,
                        uint8_t *data)
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

std::shared_ptr<ArcasVideoTrackSource> create_arcas_video_track_source();
void push_i420_to_video_track_source(
    std::shared_ptr<ArcasVideoTrackSource> source,
    int32_t width,
    int32_t height,
    int32_t stride_y,
    int32_t stride_u,
    int32_t stride_v,
    uint8_t *data);