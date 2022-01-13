#pragma once
#include "api/media_stream_interface.h"
#include "audio_track_source_internal.h"
#include "rust/cxx.h"

class ArcasAudioTrackSource
{
public:
    ArcasAudioTrackSource();
    void push_raw_s16be(rust::Vec<uint8_t> audio_data,
                        int sample_rate,
                        size_t number_of_channels,
                        size_t number_of_frames) const;

    void push_zeroed_data(int sample_rate, size_t number_of_channels) const;

    rtc::scoped_refptr<webrtc::AudioSourceInterface> GetSource() const;

private:
    rtc::scoped_refptr<ArcasAudioTrackSourceInternal> api;
};

std::shared_ptr<ArcasAudioTrackSource> create_audio_track_source();

std::shared_ptr<ArcasAudioTrackSource> gen_shared_audio_track_source();