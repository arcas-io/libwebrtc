
// #include "cxxgen.h"
#include "libwebrtc-sys/include/peer_connection_session_observers.h"
#include "libwebrtc-sys/src/peer_connection.rs.h"
#include "libwebrtc-sys/src/shared_bridge.rs.h"

ArcasCreateSessionDescriptionObserver::ArcasCreateSessionDescriptionObserver(
    rust::Box<ArcasRustCreateSessionDescriptionObserver> observer) : observer(std::move(observer)) {}

ArcasSetDescriptionObserver::ArcasSetDescriptionObserver(
    rust::Box<ArcasRustSetSessionDescriptionObserver> observer) : observer(std::move(observer)) {}

void ArcasCreateSessionDescriptionObserver::OnSuccess(webrtc::SessionDescriptionInterface *desc)
{
    observer->on_success(std::make_unique<ArcasSessionDescription>(desc->Clone()));
}
void ArcasCreateSessionDescriptionObserver::OnFailure(webrtc::RTCError error)
{
    observer->on_failure(std::move(std::make_unique<ArcasRTCError>(error)));
}

void ArcasSetDescriptionObserver::OnSetLocalDescriptionComplete(webrtc::RTCError error)
{
    if (error.ok())
    {
        observer->on_success();
    }
    else
    {
        observer->on_failure(std::move(std::make_unique<ArcasRTCError>(error)));
    }
}
void ArcasSetDescriptionObserver::OnSetRemoteDescriptionComplete(webrtc::RTCError error)
{
    if (error.ok())
    {
        observer->on_success();
    }
    else
    {
        observer->on_failure(std::move(std::make_unique<ArcasRTCError>(error)));
    }
}

std::shared_ptr<ArcasCreateSessionDescriptionObserver> create_session_description_observer(rust::Box<ArcasRustCreateSessionDescriptionObserver> observer)
{
    return std::make_shared<ArcasCreateSessionDescriptionObserver>(std::move(observer));
}

std::shared_ptr<ArcasSetDescriptionObserver> set_session_description_observer(
    rust::Box<ArcasRustSetSessionDescriptionObserver> observer)
{

    return std::make_shared<ArcasSetDescriptionObserver>(std::move(observer));
}
