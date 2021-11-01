#include "iostream"
#include "rust/cxx.h"
#include "libwebrtc-sys/include/peer_connection_factory.h"
#include "libwebrtc-sys/include/peer_connection_observer.h"
#include "libwebrtc-sys/src/lib.rs.h"

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

std::unique_ptr<ArcasPeerConnectionFactory> create_factory()
{
    // TODO: Add configuration options for log levels.
    rtc::LogMessage::LogToDebug(rtc::LS_VERBOSE);
    rtc::LogMessage::SetLogToStderr(true);

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

    auto adm = rtc::make_ref_counted<ArcasAudioDeviceModule>();

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

std::unique_ptr<ArcasPeerConnectionFactory> create_factory_with_arcas_video_encoder_factory(std::unique_ptr<ArcasVideoEncoderFactory> video_encoder_factory)
{
    // TODO: Add configuration options for log levels.
    rtc::LogMessage::LogToDebug(rtc::LS_VERBOSE);
    rtc::LogMessage::SetLogToStderr(true);

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

    auto adm = rtc::make_ref_counted<ArcasAudioDeviceModule>();

    cricket::MediaEngineDependencies media_deps;
    media_deps.task_queue_factory = dependencies.task_queue_factory.get();
    media_deps.audio_encoder_factory = webrtc::CreateBuiltinAudioEncoderFactory();
    media_deps.audio_decoder_factory = webrtc::CreateBuiltinAudioDecoderFactory();
    media_deps.video_encoder_factory = std::move(video_encoder_factory);
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

std::unique_ptr<ArcasPeerConnection> ArcasPeerConnectionFactory::create_peer_connection(std::unique_ptr<webrtc::PeerConnectionInterface::RTCConfiguration> pc_config, std::shared_ptr<ArcasPeerConnectionObserver> observer) const
{
    webrtc::PeerConnectionDependencies deps(observer.get());

    auto pc = api->factory->CreatePeerConnection(*pc_config, std::move(deps));
    auto out = std::make_unique<ArcasPeerConnection>(std::move(pc));
    return out;
}

std::unique_ptr<ArcasVideoTrack> ArcasPeerConnectionFactory::create_video_track(rust::String id, std::shared_ptr<ArcasVideoTrackSource> video_source)
{
    // We provide a pointer type here but must also save the shared_ptr so this doesn't immediately deallocate.
    auto track = api->factory->CreateVideoTrack(std::string(id.c_str()), video_source.get());
    return std::make_unique<ArcasVideoTrack>(track);
}

std::unique_ptr<webrtc::PeerConnectionInterface::RTCConfiguration> create_rtc_configuration(ArcasPeerConnectionConfig config)
{
    auto rtc = std::make_unique<webrtc::PeerConnectionInterface::RTCConfiguration>();
    webrtc::PeerConnectionInterface::IceServers servers;

    rtc->sdp_semantics = config.sdp_semantics;
    rtc->servers = servers;

    for (auto server_config : config.ice_servers)
    {
        webrtc::PeerConnectionInterface::IceServer rtc_ice_server;
        std::vector<std::string> rtc_urls;

        for (auto url : server_config.urls)
        {
            auto rtc_url = std::string(url.c_str());
            rtc_urls.push_back(rtc_url);
        }

        rtc_ice_server.urls = rtc_urls;
        rtc_ice_server.username = std::string(server_config.username.c_str());
        rtc_ice_server.password = std::string(server_config.password.c_str());
    }

    return rtc;
}

std::shared_ptr<ArcasPeerConnectionObserver> create_peer_connection_observer(rust::Box<ArcasRustPeerConnectionObserver> rust_box)
{
    return std::make_shared<ArcasPeerConnectionObserver>(std::move(rust_box));
}