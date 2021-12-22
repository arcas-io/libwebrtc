#include "rust/cxx.h"
#include "libwebrtc-sys/src/shared_bridge.rs.h"
#include "libwebrtc-sys/src/peer_connection.rs.h"
#include "libwebrtc-sys/include/peer_connection_stats_callback.h"
#include "api/stats/rtcstats_objects.h"

ArcasRTCStatsCollectorCallback::ArcasRTCStatsCollectorCallback(rust::Box<ArcasRustRTCStatsCollectorCallback> cb) : cb(std::move(cb)) {}

void ArcasRTCStatsCollectorCallback::OnStatsDelivered(const rtc::scoped_refptr<const webrtc::RTCStatsReport> &report)
{

    auto inbound_stream_stats = report->GetStatsOfType<webrtc::RTCInboundRTPStreamStats>();
    auto outbound_stream_stats = report->GetStatsOfType<webrtc::RTCOutboundRTPStreamStats>();

    rust::Vec<ArcasVideoSenderStats> out_video;
    rust::Vec<ArcasAudioSenderStats> out_audio;
    rust::Vec<ArcasVideoReceiverStats> in_video;
    rust::Vec<ArcasAudioReceiverStats> in_audio;

    // Inbound
    for (const auto &stat : inbound_stream_stats)
    {

        // record inbound video stats
        if (*stat->kind == "video")
        {
            ArcasVideoReceiverStats recv = {0};
            recv.ssrc = stat->ssrc.ValueOrDefault(0);
            recv.frame_height = stat->frame_height.ValueOrDefault(0);
            recv.frame_width = stat->frame_width.ValueOrDefault(0);
            recv.bytes_received = stat->bytes_received.ValueOrDefault(0);
            recv.frames_decoded = stat->frames_decoded.ValueOrDefault(0);
            recv.keyframes_decoded = stat->key_frames_decoded.ValueOrDefault(0);
            recv.frames_dropped = stat->frames_dropped.ValueOrDefault(0);
            recv.packets_lost = stat->packets_lost.ValueOrDefault(0);
            recv.packets_received = stat->packets_received.ValueOrDefault(0);
            recv.packets_repaired = stat->packets_repaired.ValueOrDefault(0);
            recv.total_decode_time = stat->total_decode_time.ValueOrDefault(0.0f);

            in_video.push_back(recv);
        }

        // record inbound audio stats
        else if (*stat->kind == "audio")
        {
            ArcasAudioReceiverStats receiver = {0};
            receiver.ssrc = stat->ssrc.ValueOrDefault(0);
            receiver.packets_received = stat->packets_received.ValueOrDefault(0);
            receiver.packets_lost = stat->packets_lost.ValueOrDefault(0);
            receiver.bytes_received = stat->bytes_received.ValueOrDefault(0);
            receiver.jitter = stat->jitter.ValueOrDefault(0.0);
            receiver.frames_decoded = stat->frames_decoded.ValueOrDefault(0);
            receiver.total_decode_time = stat->total_decode_time.ValueOrDefault(0.0);

            if (stat->track_id.is_defined())
            {
                auto track_stat = report->GetAs<webrtc::RTCMediaStreamTrackStats>(*stat->track_id);
                if (track_stat)
                {
                    receiver.audio_level = track_stat->audio_level.ValueOrDefault(0.0);
                    receiver.total_audio_energy = track_stat->total_audio_energy.ValueOrDefault(0.0);
                }
            }

            in_audio.push_back(receiver);
        }
    }

    // Outbound
    for (const auto &stat : outbound_stream_stats)
    {

        // record outbound video stats
        if (*stat->kind == "video")
        {
            ArcasVideoSenderStats send = {0};
            send.ssrc = stat->ssrc.ValueOrDefault(0);
            send.packets_sent = stat->packets_sent.ValueOrDefault(0);
            send.bytes_sent = stat->bytes_sent.ValueOrDefault(0);
            send.frames_encoded = stat->frames_encoded.ValueOrDefault(0);
            send.key_frames_encoded = stat->key_frames_encoded.ValueOrDefault(0);
            send.total_encode_time = stat->total_encode_time.ValueOrDefault(0.0);
            send.frame_width = stat->frame_width.ValueOrDefault(0);
            send.frame_height = stat->frame_height.ValueOrDefault(0);
            send.retransmitted_packets_sent = stat->retransmitted_packets_sent.ValueOrDefault(0);
            send.retransmitted_bytes_sent = stat->retransmitted_bytes_sent.ValueOrDefault(0);
            send.total_packet_send_delay = stat->total_packet_send_delay.ValueOrDefault(0.0);
            send.nack_count = stat->nack_count.ValueOrDefault(0);
            send.fir_count = stat->fir_count.ValueOrDefault(0);
            send.pli_count = stat->pli_count.ValueOrDefault(0);

            if (stat->quality_limitation_reason.is_defined())
            {
                // "none" = 0 (the default)
                if (*stat->quality_limitation_reason == "cpu")
                {
                    send.quality_limitation_reason = 1;
                }
                else if (*stat->quality_limitation_reason == "bandwidth")
                {
                    send.quality_limitation_reason = 2;
                }
                else
                {
                    send.quality_limitation_reason = 3;
                }
            }
            send.quality_limitation_resolution_changes = stat->quality_limitation_resolution_changes.ValueOrDefault(0);

            if (stat->remote_id.is_defined())
            {
                auto remote_stat = report->GetAs<webrtc::RTCRemoteInboundRtpStreamStats>(*stat->remote_id);
                if (remote_stat)
                {
                    send.remote_packets_lost = remote_stat->packets_lost.ValueOrDefault(0);
                    send.remote_jitter = remote_stat->jitter.ValueOrDefault(0.0);
                    send.remote_round_trip_time = remote_stat->round_trip_time.ValueOrDefault(0.0);
                }
            }

            out_video.push_back(send);
        }

        else if (*stat->kind == "audio")
        {
            ArcasAudioSenderStats send = {0};
            send.ssrc = stat->ssrc.ValueOrDefault(0);
            send.packets_sent = stat->packets_sent.ValueOrDefault(0);
            send.bytes_sent = stat->bytes_sent.ValueOrDefault(0);

            if (stat->remote_id.is_defined())
            {
                auto remote_stat = report->GetAs<webrtc::RTCRemoteInboundRtpStreamStats>(*stat->remote_id);
                if (remote_stat)
                {
                    send.remote_packets_lost = remote_stat->packets_lost.ValueOrDefault(0);
                    send.remote_jitter = remote_stat->jitter.ValueOrDefault(0.0);
                    send.remote_round_trip_time = remote_stat->round_trip_time.ValueOrDefault(0.0);
                }
            }

            if (stat->media_source_id.is_defined())
            {
                auto audio_source_stat = report->GetAs<webrtc::RTCAudioSourceStats>(*stat->media_source_id);
                if (audio_source_stat)
                {
                    send.audio_level = audio_source_stat->audio_level.ValueOrDefault(0.0);
                    send.total_audio_energy = audio_source_stat->total_audio_energy.ValueOrDefault(0.0);
                }
            }

            out_audio.push_back(send);
        }
    }

    cb->on_stats_delivered(
        in_video,
        in_audio,
        out_video,
        out_audio);
}
