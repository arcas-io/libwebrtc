#pragma once
#include "libwebrtc-sys/include/api_internal.h"
#include "libwebrtc-sys/include/peerconnection_factory_config.h"
#include "system_wrappers/include/field_trial.h"

class ArcasAPI
{
private:
    rtc::scoped_refptr<ArcasAPIInternal> internal;

public:
    ArcasAPI() : internal(rtc::make_ref_counted<ArcasAPIInternal>()) {}
    ~ArcasAPI() {}

    std::unique_ptr<ArcasPeerConnectionFactory> create_factory() const
    {
        auto cxx_factory = internal->create_factory();
        RTC_LOG(LS_INFO) << "create_factory()";
        return std::make_unique<ArcasPeerConnectionFactory>(internal, cxx_factory);
    }

    std::unique_ptr<ArcasPeerConnectionFactory> create_factory_with_arcas_video_encoder_factory(std::unique_ptr<ArcasVideoEncoderFactory> video_encoder_factory) const
    {
        auto cxx_factory = internal->create_factory_with_arcas_video_encoder_factory(std::move(video_encoder_factory));
        RTC_LOG(LS_INFO) << "field: " << webrtc::field_trial::GetFieldTrialString();
        return std::make_unique<ArcasPeerConnectionFactory>(internal, cxx_factory);
    }

    std::unique_ptr<ArcasPeerConnectionFactory> create_factory_with_config(std::unique_ptr<ArcasPeerConnectionFactoryConfig> config) const
    {
        auto cxx_factory = internal->create_factory_with_config(std::move(config));
        RTC_LOG(LS_INFO) << "field: " << webrtc::field_trial::GetFieldTrialString();
        return std::make_unique<ArcasPeerConnectionFactory>(internal, cxx_factory);
    }
};

std::unique_ptr<ArcasAPI> create_arcas_api();
