#include "iostream"
#include "rust/cxx.h"
#include "libwebrtc-sys/src/lib.rs.h"

ArcasCreateSessionDescriptionObserver::ArcasCreateSessionDescriptionObserver(
    rust::Box<ArcasRustCreateSessionDescriptionObserver> observer) : observer(std::move(observer)) {}

void ArcasCreateSessionDescriptionObserver::OnSuccess(webrtc::SessionDescriptionInterface *desc)
{
    call_session_observer_success(std::move(observer), *desc);
}

void ArcasCreateSessionDescriptionObserver::OnFailure(webrtc::RTCError error)
{
    call_session_observer_failure(std::move(observer), error);
}

ArcasPeerConnection::ArcasPeerConnection(rtc::scoped_refptr<webrtc::PeerConnectionInterface> api) : api(std::move(api))
{
}

void ArcasPeerConnection::create_offer(rust::Box<ArcasRustCreateSessionDescriptionObserver> observer) const
{
    // TODO: This probably has to live longer elsewhere.
    // ArcasCreateSessionDescriptionObserver cxx_observer(std::move(observer));
    auto cxx_observer = rtc::make_ref_counted<ArcasCreateSessionDescriptionObserver>(std::move(observer));

    webrtc::PeerConnectionInterface::RTCOfferAnswerOptions options;
    api->CreateOffer(cxx_observer, options);
}