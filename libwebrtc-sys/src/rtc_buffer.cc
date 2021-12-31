#include "libwebrtc-sys/include/rtc_buffer.h"

BufferUint8::BufferUint8(rtc::Buffer* buffer): buffer(buffer) {}

rtc::Buffer* BufferUint8::GetBuffer() const {
    return buffer;
}

void BufferUint8::append_data(rust::Slice<const uint8_t> data) {
    buffer->AppendData(data.data(), data.size());
}