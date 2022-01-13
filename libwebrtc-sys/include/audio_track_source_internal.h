#pragma once
#include "absl/synchronization/mutex.h"
#include "api/media_stream_interface.h"
#include "api/scoped_refptr.h"
#include <set>

using AudioObserver = webrtc::AudioSourceInterface::AudioObserver;

class ArcasAudioTrackSourceInternal : public webrtc::AudioSourceInterface,
                                      public rtc::RefCountedBase
{
public:
    // AudioTrackSourceInterface
    void SetVolume(double) override;
    void RegisterAudioObserver(AudioObserver*) override;
    void UnregisterAudioObserver(AudioObserver*) override;
    void AddSink(webrtc::AudioTrackSinkInterface*) override;
    void RemoveSink(webrtc::AudioTrackSinkInterface*) override;

    /*
    `PushData` can currently use LPCM audio data (also required by
    audio_device_module).
    */
    void PushData(const void* audio_data,
                  int bits_per_sample,
                  int sample_rate,
                  size_t number_of_channels,
                  size_t number_of_frames);

    // MediaSourceInterface
    // TODO: Write bindings for these
    webrtc::MediaSourceInterface::SourceState state() const override
    {
        return webrtc::MediaSourceInterface::kLive;
    }

    bool remote() const override
    {
        return false;
    }

    // Notifier interface
    void RegisterObserver(webrtc::ObserverInterface* obs) override
    {
        notifier_observers_.insert(obs);
    }

    void UnregisterObserver(webrtc::ObserverInterface* obs) override
    {
        notifier_observers_.erase(obs);
    }

    // RefCountedBase

    void AddRef() const override
    {
        rtc::RefCountedBase::AddRef();
    }

    rtc::RefCountReleaseStatus Release() const override
    {
        return rtc::RefCountedBase::Release();
    }

private:
    absl::Mutex lock_;
    double volume_;
    std::set<AudioObserver*> observers_;
    std::set<webrtc::AudioTrackSinkInterface*> sinks_;
    std::set<webrtc::ObserverInterface*> notifier_observers_;
};