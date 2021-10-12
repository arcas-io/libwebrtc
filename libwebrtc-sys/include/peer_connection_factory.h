#pragma once
#include <string>
#include "rust/cxx.h"
#include <mutex>
#include <memory>
#include "api/audio_codecs/audio_decoder_factory_template.h"
#include "api/audio_codecs/audio_encoder_factory_template.h"
#include "api/audio_codecs/opus/audio_decoder_opus.h"
#include "api/audio_codecs/opus/audio_encoder_opus.h"
#include "api/call/call_factory_interface.h"
#include "api/create_peerconnection_factory.h"
#include "api/peer_connection_interface.h"
#include "api/rtc_event_log/rtc_event_log_factory.h"
#include "api/stats/rtcstats_objects.h"
#include "api/task_queue/default_task_queue_factory.h"
#include "api/video_codecs/builtin_video_decoder_factory.h"
#include "api/video_codecs/builtin_video_encoder_factory.h"
#include "media/engine/webrtc_media_engine.h"
#include "modules/audio_device/include/audio_device.h"
#include "modules/audio_processing/include/audio_processing.h"
#include "api/create_peerconnection_factory.h"
#include "api/scoped_refptr.h"
#include "api/peer_connection_interface.h"
#include "api/call/call_factory_interface.h"
#include "api/task_queue/default_task_queue_factory.h"
#include "api/rtc_event_log/rtc_event_log_factory.h"
#include "api/audio_codecs/builtin_audio_encoder_factory.h"
#include "api/audio_codecs/builtin_audio_decoder_factory.h"
#include "api/video_codecs/builtin_video_encoder_factory.h"
#include "api/video_codecs/builtin_video_decoder_factory.h"
#include "modules/audio_mixer/audio_mixer_impl.h"
#include "modules/audio_processing/include/audio_processing.h"
#include "modules/audio_device/include/fake_audio_device.h"
#include "media/engine/webrtc_media_engine.h"
#include "pc/test/fake_audio_capture_module.h"
#include "rtc_base/ref_count.h"
#include "rtc_base/thread.h"
#include "rtc_base/logging.h"

// C++ style was largely copied from the example provided by cxx
// https://github.com/dtolnay/cxx/blob/6ca43ce46fe52c5e9da010427caa55af99918a88/demo/src/blobstore.cc
//
// This seems to include use of the pImpl (https://en.cppreference.com/w/cpp/language/pimpl) pattern.
// Using the pattern works around some issues around how const gets added in cxx so it was followed here.

class ArcasPeerConnectionFactory
{
private:
    class impl;
    std::shared_ptr<impl> api;

public:
    ArcasPeerConnectionFactory(
        webrtc::PeerConnectionFactoryInterface *factory,
        std::unique_ptr<rtc::Thread> signal_thread,
        std::unique_ptr<rtc::Thread> worker_thread,
        std::unique_ptr<rtc::Thread> network_thread,
        rtc::scoped_refptr<webrtc::AudioDeviceModule> adm);
};

std::unique_ptr<ArcasPeerConnectionFactory> createFactory();