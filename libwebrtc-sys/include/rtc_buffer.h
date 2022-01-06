#pragma once
// Wraps Buffer<uint8_t> class
#include "rtc_base/buffer.h"
#include "rust/cxx.h"

class BufferUint8 {
    public:
    BufferUint8(rtc::BufferT<uint8_t>*);

    rtc::Buffer* GetBuffer() const;
    void append_data(rust::Slice<const uint8_t>);

    private:
    rtc::Buffer* buffer;
};

std::unique_ptr<BufferUint8> gen_unique_rtc_buffer_u8();