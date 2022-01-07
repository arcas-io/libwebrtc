#pragma once
#include "modules/audio_device/include/audio_device.h"

class ArcasAudioDeviceModule : public webrtc::AudioDeviceModule
{
public:
    ArcasAudioDeviceModule();

    // Retrieve the currently utilized audio layer
    int32_t ActiveAudioLayer(webrtc::AudioDeviceModule::AudioLayer* audioLayer) const
    {
        return -1;
    };

    // Full-duplex transportation of PCM audio
    int32_t RegisterAudioCallback(webrtc::AudioTransport* audioCallback)
    {
        return -1;
    };

    // Main initialization and termination
    int32_t Init()
    {
        return 0;
    };
    int32_t Terminate()
    {
        return 0;
    };
    bool Initialized() const
    {
        return 0;
    };

    // Device enumeration
    int16_t PlayoutDevices()
    {
        return -1;
    };
    int16_t RecordingDevices()
    {
        return -1;
    };
    int32_t PlayoutDeviceName(uint16_t index,
                              char name[webrtc::kAdmMaxDeviceNameSize],
                              char guid[webrtc::kAdmMaxGuidSize])
    {
        return 0;
    };
    int32_t RecordingDeviceName(uint16_t index,
                                char name[webrtc::kAdmMaxDeviceNameSize],
                                char guid[webrtc::kAdmMaxGuidSize])
    {
        return 0;
    };

    // Device selection
    int32_t SetPlayoutDevice(uint16_t index)
    {
        return 0;
    };
    int32_t SetPlayoutDevice(webrtc::AudioDeviceModule::WindowsDeviceType device)
    {
        return 0;
    };
    int32_t SetRecordingDevice(uint16_t index)
    {
        return 0;
    };
    int32_t SetRecordingDevice(webrtc::AudioDeviceModule::WindowsDeviceType device)
    {
        return -1;
    };

    // Audio transport initialization
    int32_t PlayoutIsAvailable(bool* available)
    {
        return -1;
    };
    int32_t InitPlayout()
    {
        return -1;
    };
    bool PlayoutIsInitialized() const
    {
        return true;
    };
    int32_t RecordingIsAvailable(bool* available)
    {
        return 0;
    };
    int32_t InitRecording()
    {
        return 0;
    };
    bool RecordingIsInitialized() const
    {
        return true;
    };

    // Audio transport control
    int32_t StartPlayout()
    {
        return 0;
    };
    int32_t StopPlayout()
    {
        return -1;
    };
    bool Playing() const
    {
        return true;
    };
    int32_t StartRecording()
    {
        return 0;
    };
    int32_t StopRecording()
    {
        return 0;
    };
    bool Recording() const
    {
        return true;
    };

    // Audio mixer initialization
    int32_t InitSpeaker()
    {
        return 0;
    };
    bool SpeakerIsInitialized() const
    {
        return -1;
    };
    int32_t InitMicrophone()
    {
        return 0;
    };
    bool MicrophoneIsInitialized() const
    {
        return -1;
    };

    // Speaker volume controls
    int32_t SpeakerVolumeIsAvailable(bool* available)
    {
        return -1;
    };
    int32_t SetSpeakerVolume(uint32_t volume)
    {
        return -1;
    };
    int32_t SpeakerVolume(uint32_t* volume) const
    {
        return -1;
    };
    int32_t MaxSpeakerVolume(uint32_t* maxVolume) const
    {
        return -1;
    };
    int32_t MinSpeakerVolume(uint32_t* minVolume) const
    {
        return -1;
    };

    // Microphone volume controls
    int32_t MicrophoneVolumeIsAvailable(bool* available)
    {
        return -1;
    };
    int32_t SetMicrophoneVolume(uint32_t volume)
    {
        return -1;
    };
    int32_t MicrophoneVolume(uint32_t* volume) const
    {
        return -1;
    };
    int32_t MaxMicrophoneVolume(uint32_t* maxVolume) const
    {
        return -1;
    };
    int32_t MinMicrophoneVolume(uint32_t* minVolume) const
    {
        return -1;
    };

    // Speaker mute control
    int32_t SpeakerMuteIsAvailable(bool* available)
    {
        return -1;
    };
    int32_t SetSpeakerMute(bool enable)
    {
        return -1;
    };
    int32_t SpeakerMute(bool* enabled) const
    {
        return -1;
    };

    // Microphone mute control
    int32_t MicrophoneMuteIsAvailable(bool* available)
    {
        return -1;
    };
    int32_t SetMicrophoneMute(bool enable)
    {
        return -1;
    };
    int32_t MicrophoneMute(bool* enabled) const
    {
        return -1;
    };

    // Stereo support
    int32_t StereoPlayoutIsAvailable(bool* available) const
    {
        return 1;
    };
    int32_t SetStereoPlayout(bool enable)
    {
        return 0;
    };
    int32_t StereoPlayout(bool* enabled) const
    {
        return 0;
    };
    int32_t StereoRecordingIsAvailable(bool* available) const
    {
        return -1;
    };
    int32_t SetStereoRecording(bool enable)
    {
        return -1;
    };
    int32_t StereoRecording(bool* enabled) const
    {
        return -1;
    };

    // Playout delay
    int32_t PlayoutDelay(uint16_t* delayMS) const
    {
        return 0;
    };

    // Only supported on Android.
    bool BuiltInAECIsAvailable() const
    {
        return -1;
    };
    bool BuiltInAGCIsAvailable() const
    {
        return -1;
    };
    bool BuiltInNSIsAvailable() const
    {
        return -1;
    };

    // Enables the built-in audio effects. Only supported on Android.
    int32_t EnableBuiltInAEC(bool enable)
    {
        return -1;
    };
    int32_t EnableBuiltInAGC(bool enable)
    {
        return -1;
    };
    int32_t EnableBuiltInNS(bool enable)
    {
        return -1;
    };

    // Play underrun count. Only supported on Android.
    // TODO(alexnarest): Make it abstract after upstream projects support it.
    int32_t GetPlayoutUnderrunCount() const
    {
        return -1;
    }
};
