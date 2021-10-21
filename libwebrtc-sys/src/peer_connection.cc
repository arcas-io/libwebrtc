#include "iostream"
#include "rust/cxx.h"
#include "libwebrtc-sys/include/peer_connection_observer.h"
#include "libwebrtc-sys/include/peer_connection.h"
#include <iostream>

ArcasPeerConnection::ArcasPeerConnection(
    rtc::scoped_refptr<webrtc::PeerConnectionInterface> api,
    rtc::scoped_refptr<ArcasPeerConnectionObserver> observer) : api(std::move(api)),
                                                                observer(observer)
{
}

void ArcasPeerConnection::create_offer(rust::Box<ArcasRustCreateSessionDescriptionObserver> observer) const
{
    webrtc::PeerConnectionInterface::RTCOfferAnswerOptions options;
    auto ref_counted = rtc::make_ref_counted<ArcasCreateSessionDescriptionObserver>(std::move(observer));
    api->CreateOffer(ref_counted, options);
}

void ArcasPeerConnection::create_answer(rust::Box<ArcasRustCreateSessionDescriptionObserver> observer) const
{
    // TODO: This probably has to live longer elsewhere.
    webrtc::PeerConnectionInterface::RTCOfferAnswerOptions options;
    auto ref_counted = rtc::make_ref_counted<ArcasCreateSessionDescriptionObserver>(std::move(observer));
    api->CreateAnswer(ref_counted, options);
}

void ArcasPeerConnection::set_local_description(rust::Box<ArcasRustSetSessionDescriptionObserver> observer, std::unique_ptr<ArcasSessionDescription> sdp) const
{
    auto ref_counted = rtc::make_ref_counted<ArcasSetDescriptionObserver>(std::move(observer));
    api->SetLocalDescription(std::move(sdp->clone_sdp()), ref_counted);
}

void ArcasPeerConnection::set_remote_description(rust::Box<ArcasRustSetSessionDescriptionObserver> observer, std::unique_ptr<ArcasSessionDescription> sdp) const
{
    auto ref_counted = rtc::make_ref_counted<ArcasSetDescriptionObserver>(std::move(observer));
    api->SetRemoteDescription(std::move(sdp->clone_sdp()), ref_counted);
}

std::unique_ptr<ArcasRTPTransceiver> ArcasPeerConnection::add_simple_media_transceiver(cricket::MediaType media) const
{
    auto result = api->AddTransceiver(media);

    if (result.ok())
    {
        return std::make_unique<ArcasRTPTransceiver>(result.MoveValue());
    }

    // TODO: Handle error cases.
    return nullptr;
}