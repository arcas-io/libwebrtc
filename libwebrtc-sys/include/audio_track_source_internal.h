#pragma once
#include "api/media_stream_interface.h"
#include "absl/synchronization/mutex.h"
#include <set>

using AudioObserver = webrtc::AudioSourceInterface::AudioObserver;

class ArcasAudioTrackSourceInternal: public webrtc::AudioSourceInterface {
    public:
    void SetVolume(double) override;
    void RegisterAudioObserver(AudioObserver*) override;
    void UnregisterAudioObserver(AudioObserver*) override;
    void AddSink(webrtc::AudioTrackSinkInterface*) override;
    void RemoveSink(webrtc::AudioTrackSinkInterface*) override;

    /*
    `PushData` can currently use LPCM audio data (also required by
    audio_device_module).
    */
    void PushData(
        const void* audio_data,
        int bits_per_sample,
        int sample_rate,
        size_t number_of_channels,
        size_t number_of_frames
    );

    private:
    absl::Mutex lock_;
    double volume_;
    std::set<AudioObserver*> observers_;
    std::set<webrtc::AudioTrackSinkInterface*> sinks_;
};