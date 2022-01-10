#include "libwebrtc-sys/include/audio_device_module.h"
#include "rtc_base/logging.h"

/* ArcasAudioDeviceModule::ArcasAudioDeviceModule(webrtc::TaskQueueFactory* factory): audio_buffer_(factory) {} */
ArcasAudioDeviceModule::ArcasAudioDeviceModule(webrtc::TaskQueueFactory *factory) {}

int32_t ArcasAudioDeviceModule::RegisterAudioCallback(webrtc::AudioTransport *callback)
{
    absl::MutexLock l(&lock_);
    if (playing_) {
        return -1;
    }
    RTC_LOG(LS_ERROR) << "RegisterAudioCallback " << callback;
    audioCallback = callback;
    return 0;
}

int32_t ArcasAudioDeviceModule::StartPlayout()
{
    RTC_LOG(LS_ERROR) << "StartPlayout";
    absl::MutexLock l(&lock_);
    if (playing_)
    {
        return 0;
    }
    playout_thread_ = rtc::PlatformThread::SpawnJoinable(
        [this]
        {
            RTC_LOG(LS_ERROR) << "Playout thread spawned " << this;
            while (PlayoutThread())
            {
                rtc::Thread::SleepMs(9);
            }
        },
        "arcas_audio_module_playout",
        rtc::ThreadAttributes().SetPriority(rtc::ThreadPriority::kRealtime));
    playing_ = true;
    return 0;
}

int32_t ArcasAudioDeviceModule::StopPlayout()
{
    absl::MutexLock l(&lock_);
    if (!playing_)
    {
        return 0;
    }
    playout_thread_.Finalize();
    playing_ = false;
}

bool ArcasAudioDeviceModule::Playing() const
{
    // not safe, invariant read outside lock
    return playing_;
}

int32_t ArcasAudioDeviceModule::PlayoutThread()
{
    /* RTC_LOG(LS_ERROR) << "PlayoutThread"; */
    absl::MutexLock l(&lock_);
    if (audioCallback != nullptr)
    {
        /* RTC_LOG(LS_ERROR) << "audioCallback"; */
        int samples_per_channel = 80;
        int64_t elapsed_time_ms = -1;
        int64_t ntp_time_ms = -1;
        size_t samples_out = 0;
        audioCallback->NeedMorePlayData(
            samples_per_channel,
            2,
            1,
            8000,
            sample_buf,
            samples_out,
            &elapsed_time_ms,
            &ntp_time_ms);
        /* RTC_LOG(LS_ERROR) << "samples" << samples_out; */
    }
    return true;
}
