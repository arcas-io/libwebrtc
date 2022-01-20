#pragma once
#include "p2p/base/ice_transport_internal.h"

class ArcasP2PIceConfig
{
private:
    cricket::IceConfig _config;

public:
    cricket::IceConfig get_config() const
    {
        return _config;
    }

    void set_receiving_timeout(int receiving_timeout)
    {
        _config.receiving_timeout = receiving_timeout;
    }

    void set_backup_connection_ping_interval(int backup_connection_ping_interval)
    {
        _config.backup_connection_ping_interval = backup_connection_ping_interval;
    }

    void set_continual_gathering_policy(cricket::ContinualGatheringPolicy continual_gathering_policy)
    {
        _config.continual_gathering_policy = continual_gathering_policy;
    }

    void set_prioritize_most_likely_candidate_pairs(bool prioritize_most_likely_candidate_pairs)
    {
        _config.prioritize_most_likely_candidate_pairs = prioritize_most_likely_candidate_pairs;
    }

    void set_stable_writable_connection_ping_interval(int stable_writable_connection_ping_interval)
    {
        _config.stable_writable_connection_ping_interval = stable_writable_connection_ping_interval;
    }

    void set_presume_writable_when_fully_relayed(bool presume_writable_when_fully_relayed)
    {
        _config.presume_writable_when_fully_relayed = presume_writable_when_fully_relayed;
    }

    void set_surface_ice_candidates_on_ice_transport_type_changed(bool surface_ice_candidates_on_ice_transport_type_changed)
    {
        _config.surface_ice_candidates_on_ice_transport_type_changed = surface_ice_candidates_on_ice_transport_type_changed;
    }

    void set_regather_on_failed_networks_interval(int regather_on_failed_networks_interval)
    {
        _config.regather_on_failed_networks_interval = regather_on_failed_networks_interval;
    }

    void set_receiving_switching_delay(int receiving_switch_delay)
    {
        _config.receiving_switching_delay = receiving_switch_delay;
    }

    void set_default_nomination_mode(cricket::NominationMode default_nomination_mode)
    {
        _config.default_nomination_mode = default_nomination_mode;
    }

    void set_ice_check_interval_strong_connectivity(int ice_check_interval_strong_connectivity)
    {
        _config.ice_check_interval_strong_connectivity = ice_check_interval_strong_connectivity;
    }

    void set_ice_check_interval_weak_connectivity(int ice_check_interval_weak_connectivity)
    {
        _config.ice_check_interval_weak_connectivity = ice_check_interval_weak_connectivity;
    }

    void set_ice_check_min_interval(int ice_check_min_interval)
    {
        _config.ice_check_min_interval = ice_check_min_interval;
    }

    void set_ice_unwritable_timeout(int ice_unwritable_timeout)
    {
        _config.ice_unwritable_timeout = ice_unwritable_timeout;
    }

    void set_ice_unwritable_min_checks(int ice_unwritable_min_checks)
    {
        _config.ice_unwritable_min_checks = ice_unwritable_min_checks;
    }

    void set_ice_inactive_timeout(int ice_inactive_timeout)
    {
        _config.ice_inactive_timeout = ice_inactive_timeout;
    }

    void set_stun_keepalive_interval(int stun_keepalive_interval)
    {
        _config.stun_keepalive_interval = stun_keepalive_interval;
    }

    void set_network_preference(rtc::AdapterType network_preference)
    {
        _config.network_preference = network_preference;
    }
};

std::unique_ptr<ArcasP2PIceConfig> create_arcas_p2p_ice_config();
