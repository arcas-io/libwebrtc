#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("include/media_stream.h");
        type ArcasMediaStream;

        fn gen_unique_media_stream() -> UniquePtr<ArcasMediaStream>;
    }
}
