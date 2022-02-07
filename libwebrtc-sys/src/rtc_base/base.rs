use std::os::raw::c_char;

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
        #[namespace = "rtc"]
        type ByteBufferReader;
        #[namespace = "rtc"]
        type ByteBufferWriter;

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

        // ByteBufferWriter

        // ByteBufferReader

        /// # Safety
        ///
        /// Memory must be managed outside of the pointer passed here.
        /// It must live as long as the pointer is valid.
        unsafe fn create_arcas_cxx_byte_buffer_writer(
            ptr: *mut c_char,
            size: usize,
        ) -> UniquePtr<ByteBufferWriter>;

        /// # Safety
        ///
        /// The pointer must be valid for the lifetime of the object.
        unsafe fn create_arcas_cxx_byte_buffer_reader(
            ptr: *mut c_char,
            size: usize,
        ) -> UniquePtr<ByteBufferReader>;

        #[cxx_name = "Data"]
        fn data(self: &ByteBufferReader) -> *const c_char;
        #[cxx_name = "Length"]
        fn len(self: &ByteBufferReader) -> usize;
        #[cxx_name = "ReadUInt8"]
        unsafe fn read_uint8(self: Pin<&mut ByteBufferReader>, out_val: *mut u8) -> bool;

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
    fn it_creates_byte_buffer() {
        let mut slice = [10i8, 1i8, 2i8, 3i8];
        let mut reader = unsafe {
            super::ffi::create_arcas_cxx_byte_buffer_reader(slice.as_mut_ptr(), slice.len())
        };

        let mut output = [0u8; 10];
        unsafe {
            assert!(reader.pin_mut().read_uint8(output.as_mut_ptr()));
        }
        assert_eq!(10u8, output[0]);
    }
}
