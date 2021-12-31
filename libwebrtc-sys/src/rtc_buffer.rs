use cxx::UniquePtr;

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("include/rtc_buffer.h");
        type BufferUint8;

        fn append_data(self: Pin<&mut BufferUint8>, data: &[u8]);
        fn gen_unique_rtc_buffer_u8() -> UniquePtr<BufferUint8>;
    }
}