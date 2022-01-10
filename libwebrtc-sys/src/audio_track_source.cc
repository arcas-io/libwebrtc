#include "libwebrtc-sys/include/audio_track_source.h"
#include "math.h"
#include "rtc_base/ref_counted_object.h"

ArcasAudioTrackSource::ArcasAudioTrackSource()
{
    api = rtc::make_ref_counted<ArcasAudioTrackSourceInternal>();
}

rtc::scoped_refptr<webrtc::AudioSourceInterface> ArcasAudioTrackSource::GetSource() const
{
    return api;
}

void ArcasAudioTrackSource::push_raw_s16be(rust::Vec<uint8_t> audio_data,
                                           int sample_rate,
                                           size_t number_of_channels,
                                           size_t number_of_frames) const
{
    api->PushData(audio_data.data(), 16, sample_rate, number_of_channels, number_of_frames);
}

// pushes 10ms of zeroed data
void ArcasAudioTrackSource::push_zeroed_data(int sample_rate, size_t number_of_channels) const
{
    std::vector<int16_t> data;
    int total_samples = (sample_rate)*number_of_channels;
    int t_idx = 0;
    for (int i = 0; i < total_samples; i += number_of_channels)
    {
        double t = (double)t_idx / (double)sample_rate;
        for (int j = 0; j < number_of_channels; j++)
        {
            double amplitude = 1 << 8;
            data.push_back((int16_t)(amplitude * sin(2.0f * M_PI * 440.0f * t)));
            /* data.push_back(0); */
        }
        t_idx++;
    }
    api->PushData(data.data(),
                  16 * number_of_channels,
                  sample_rate,
                  number_of_channels,
                  sample_rate / 100);
}

std::shared_ptr<ArcasAudioTrackSource> create_audio_track_source()
{
    return std::make_shared<ArcasAudioTrackSource>();
}