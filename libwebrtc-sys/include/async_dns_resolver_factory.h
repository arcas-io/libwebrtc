#pragma once
#include "p2p/base/basic_async_resolver_factory.h"

std::unique_ptr<webrtc::AsyncDnsResolverFactoryInterface> create_arcas_cxx_async_dns_resolver_factory();
