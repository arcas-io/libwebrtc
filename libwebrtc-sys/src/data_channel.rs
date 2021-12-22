#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("include/data_channel.h");
        type ArcasDataChannel;
        fn gen_unique_data_channel() -> UniquePtr<ArcasDataChannel>;
    }
}
