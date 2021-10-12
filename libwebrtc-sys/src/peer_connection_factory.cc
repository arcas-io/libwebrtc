#include "libwebrtc-sys/include/peer_connection_factory.h"
#include "libwebrtc-sys/src/lib.rs.h"
#include <string>
#include <iostream>

class ArcasPeerConnectionFactory::impl
{
    friend ArcasPeerConnectionFactory;

    webrtc::PeerConnectionFactoryInterface *factory;
    std::unique_ptr<rtc::Thread> signal_thread;
    std::unique_ptr<rtc::Thread> worker_thread;
    std::unique_ptr<rtc::Thread> network_thread;
    rtc::scoped_refptr<webrtc::AudioDeviceModule> adm;

public:
    impl(
        webrtc::PeerConnectionFactoryInterface *factory,
        std::unique_ptr<rtc::Thread> signal_thread,
        std::unique_ptr<rtc::Thread> worker_thread,
        std::unique_ptr<rtc::Thread> network_thread,
        rtc::scoped_refptr<webrtc::AudioDeviceModule> adm);
    ~impl();
};

ArcasPeerConnectionFactory::impl::impl(
    webrtc::PeerConnectionFactoryInterface *factory,
    std::unique_ptr<rtc::Thread> signal_thread,
    std::unique_ptr<rtc::Thread> worker_thread,
    std::unique_ptr<rtc::Thread> network_thread,
    rtc::scoped_refptr<webrtc::AudioDeviceModule> adm) : factory(factory),
                                                         signal_thread(std::move(signal_thread)),
                                                         worker_thread(std::move(worker_thread)),
                                                         network_thread(std::move(network_thread)),
                                                         adm(adm) {}

ArcasPeerConnectionFactory::impl::~impl()
{
    // C++ side must free it's own resources.
    factory->Release();
    signal_thread->Stop();
    worker_thread->Stop();
    network_thread->Stop();
    RTC_LOG(LS_VERBOSE) << "~FFI_PeerConnectionFactory";
}

ArcasPeerConnectionFactory::ArcasPeerConnectionFactory(
    webrtc::PeerConnectionFactoryInterface *factory,
    std::unique_ptr<rtc::Thread> signal_thread,
    std::unique_ptr<rtc::Thread> worker_thread,
    std::unique_ptr<rtc::Thread> network_thread,
    rtc::scoped_refptr<webrtc::AudioDeviceModule> adm)
{
    this->api = std::make_shared<ArcasPeerConnectionFactory::impl>(factory,
                                                                   std::move(signal_thread),
                                                                   std::move(worker_thread),
                                                                   std::move(network_thread),
                                                                   adm);
}

std::unique_ptr<ArcasPeerConnectionFactory> createFactory()
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