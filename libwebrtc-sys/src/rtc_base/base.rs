#[cxx::bridge]
pub mod ffi {
    #[derive(Debug)]
    #[repr(u32)]
    enum ArcasCxxSSLHandshakeError {
        UNKNOWN,
        INCOMPATIBLE_CIPHERSUITE,
        MAX_VALUE,
    }

    #[derive(Debug)]
    #[repr(u32)]
    #[namespace = "rtc"]
    enum AdapterType {
        // This enum resembles the one in Chromium net::ConnectionType.
        ADAPTER_TYPE_UNKNOWN = 0,
        ADAPTER_TYPE_ETHERNET = 1,
        ADAPTER_TYPE_WIFI = 2,
        ADAPTER_TYPE_CELLULAR = 4, // This is CELLULAR of unknown type.
        ADAPTER_TYPE_VPN = 8,
        ADAPTER_TYPE_LOOPBACK = 16,
        // ADAPTER_TYPE_ANY is used for a network, which only contains a single "any
        // address" IP address (INADDR_ANY for IPv4 or in6addr_any for IPv6), and can
        // use any/all network interfaces. Whereas ADAPTER_TYPE_UNKNOWN is used
        // when the network uses a specific interface/IP, but its interface type can
        // not be determined or not fit in this enum.
        ADAPTER_TYPE_ANY = 32,
        ADAPTER_TYPE_CELLULAR_2G = 64,
        ADAPTER_TYPE_CELLULAR_3G = 128,
        ADAPTER_TYPE_CELLULAR_4G = 256,
        ADAPTER_TYPE_CELLULAR_5G = 512,
    }

    unsafe extern "C++" {
        include!("include/rtc_base/base.h");

        #[namespace = "rtc"]
        type Thread;
        #[namespace = "rtc"]
        type NetworkManager;
        #[namespace = "rtc"]
        type AdapterType;

        type ArcasCxxSSLHandshakeError;

        // rtc::Thread
        #[cxx_name = "Quit"]
        fn quit(self: Pin<&mut Thread>);
        #[cxx_name = "IsQuitting"]
        fn is_quitting(self: Pin<&mut Thread>) -> bool;
        #[cxx_name = "Restart"]
        fn restart(self: Pin<&mut Thread>);
        #[cxx_name = "IsCurrent"]
        fn is_current(self: &Thread) -> bool;
        #[cxx_name = "Start"]
        fn start(self: Pin<&mut Thread>) -> bool;
        #[cxx_name = "Run"]
        fn run(self: Pin<&mut Thread>);

        // ArcasThread
        fn create_arcas_cxx_thread() -> UniquePtr<Thread>;

        // Network Manager
        fn create_arcas_cxx_network_manager() -> UniquePtr<NetworkManager>;

        /// # Safety
        ///
        /// Must be passed valid Thread object. This will not take ownership.
        unsafe fn arcas_cxx_thread_post_task(thread: *mut Thread, task: Box<QueuedTask>);
    }

    extern "Rust" {
        #[rust_name = "QueuedTask"]
        type ArcasRustQueuedTask;
        fn run(self: &QueuedTask) -> bool;
    }
}

pub struct QueuedTask {
    callback: Box<dyn Fn() -> bool>,
}

impl QueuedTask {
    pub fn new(callback: Box<dyn Fn() -> bool>) -> Self {
        Self { callback }
    }

    pub fn run(&self) -> bool {
        (self.callback)()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
