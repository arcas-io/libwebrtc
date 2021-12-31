#pragma once
// Wraps Buffer<uint8_t> class
#include "rtc_base/buffer.h"
#include "rust/cxx.h"

class BufferUint8 {
    public:
    BufferUint8(rtc::Buffer*);

    rtc::Buffer* GetBuffer() const;
    void append_data(rust::Slice<const uint8_t>);

    private:
    rtc::Buffer* buffer;
};