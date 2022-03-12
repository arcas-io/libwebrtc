#include <pc/jsep_api.h>
#include <pc/jsep_api.rs.h>

#include <candidate.rs.h>
#include <session_description.rs.h>

#include <async_dns_resolver_factory.h>
#include <ice_transport.h>
#include <p2p/base/ice_transport_internal.h>
#include <peer_connection_factory.h>
#include <rtc_base/ssl_stream_adapter.h>

#include <rtc4k/pc/rtp_transmission_manager.h>
#include <rtc4k/pc/srtp_transport.h>
#include <rtc4k/pc/video_rtp_receiver.h>
#include <rtc4k/pc/video_track.h>
#include <rtc4k/rtc_base/location.h>

#include <tests/mock/erasure.h>
#include <tests/mock/privablic.h>

#include <catch2/catch.hpp>
#include <cxx.h>
#include <fmt/core.h>

#include <thread>

using namespace std::literals;

struct thief
{
    using type = void (cricket::DtlsTransport::*)(bool);
};
template class private_method<thief, &cricket::DtlsTransport::set_writable>;

namespace
{
struct slots : public sigslot::has_slots<>
{
    void slot_bool(bool b)
    {
        std::clog << __PRETTY_FUNCTION__ << '(' << std::boolalpha << b << ')' << std::endl;
        std::abort();
    }
};
slots to_connect;
bool some_callback(std::string txt, webrtc::RtpTransportInternal* rti, ArcasDTLSTransport&& adt, webrtc::DataChannelTransportInterface* dcti)
{
    std::clog << "Transport changed: " << txt << '\n';//<< " is ready to send? " << std::boolalpha << rti->IsReadyToSend() << " - called back\n";
    if (rti)
    {
        std::clog << " transport name: " << rti->transport_name() << '\n';
        CHECK(!rti->IsReadyToSend());
        rti->SignalReadyToSend.connect(&to_connect, &slots::slot_bool);
        rti->SignalWritableState.connect(&to_connect, &slots::slot_bool);
    }
    else
    {
        std::clog << "rti is null\n";
    }
    CHECK(dcti == nullptr);
    CHECK(txt.size() == 1);
    CHECK(txt.at(0) == '0');
    return true;
}
void different_callback(rtc::CopyOnWriteBuffer const&, std::int64_t)
{
    std::abort();
}
void a_third(ArcasCxxSSLHandshakeError&&)
{
    std::abort();
}

using make_tc_result = std::pair<std::unique_ptr<ArcasJsepTransportController>, std::string>;
make_tc_result make_tc(rtc::NetworkManager&, rtc::Thread&, int);
std::string offer_text(std::string fingerprint);
std::string answer_text(std::string fingerprint);
auto copy_to_remote = [](auto& sd)
{
    auto res = create_arcas_session_description(sd.get_type(), sd.to_string());
    CHECK(res.ok);
    return std::move(res.session);
};
}//namespace
#define CP                                                                                                                                           \
    {                                                                                                                                                \
        std::cout << "stdout Checkpoint: " << __FILE__ << ':' << __LINE__ << std::endl;                                                              \
        std::clog << "stdlog Checkpoint: " << __FILE__ << ':' << __LINE__ << std::endl;                                                              \
        std::this_thread::sleep_for(std::chrono::seconds(1));                                                                                        \
    }

TEST_CASE("Mimic the rust test jsep::test::test_transport_controller for deeper visibility", "[ArcasJsepTransportController]")
{
    auto net_man = create_arcas_cxx_network_manager();
    auto net_thread = rtc::Thread::CreateWithSocketServer();
    net_thread->SetName("A network thread for functional testing", nullptr);
    net_thread->Start();

    //Moved a bunch of init to make_tc so I don't have all these moved-from locals littered about
    auto [tc1, finger1] = make_tc(*net_man, *net_thread, 3340);
    auto [tc2, finger2] = make_tc(*net_man, *net_thread, 3341);

    std::unique_ptr<ArcasRTCError> err;
    auto not_err = [&err]() { return !err || err->ok(); };
    auto const sdp1 = offer_text(finger1);
    {
        auto offer = create_arcas_session_description(webrtc::SdpType::kOffer, sdp1);
        CHECK(offer.ok);
        auto remote_offer = copy_to_remote(*offer.session).release();
        err = tc1->set_local_description(webrtc::SdpType::kOffer, *offer.session.release());
        CHECK(not_err());
        err = tc2->set_remote_description(webrtc::SdpType::kOffer, *remote_offer);
        CHECK(not_err());
    }
    auto const sdp2 = answer_text(finger2);
    {
        auto answer = create_arcas_session_description(webrtc::SdpType::kAnswer, sdp2);
        CHECK(answer.ok);
        auto remote_answer = copy_to_remote(*answer.session).release();
        err = tc2->set_local_description(webrtc::SdpType::kAnswer, *answer.session.release());
        if (err)
        {
            CHECK(err->message().c_str() == ""s);
        }
        CHECK(not_err());
        err = tc1->set_remote_description(webrtc::SdpType::kAnswer, *remote_answer);
        if (err)
        {
            CHECK(err->message().c_str() == ""s);
        }
        CHECK(not_err());
    }
    CP;
    tc1->subsribe_ice_connection_state([](auto s) { std::clog << "t1 ice_conn=" << s << std::endl; });
    CP;
    tc2->subsribe_ice_connection_state([](auto s) { std::clog << "t2 ice_conn=" << s << std::endl; });
    auto ice_cfg = create_arcas_p2p_ice_config();
    CP;
    ice_cfg->set_presume_writable_when_fully_relayed(true);

    tc1->set_ice_config(std::move(ice_cfg));
    ice_cfg = create_arcas_p2p_ice_config();
    ice_cfg->set_presume_writable_when_fully_relayed(true);
    tc2->set_ice_config(std::move(ice_cfg));


    auto cand1 = create_arcas_candidate();
    cand1->set_address(std::getenv("HOSTNAME") + ":3340"s);
    cand1->set_component(CandidateComponent::Rtp);
    cand1->set_protocol("tcp");
    auto e = cricket::VerifyCandidate(cand1->get_candidate());
    CHECK(e.ok());
    CHECK(e.message() == ""s);
    err = tc2->add_remote_candidates("0"s, {cand1->get_candidate()});
    CHECK(not_err());
    auto cand2 = create_arcas_candidate();
    cand2->set_address(std::getenv("HOSTNAME") + ":3341"s);
    cand2->set_component(CandidateComponent::Rtp);
    cand2->set_protocol("tcp");
    e = cricket::VerifyCandidate(cand2->get_candidate());
    CHECK(e.ok());

    err = tc1->add_remote_candidates("0"s, {cand2->get_candidate()});
    CHECK(not_err());
    auto transport = tc1->get_srtp_transport("0");
    CHECK(transport);
    CP;
    auto name = net_thread->Invoke<std::string>({}, [&]() { return transport->transport_name(); });
    CHECK(name == "0");
    auto srtp_transport = dynamic_cast<webrtc::SrtpTransport*>(transport);
    CHECK(srtp_transport);//this might fail if you compiled with -fno-rtti
    std::array<std::uint8_t, 30> send_key, recv_key;
    auto f = [&]() { srtp_transport->SetRtpParams(1, send_key.data(), send_key.size(), {}, 1, recv_key.data(), recv_key.size(), {}); };
    net_thread->Invoke<void>({}, f);
    auto t2 = srtp_transport->rtp_packet_transport();
    CHECK(t2);
    auto cricket_dtls = dynamic_cast<cricket::DtlsTransport*>(t2);
    CHECK(cricket_dtls);
    tc1->maybe_start_gathering();
    tc2->maybe_start_gathering();
    std::string packet{"0123456789012"};
    packet[0] = (1 << 7);
    packet[1] = 55;
    rtc::CopyOnWriteBuffer cow{packet};
    cow.EnsureCapacity(99);
    auto send = [&]() { return transport->SendRtpPacket(&cow, {}, {}); };
    auto sig_thr = create_arcas_cxx_thread();
    auto wkr_thr = create_arcas_cxx_thread();
    sig_thr->Start();
    wkr_thr->Start();
    //Note that transport is captured by ref
    transport = tc2->get_srtp_transport("0");
    srtp_transport = dynamic_cast<webrtc::SrtpTransport*>(transport);
    CHECK(srtp_transport);//this might fail if you compiled with -fno-rtti
    net_thread->Invoke<void>({}, f);
    CP;
    CP;
    bool success = net_thread->Invoke<bool>({}, send);
    //    CHECK(success);
    /*
    CP;
    CHECK(transport->IsReadyToSend());
    CP;
    CHECK(transport->IsWritable(true));
    CP;
    success = net_thread->Invoke<bool>({}, send);
    CP;
    CHECK(success);
    CP;
    webrtc::RtpTransmissionManager trx_man{true,
                                           sig_thr.get(),
                                           wkr_thr.get(),
                                           nullptr,//cricket::ChannelManager* channel_manager
                                           nullptr,//UsagePattern* usage_pattern
                                           nullptr,//PeerConnectionObserver* observer
                                           nullptr,//StatsCollectorInterface* stats_
                                           []()
                                           {
                                               std::abort();
                                           }};
    webrtc::RtpTransceiverInit transceiver_init;
    /*
    auto cm = wkr_thr->Invoke<std::unique_ptr<cricket::ChannelManager>>(
        {},
        [&]() { return cricket::ChannelManager::Create({}, true, wkr_thr.get(), net_thread.get()); });
    auto sender = webrtc::VideoRtpSender::Create(wkr_thr.get(), "0", nullptr);
    auto sender_proxy = webrtc::RtpSenderProxyWithInternal<webrtc::RtpSenderInternal>::Create(sig_thr.get(), sender);

    webrtc::VideoRtpReceiver recv{wkr_thr.get(), "0", std::vector<std::string>{}};
    //    webrtc::RtpTransceiver transceiver{cricket::MediaType::MEDIA_TYPE_VIDEO, cm.get()};
    webrtc::RtpTransceiver transceiver{sender_proxy, nullptr, cm.get(), {}, []() { std::abort(); }};
    auto vts = create_arcas_video_track_source();
    CHECK(vts);
    auto trk = webrtc::VideoTrack::Create("0", vts->ref().get(), wkr_thr.get());
    transceiver.sender()->SetTrack(trk.get());
    auto frame_buf = webrtc::I420Buffer::Create(9, 8);
    auto frame = webrtc::VideoFrame::Builder{}.set_video_frame_buffer(frame_buf).build();
    vts->push_frame(frame);
    wkr_thr->Invoke<void>({}, [&cm]() { cm.reset(); });
*/
}

namespace
{
std::string offer_text(std::string fingerprint)
{
    return fmt::format(R"sdp(v=0
o=- 1078046247596315931 2 IN IP4 {}
s=-
t=0 0
a=group:BUNDLE 0
a=extmap-allow-mixed
a=msid-semantic: WMS 0
m=video 3340 TCP/TLS/RTP/SAVPF 96 97 98 99 100
c=IN IP4 0.0.0.0
a=rtcp:9 IN IP4 0.0.0.0
a=ice-ufrag:UUEB
a=ice-pwd:oBEHxMxyOKOvLLu13/fDqA68
a=ice-options:trickle
a=fingerprint:{}
a=setup:actpass
a=mid:0
a=extmap:1 urn:ietf:params:rtp-hdrext:toffset
a=extmap:2 http://www.webrtc.org/experiments/rtp-hdrext/abs-send-time
a=extmap:3 urn:3gpp:video-orientation
a=extmap:4 http://www.ietf.org/id/draft-holmer-rmcat-transport-wide-cc-extensions-01
a=extmap:5 http://www.webrtc.org/experiments/rtp-hdrext/playout-delay
a=extmap:6 http://www.webrtc.org/experiments/rtp-hdrext/video-content-type
a=extmap:7 http://www.webrtc.org/experiments/rtp-hdrext/video-timing
a=extmap:8 http://www.webrtc.org/experiments/rtp-hdrext/color-space
a=extmap:9 urn:ietf:params:rtp-hdrext:sdes:mid
a=extmap:10 urn:ietf:params:rtp-hdrext:sdes:rtp-stream-id
a=extmap:11 urn:ietf:params:rtp-hdrext:sdes:repaired-rtp-stream-id
a=sendrecv
a=msid:0 Testlabel
a=rtcp-mux
a=rtcp-rsize
a=rtpmap:96 VP8/90000
a=rtcp-fb:96 goog-remb
a=rtcp-fb:96 transport-cc
a=rtcp-fb:96 ccm fir
a=rtcp-fb:96 nack
a=rtcp-fb:96 nack pli
a=rtpmap:97 rtx/90000
a=fmtp:97 apt=96
a=rtpmap:98 red/90000
a=rtpmap:99 rtx/90000
a=fmtp:99 apt=98
a=rtpmap:100 ulpfec/90000
a=ssrc-group:FID 3901065077 2566872679
a=ssrc:3901065077 cname:uNQfOR05i3pJxaq1
a=ssrc:3901065077 msid:0 Testlabel
a=ssrc:3901065077 mslabel:0
a=ssrc:3901065077 label:Testlabel
a=ssrc:2566872679 cname:uNQfOR05i3pJxaq1
a=ssrc:2566872679 msid:0 Testlabel
a=ssrc:2566872679 mslabel:0
a=ssrc:2566872679 label:Testlabel
)sdp",
                       std::getenv("HOSTNAME"),
                       fingerprint);
}
std::string answer_text(std::string fingerprint)
{
    return fmt::format(R"sdp(v=0
o=- 1078046247596315931 2 IN IP4 {}
s=-
t=0 0
a=extmap-allow-mixed
a=msid-semantic: WMS 0
m=video 3341 TCP/TLS/RTP/SAVPF 96 97 98 99 100
c=IN IP4 0.0.0.0
a=rtcp:9 IN IP4 0.0.0.0
a=ice-ufrag:UUEB
a=ice-pwd:oBEHxMxyOKOvLLu13/fDqA68
a=ice-options:trickle
a=fingerprint:{}
a=setup:passive
a=mid:0
a=extmap:1 urn:ietf:params:rtp-hdrext:toffset
a=extmap:2 http://www.webrtc.org/experiments/rtp-hdrext/abs-send-time
a=extmap:3 urn:3gpp:video-orientation
a=extmap:4 http://www.ietf.org/id/draft-holmer-rmcat-transport-wide-cc-extensions-01
a=extmap:5 http://www.webrtc.org/experiments/rtp-hdrext/playout-delay
a=extmap:6 http://www.webrtc.org/experiments/rtp-hdrext/video-content-type
a=extmap:7 http://www.webrtc.org/experiments/rtp-hdrext/video-timing
a=extmap:8 http://www.webrtc.org/experiments/rtp-hdrext/color-space
a=extmap:9 urn:ietf:params:rtp-hdrext:sdes:mid
a=extmap:10 urn:ietf:params:rtp-hdrext:sdes:rtp-stream-id
a=extmap:11 urn:ietf:params:rtp-hdrext:sdes:repaired-rtp-stream-id
a=sendrecv
a=msid:0 Testlabel
a=rtcp-mux
a=rtcp-rsize
a=rtpmap:96 VP8/90000
a=rtcp-fb:96 goog-remb
a=rtcp-fb:96 transport-cc
a=rtcp-fb:96 ccm fir
a=rtcp-fb:96 nack
a=rtcp-fb:96 nack pli
a=rtpmap:97 rtx/90000
a=fmtp:97 apt=96
a=rtpmap:98 red/90000
a=rtpmap:99 rtx/90000
a=fmtp:99 apt=98
a=rtpmap:100 ulpfec/90000
a=ssrc-group:FID 3901065077 2566872679
a=ssrc:3901065077 cname:uNQfOR05i3pJxaq1
a=ssrc:3901065077 msid:0 Testlabel
a=ssrc:3901065077 mslabel:0
a=ssrc:3901065077 label:Testlabel
a=ssrc:2566872679 cname:uNQfOR05i3pJxaq1
a=ssrc:2566872679 msid:0 Testlabel
a=ssrc:2566872679 mslabel:0
a=ssrc:2566872679 label:Testlabel
)sdp",
                       std::getenv("HOSTNAME"),
                       fingerprint);
}
std::unique_ptr<ArcasJsepTransportControllerConfig> plain_config()
{
    auto cfg = create_arcas_jsep_transport_controller_config();
    cfg->set_observer(make_jsep_obs(some_callback));
    cfg->bypass_rtcp_handler(different_callback);
    cfg->set_ice_transport_factory(create_arcas_cxx_ice_transport_factory());
    cfg->bypass_dtls_handshake_error_handler(a_third);
    return std::move(cfg);
}
make_tc_result make_tc(rtc::NetworkManager& nm, rtc::Thread& thr, int port)
{
    auto port_alloc = create_arcas_cxx_port_allocator(&nm);
    port_alloc->SetPortRange(port, port);
    //    port_alloc->SetNetworkIgnoreMask(0);
    auto async_dns_resolver_factory = create_arcas_cxx_async_dns_resolver_factory().release();
    auto a_resolver = async_dns_resolver_factory->Create();
    auto tc = create_arcas_jsep_transport_controller(&thr, std::move(port_alloc), async_dns_resolver_factory, plain_config());
    auto ident = create_arcas_ssl_identity_with_key_params("common name", create_arcas_key_params_rsa());
    auto cert = create_arcas_rtc_certificate(std::move(ident));
    auto const finger = cert->get_fingerprint();
    tc->set_local_certificate(std::move(cert));
    return {std::move(tc), std::string{finger.data(), finger.size()}};
}
}//namespace
