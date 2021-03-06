#pragma once

#include <api/candidate.h>
#include <p2p/base/p2p_constants.h>
#include <rtc_base/network_constants.h>

#include "rust/cxx.h"
#include <memory>

enum class CandidateComponent : std::uint8_t;

// Wrapper around cricket::Candidate not to be confused with webrtc::IceCandidate*
class ArcasCandidate
{
private:
    cricket::Candidate _candidate;

public:
    explicit ArcasCandidate()
    : _candidate{}
    {
    }

    ArcasCandidate(cricket::Candidate candidate)
    : _candidate(candidate)
    {
    }

    cricket::Candidate get_candidate() const
    {
        return _candidate;
    }

    rust::String id() const
    {
        return rust::String(_candidate.id().c_str());
    }

    int component() const
    {
        return _candidate.component();
    }
    void set_component(CandidateComponent val);

    rust::String protocol() const
    {
        return rust::String(_candidate.protocol().c_str());
    }
    void set_protocol(rust::String proto)
    {
        _candidate.set_protocol({proto.data(), proto.size()});
    }

    rust::String relay_protocol() const
    {
        return rust::String(_candidate.relay_protocol().c_str());
    }

    rust::String address() const
    {
        return rust::String(_candidate.address().ToString().c_str());
    }

    // Parses hostname:port and [hostname]:port.
    void set_address(rust::String s)
    {
        set_address(std::string{s.data(), s.size()});
    }
    void set_address(std::string s)
    {
        rtc::SocketAddress addr;
        addr.FromString(s);
        _candidate.set_address(addr);
    }

    uint32_t priority() const
    {
        return _candidate.priority();
    }

    float preference() const
    {
        return _candidate.preference();
    }

    rust::String username() const
    {
        return rust::String(_candidate.username().c_str());
    }

    rust::String password() const
    {
        return rust::String(_candidate.password().c_str());
    }

    rust::String candidate_type() const
    {
        return rust::String(_candidate.type().c_str());
    }

    rust::String network_name() const
    {
        return rust::String(_candidate.network_name().c_str());
    }

    rtc::AdapterType network_type() const
    {
        return _candidate.network_type();
    }

    uint32_t generation() const
    {
        return _candidate.generation();
    }

    uint16_t network_cost() const
    {
        return _candidate.network_cost();
    }

    rust::String foundation() const
    {
        return rust::String(_candidate.foundation().c_str());
    }

    rust::String related_address() const
    {
        return rust::String(_candidate.related_address().ToString().c_str());
    }

    rust::String tcptype() const
    {
        return rust::String(_candidate.tcptype().c_str());
    }

    rust::String transport_name() const
    {
        return rust::String(_candidate.transport_name().c_str());
    }

    rust::String url() const
    {
        return rust::String(_candidate.url().c_str());
    }

    bool is_equivalent(const ArcasCandidate& other) const
    {
        return _candidate.IsEquivalent(other._candidate);
    }

    rust::String cxx_to_string() const
    {
        return rust::String(_candidate.ToString().c_str());
    }

    rust::String to_sensitive_string() const
    {
        return rust::String(_candidate.ToSensitiveString().c_str());
    }
};

inline std::unique_ptr<ArcasCandidate> create_arcas_candidate()
{
    return std::make_unique<ArcasCandidate>();
}
