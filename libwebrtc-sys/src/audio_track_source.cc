#include "libwebrtc-sys/include/audio_track_source.h"

ArcasAudioTrackSource::ArcasAudioTrackSource() {
    api = rtc::scoped_refptr<ArcasAudioTrackSourceInternal>();
}

rtc::scoped_refptr<webrtc::AudioSourceInterface> ArcasAudioTrackSource::GetSource() const {
    return api;
}

void ArcasAudioTrackSource::push_raw_s16be(
    rust::Vec<uint8_t> audio_data,
    int sample_rate,
    size_t number_of_channels,
    size_t number_of_frames
) const {
    api->PushData(audio_data.data(), 16, sample_rate, number_of_channels, number_of_frames);
}

// pushes 10ms of zeroed data
void ArcasAudioTrackSource::push_zeroed_data(
    int sample_rate,
    size_t number_of_channels
) const {
    std::vector<int16_t> data;
    for(int i=0; i< (sample_rate/100) * number_of_channels; i++) {
        data.push_back(0);
    }
    api->PushData(
        data.data(),
        16,
        sample_rate,
        number_of_channels,
        1
    );
}

std::shared_ptr<ArcasAudioTrackSource> create_audio_track_source() {
    return std::make_shared<ArcasAudioTrackSource>();
}