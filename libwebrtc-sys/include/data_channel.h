#pragma once
#include "api/data_channel_interface.h"
#include "libwebrtc-sys/include/error.h"
#include "rust/cxx.h"

using ArcasCxxDataState = webrtc::DataChannelInterface::DataState;

class ArcasDataChannelObserverWrapper;
class ArcasDataBuffer;

class ArcasDataChannelObserver : public webrtc::DataChannelObserver
{
private:
    rust::Box<ArcasDataChannelObserverWrapper> api_;

public:
    ArcasDataChannelObserver(rust::Box<ArcasDataChannelObserverWrapper> api)
    : api_(std::move(api))
    {
    }
    void OnStateChange();
    void OnMessage(const webrtc::DataBuffer& buffer);
    void OnBufferedAmountChange(uint64_t sent_data_size);
};


class ArcasDataChannel
{
private:
    rtc::scoped_refptr<webrtc::DataChannelInterface> api;

public:
    ArcasDataChannel(rtc::scoped_refptr<webrtc::DataChannelInterface> api);

    void register_observer(ArcasDataChannelObserver* observer);
    void unregister_observer()
    {
        this->api->UnregisterObserver();
    }
    rust::String label() const
    {
        return rust::String(this->api->label().c_str());
    }
    bool reliable() const
    {
        return this->api->reliable();
    }

    bool ordered() const
    {
        return this->api->ordered();
    }

    uint16_t max_retransmit_time() const
    {
        return this->api->maxRetransmitTime();
    }

    uint16_t max_retransmits() const
    {
        return this->api->maxRetransmits();
    }

    rust::String protocol() const
    {
        return rust::String(this->api->protocol().c_str());
    }

    bool negotiated() const
    {
        return this->api->negotiated();
    }

    int id() const
    {
        return this->api->id();
    }

    webrtc::Priority priority() const
    {
        return this->api->priority();
    }

    ArcasCxxDataState state() const
    {
        return this->api->state();
    }

    std::unique_ptr<ArcasRTCError> error() const
    {
        return std::make_unique<ArcasRTCError>(this->api->error());
    }

    uint32_t messages_sent() const
    {
        return this->api->messages_sent();
    }

    uint64_t bytes_sent() const
    {
        return this->api->bytes_sent();
    }

    uint32_t messages_received() const
    {
        return this->api->messages_received();
    }

    uint64_t bytes_received() const
    {
        return this->api->bytes_received();
    }

    uint64_t buffered_amount() const
    {
        return this->api->buffered_amount();
    }

    void close()
    {
        this->api->Close();
    }

    void send(const ArcasDataBuffer& rust_buf);
};

std::unique_ptr<ArcasDataChannel> gen_unique_data_channel();
std::unique_ptr<ArcasDataChannelObserver>
create_arcas_data_channel_observer(rust::Box<ArcasDataChannelObserverWrapper> api);
