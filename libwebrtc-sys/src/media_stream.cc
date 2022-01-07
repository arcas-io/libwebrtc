
#include "libwebrtc-sys/include/media_stream.h"

ArcasMediaStream::ArcasMediaStream(rtc::scoped_refptr<webrtc::MediaStreamInterface> api)
: api(api)
{
}