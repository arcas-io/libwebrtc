#[cxx::bridge]
pub mod ffi {

    unsafe extern "C++" {
        include!("include/async_dns_resolver_factory.h");

        #[namespace = "webrtc"]
        type AsyncDnsResolverFactoryInterface;

        fn create_arcas_cxx_async_dns_resolver_factory(
        ) -> UniquePtr<AsyncDnsResolverFactoryInterface>;
    }
}
