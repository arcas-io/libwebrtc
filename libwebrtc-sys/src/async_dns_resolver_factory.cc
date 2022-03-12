#include "async_dns_resolver_factory.h"

std::unique_ptr<webrtc::AsyncDnsResolverFactoryInterface> create_arcas_cxx_async_dns_resolver_factory()
{
    webrtc::BasicAsyncResolverFactory fact;
    auto blah = fact.Create();
    auto wrapped = std::make_unique<webrtc::BasicAsyncResolverFactory>();
    blah = wrapped->Create();
    auto result = std::make_unique<webrtc::WrappingAsyncDnsResolverFactory>(std::move(wrapped));
    return std::move(result);
}
