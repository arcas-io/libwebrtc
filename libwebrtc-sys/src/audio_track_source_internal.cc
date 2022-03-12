#include "audio_track_source_internal.h"

void ArcasAudioTrackSourceInternal::SetVolume(double d) {}

void ArcasAudioTrackSourceInternal::RegisterAudioObserver(AudioObserver* observer)
{
    observers_.insert(observer);
}

void ArcasAudioTrackSourceInternal::UnregisterAudioObserver(AudioObserver* observer)
{
    observers_.erase(observer);
}

void ArcasAudioTrackSourceInternal::AddSink(webrtc::AudioTrackSinkInterface* sink)
{
    sinks_.insert(sink);
}

void ArcasAudioTrackSourceInternal::RemoveSink(webrtc::AudioTrackSinkInterface* sink)
{
    sinks_.erase(sink);
}

void ArcasAudioTrackSourceInternal::PushData(
    const void* audio_data, int bits_per_sample, int sample_rate, size_t number_of_channels, size_t number_of_frames)
{
    for (auto sink : sinks_) { sink->OnData(audio_data, bits_per_sample, sample_rate, number_of_channels, number_of_frames); }
}