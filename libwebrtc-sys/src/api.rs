#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("include/api.h");
        type ArcasAPI;
        type ArcasPeerConnectionFactory =
            crate::peer_connection_factory::ffi::ArcasPeerConnectionFactory;
        type ArcasVideoEncoderFactory = crate::video_encoding::ffi::ArcasVideoEncoderFactory;
        type ArcasPeerConnectionFactoryConfig =
            crate::peerconnection_factory_config::ffi::ArcasPeerConnectionFactoryConfig;

        // wrapper functions around constructors.
        fn create_arcas_api() -> UniquePtr<ArcasAPI>;

        fn create_factory(self: &ArcasAPI) -> UniquePtr<ArcasPeerConnectionFactory>;

        fn create_factory_with_arcas_video_encoder_factory(
            self: &ArcasAPI,
            video_encoder_factory: UniquePtr<ArcasVideoEncoderFactory>,
        ) -> UniquePtr<ArcasPeerConnectionFactory>;

        fn create_factory_with_config(
            self: &ArcasAPI,
            config: UniquePtr<ArcasPeerConnectionFactoryConfig>,
        ) -> UniquePtr<ArcasPeerConnectionFactory>;
    }
}
