#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("include/video_codec.h");
        type ArcasSpatialLayer;

        fn create_arcas_spatial_layer() -> SharedPtr<ArcasSpatialLayer>;
        fn gen_unique_vector_spatial_layers() -> UniquePtr<CxxVector<ArcasSpatialLayer>>;

        // ArcasSpatialLayer
        fn get_width(self: &ArcasSpatialLayer) -> i32;
        fn get_height(self: &ArcasSpatialLayer) -> i32;
        fn get_max_framerate(self: &ArcasSpatialLayer) -> f32;
        fn get_number_of_temporal_layers(self: &ArcasSpatialLayer) -> u8;
        fn get_max_bitrate(self: &ArcasSpatialLayer) -> u32;
        fn get_target_bitrate(self: &ArcasSpatialLayer) -> u32;
        fn get_min_bitrate(self: &ArcasSpatialLayer) -> u32;
        fn get_qp_max(self: &ArcasSpatialLayer) -> u32;
        fn get_active(self: &ArcasSpatialLayer) -> bool;
        fn set_width(self: &ArcasSpatialLayer, width: i32);
        fn set_height(self: &ArcasSpatialLayer, height: i32);
        fn set_max_framerate(self: &ArcasSpatialLayer, max_frame_rate: f32);
        fn set_number_of_temporal_layers(self: &ArcasSpatialLayer, number_of_temporal_layers: u8);
        fn set_max_bitrate(self: &ArcasSpatialLayer, max_bitrate: u32);
        fn set_target_bitrate(self: &ArcasSpatialLayer, target_bitrate: u32);
        fn set_min_bitrate(self: &ArcasSpatialLayer, min_bitrate: u32);
        fn set_qp_max(self: &ArcasSpatialLayer, qp_max: u32);
        fn set_active(self: &ArcasSpatialLayer, active: bool);
    }
}
