#include "libwebrtc-sys/include/webrtc_api.h"
#include "libwebrtc-sys/include/ice_candidate.h"
#include "libwebrtc-sys/include/sdp_video_format.h"
#include "libwebrtc-sys/include/utils.h"
#include "libwebrtc-sys/include/video_codec.h"
#include "libwebrtc-sys/include/encoded_image_factory.h"
#include "libwebrtc-sys/include/reactive_video_encoder_wrapper.h"
#include "libwebrtc-sys/include/video_encoder.h"
#include "libwebrtc-sys/include/video_encoder_factory.h"
#include "libwebrtc-sys/include/media_stream.h"
#include "libwebrtc-sys/include/data_channel.h"
#include "libwebrtc-sys/include/session_description.h"
#include "libwebrtc-sys/include/peer_connection_session_observers.h"
#include "libwebrtc-sys/include/peer_connection_stats_callback.h"
#include "libwebrtc-sys/include/peer_connection.h"
#include "libwebrtc-sys/include/data_channel.h"
#include "libwebrtc-sys/include/rtp_receiver.h"
#include "libwebrtc-sys/include/video_frame.h"
#include "libwebrtc-sys/include/peer_connection_factory.h"
#include "libwebrtc-sys/include/rtp_parameters.h"
#include "libwebrtc-sys/include/color_space.h"
#include "libwebrtc-sys/include/video_track_source.h"
#include "libwebrtc-sys/include/api.h"
#include "libwebrtc-sys/include/logging.h"
#include "libwebrtc-sys/include/video_encoding_wrapper.h"


// The below header statements will cause Cxx to generate UniquePtrTarget types
// for anything returned by these functions. This is required for some reason and is
// not done automatically just by using these types.

std::unique_ptr<ArcasDataChannel> gen_unique_ptr1();
std::unique_ptr<ArcasMediaStream> gen_unique_ptr2();
std::unique_ptr<ArcasVideoCodecSettings> gen_unique_ptr3();
std::unique_ptr<ArcasPeerConnectionConfig> gen_unique_ptr4();
std::unique_ptr<ArcasSpatialLayer> gen_unique_ptr5();
std::shared_ptr<ArcasCxxEncodedImage> gen_shared_ptr1();
std::shared_ptr<ArcasCodecSpecificInfo> gen_shared_ptr2();
