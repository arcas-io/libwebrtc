#pragma once
#include "api/transport/stun.h"
#include "rust/cxx.h"

using ArcasCxxIntegrityStatus = cricket::StunMessage::IntegrityStatus;

class ArcasICEMessage : public cricket::IceMessage
{
public:
    int get_type() const
    {
        return this->type();
    }

    std::unique_ptr<std::vector<uint16_t>> unknown_attributes() const
    {
        return std::make_unique<std::vector<uint16_t>>(this->GetNonComprehendedAttributes());
    }

    rust::String rust_password() const
    {
        return rust::String(this->password().c_str());
    }

    bool rust_add_message_integrity(rust::String password)
    {
        std::string cxx_password{password.data(), password.size()};
        return this->AddMessageIntegrity(cxx_password);
    }

    bool rust_add_message_integrity32(rust::String password)
    {
        absl::string_view cxx_password{password.data(), password.size()};
        return this->AddMessageIntegrity32(cxx_password);
    }

    void set_transaction_id(const std::string& transaction_id)
    {
        this->SetTransactionID(transaction_id);
    }
};

std::unique_ptr<ArcasICEMessage> create_arcas_ice_message();
