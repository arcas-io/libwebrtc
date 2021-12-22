#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("include/alias.h");
        include!("include/encoded_image_factory.h");
        type ArcasEncodedImageFactory;
        type ArcasOpaqueEncodedImageBuffer;
        type ArcasCxxEncodedImage = crate::shared_bridge::ffi::ArcasCxxEncodedImage;

        fn create_arcas_encoded_image_factory() -> UniquePtr<ArcasEncodedImageFactory>;

        fn create_empty_encoded_image_buffer(
            self: &ArcasEncodedImageFactory,
        ) -> SharedPtr<ArcasOpaqueEncodedImageBuffer>;

        fn create_encoded_image(self: &ArcasEncodedImageFactory)
            -> UniquePtr<ArcasCxxEncodedImage>;

        /// Create a new ArcasEncodedImageFactory
        ///
        /// # Safety
        ///
        /// This will *not* copy underlying memory.
        unsafe fn create_encoded_image_buffer(
            self: &ArcasEncodedImageFactory,
            data: *const u8,
            size: usize,
        ) -> SharedPtr<ArcasOpaqueEncodedImageBuffer>;

    }
}
