#include "libwebrtc-sys/include/webrtc.h"
#include "libwebrtc-sys/src/lib.rs.h"
#include <string>
#include <iostream>

ArcasWebRTC::ArcasWebRTC() : inner() {}

std::unique_ptr<ArcasWebRTC> createWebRTC()
{
    return std::make_unique<ArcasWebRTC>();
}

ArcasPeerConnectionFactory::ArcasPeerConnectionFactory(
    webrtc::PeerConnectionFactoryInterface *factory,
    std::unique_ptr<rtc::Thread> signal_thread,
    std::unique_ptr<rtc::Thread> worker_thread,
    std::unique_ptr<rtc::Thread> network_thread,
    rtc::scoped_refptr<webrtc::AudioDeviceModule> adm) : factory_(factory),
                                                         signal_thread_(std::move(signal_thread)),
                                                         worker_thread_(std::move(worker_thread)),
                                                         network_thread_(std::move(network_thread)),
                                                         adm_(adm)
{
}

ArcasPeerConnectionFactory::~ArcasPeerConnectionFactory()
{
    factory_->Release();
    signal_thread_->Stop();
    worker_thread_->Stop();
    network_thread_->Stop();
    RTC_LOG(LS_VERBOSE) << "~FFI_PeerConnectionFactory";
}

std::unique_ptr<ArcasPeerConnectionFactory> ArcasWebRTC::createFactory() const
{
    // Create threads
    auto worker_thread = rtc::Thread::Create();
    auto network_thread = rtc::Thread::CreateWithSocketServer();
    auto signal_thread = rtc::Thread::Create();

    signal_thread->Start();
    worker_thread->Start();
    network_thread->Start();

    RTC_LOG(LS_VERBOSE) << "created and started threads";

    webrtc::PeerConnectionFactoryDependencies dependencies;
    dependencies.network_thread = network_thread.get();
    dependencies.signaling_thread = signal_thread.get();
    dependencies.worker_thread = worker_thread.get();
    dependencies.call_factory = webrtc::CreateCallFactory();
    dependencies.task_queue_factory = webrtc::CreateDefaultTaskQueueFactory();
    dependencies.event_log_factory = std::make_unique<webrtc::RtcEventLogFactory>(dependencies.task_queue_factory.get());

    auto adm = worker_thread->Invoke<rtc::scoped_refptr<webrtc::AudioDeviceModule>>(
        RTC_FROM_HERE, [&dependencies]()
        { return webrtc::FakeAudioDeviceModule::Create(webrtc::AudioDeviceModule::kDummyAudio, dependencies.task_queue_factory.get()); });

    cricket::MediaEngineDependencies media_deps;
    media_deps.task_queue_factory = dependencies.task_queue_factory.get();
    media_deps.audio_encoder_factory = webrtc::CreateBuiltinAudioEncoderFactory();
    media_deps.audio_decoder_factory = webrtc::CreateBuiltinAudioDecoderFactory();
    media_deps.video_encoder_factory = webrtc::CreateBuiltinVideoEncoderFactory();
    media_deps.video_decoder_factory = webrtc::CreateBuiltinVideoDecoderFactory();
    media_deps.audio_processing = webrtc::AudioProcessingBuilder().Create();
    media_deps.audio_mixer = webrtc::AudioMixerImpl::Create();
    media_deps.adm = adm;

    dependencies.media_engine = cricket::CreateMediaEngine(std::move(media_deps));

    auto factory = webrtc::CreateModularPeerConnectionFactory(std::move(dependencies)).release();

    RTC_LOG(LS_INFO) << ">>>>>>>>>>>>>>>>>>> instantiated PEERCONNECTION_FACTORY";

    auto result = std::make_unique<ArcasPeerConnectionFactory>(
        std::move(factory),
        std::move(signal_thread),
        std::move(worker_thread),
        std::move(network_thread),
        adm);
    return result;
}