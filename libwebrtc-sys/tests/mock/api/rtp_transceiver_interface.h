#pragma once

#include "../unimp.h"
#include <api/rtp_transceiver_interface.h>

namespace webrtc
{
namespace
{
    class test_webrtc_peer_rtp_transceiver final : public webrtc::RtpTransceiverInterface
    {
        virtual void AddRef() const {}
        virtual rtc::RefCountReleaseStatus Release() const
        {
            return rtc::RefCountReleaseStatus::kOtherRefsRemained;
        }

        virtual cricket::MediaType media_type() const UNIMP
            virtual absl::optional<std::string> mid()
                const UNIMP rtc::scoped_refptr<RtpSenderInterface> sender_;
        rtc::scoped_refptr<RtpReceiverInterface> receiver_;
        virtual rtc::scoped_refptr<RtpSenderInterface> sender() const
        {
            return sender_;
        }
        virtual rtc::scoped_refptr<RtpReceiverInterface> receiver() const
        {
            return receiver_;
        }
        virtual bool stopped() const UNIMP virtual bool stopping() const UNIMP
            virtual RtpTransceiverDirection direction() const UNIMP
            virtual void SetDirection(RtpTransceiverDirection new_direction) UNIMP virtual RTCError
            SetDirectionWithError(RtpTransceiverDirection new_direction) UNIMP
            virtual absl::optional<RtpTransceiverDirection> current_direction() const UNIMP
    };
}//namespace
}//namespace webrtc
