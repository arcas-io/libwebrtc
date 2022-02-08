
#include "libwebrtc-sys/include/data_channel.h"
#include "libwebrtc-sys/src/data_channel.rs.h"

void ArcasDataChannelObserver::OnStateChange()
{
    api_->on_state_change();
}

void ArcasDataChannelObserver::OnMessage(const webrtc::DataBuffer& buffer)
{
    ArcasDataBuffer rust_buffer = {
        .ptr = buffer.data.data(),
        .len = buffer.data.size(),
        .binary = buffer.binary,
    };
    api_->on_message(std::move(rust_buffer));
}

void ArcasDataChannelObserver::OnBufferedAmountChange(uint64_t sent_data_size)
{
    api_->on_buffered_amount_change(sent_data_size);
}


ArcasDataChannel::ArcasDataChannel(rtc::scoped_refptr<webrtc::DataChannelInterface> api)
: api(api)
{
}

void ArcasDataChannel::register_observer(ArcasDataChannelObserver* observer)
{
    this->api->RegisterObserver(observer);
}

void ArcasDataChannel::send(const ArcasDataBuffer& buffer)
{
    webrtc::DataBuffer webrtc_buffer(rtc::CopyOnWriteBuffer(buffer.ptr, buffer.len), buffer.binary);
    this->api->Send(webrtc_buffer);
}

std::unique_ptr<ArcasDataChannelObserver>
create_arcas_data_channel_observer(rust::Box<ArcasDataChannelObserverWrapper> api)
{
    return std::make_unique<ArcasDataChannelObserver>(std::move(api));
}
