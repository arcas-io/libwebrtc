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

const double AUDIO_FREQUENCY = 440.0;
const double AUDIO_AMPLITUDE = 32000.0;
// pushes 10ms of 440 Hz sine wave data
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
            int res = (int16_t)(AUDIO_AMPLITUDE * sin(2.0f * M_PI * AUDIO_FREQUENCY * t));
            data.push_back(res);
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