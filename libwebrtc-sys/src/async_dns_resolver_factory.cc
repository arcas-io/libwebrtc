#include "libwebrtc-sys/include/async_dns_resolver_factory.h"

std::unique_ptr<webrtc::AsyncDnsResolverFactoryInterface> create_arcas_cxx_async_dns_resolver_factory()
{
    return std::make_unique<webrtc::WrappingAsyncDnsResolverFactory>(
        std::make_unique<webrtc::BasicAsyncResolverFactory>());
}
