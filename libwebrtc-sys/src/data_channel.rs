#[cxx::bridge]
pub mod ffi {

    #[derive(Debug)]
    #[repr(u32)]
    #[namespace = "webrtc"]
    enum Priority {
        kVeryLow,
        kLow,
        kMedium,
        kHigh,
    }

    /// Vectors are used as optional types in C++.
    /// Zero length = none
    /// One length = some
    #[derive(Debug)]
    struct ArcasDataChannelInit {
        reliable: bool,
        ordered: bool,
        max_retransmit_time: Vec<i32>,
        max_retransmits: Vec<i32>,
        protocol: String,
        id: Vec<u16>,
        priority: Vec<Priority>,
    }

    struct ArcasDataBuffer {
        ptr: *const u8,
        len: usize,
        binary: bool,
    }

    #[derive(Debug)]
    #[repr(u32)]
    enum ArcasCxxDataState {
        kConnecting,
        kOpen, // The DataChannel is ready to send data.
        kClosing,
        kClosed,
    }

    unsafe extern "C++" {
        include!("include/data_channel.h");

        type ArcasCxxDataState;
        type ArcasDataChannelObserver;
        #[namespace = "webrtc"]
        type DataBuffer;

        #[namespace = "webrtc"]
        type Priority;
        type ArcasDataChannel;
        type ArcasRTCError = crate::error::ffi::ArcasRTCError;

        // ArcasDataChannel
        fn create_arcas_data_channel_observer(
            wrapper: Box<ArcasDataChannelObserverWrapper>,
        ) -> UniquePtr<ArcasDataChannelObserver>;

        /// # Safety
        ///
        /// Observer must be kept alive until the DataChannel is destroyed.
        ///
        unsafe fn register_observer(
            self: Pin<&mut ArcasDataChannel>,
            observer: *mut ArcasDataChannelObserver,
        );
        fn unregister_observer(self: Pin<&mut ArcasDataChannel>);
        fn label(self: &ArcasDataChannel) -> String;
        fn reliable(self: &ArcasDataChannel) -> bool;
        fn ordered(self: &ArcasDataChannel) -> bool;
        fn max_retransmit_time(self: &ArcasDataChannel) -> u16;
        fn max_retransmits(self: &ArcasDataChannel) -> u16;
        fn protocol(self: &ArcasDataChannel) -> String;
        fn negotiated(self: &ArcasDataChannel) -> bool;
        fn id(self: &ArcasDataChannel) -> i32;
        fn priority(self: &ArcasDataChannel) -> Priority;
        fn state(self: &ArcasDataChannel) -> ArcasCxxDataState;
        fn error(self: &ArcasDataChannel) -> UniquePtr<ArcasRTCError>;
        fn messages_sent(self: &ArcasDataChannel) -> u32;
        fn bytes_sent(self: &ArcasDataChannel) -> u64;
        fn messages_received(self: &ArcasDataChannel) -> u32;
        fn bytes_received(self: &ArcasDataChannel) -> u64;
        fn buffered_amount(self: &ArcasDataChannel) -> u64;
        fn close(self: Pin<&mut ArcasDataChannel>);
        fn send(self: Pin<&mut ArcasDataChannel>, data: &ArcasDataBuffer);

        fn gen_unique_data_channel() -> UniquePtr<ArcasDataChannel>;
    }

    extern "Rust" {
        type ArcasDataChannelObserverWrapper;

        // ArcasDataChannelObserverWrapper
        fn on_state_change(self: &ArcasDataChannelObserverWrapper);
        fn on_message(self: &ArcasDataChannelObserverWrapper, data: ArcasDataBuffer);
        fn on_buffered_amount_change(self: &ArcasDataChannelObserverWrapper, sent_data_size: u64);
    }
}

pub trait DataChannelObserverImpl {
    fn on_state_change(&self);
    fn on_message(&self, slice: &[u8], is_binary: bool);
    fn on_buffered_amount_change(&self, sent_data_size: u64);
}

pub struct ArcasDataChannelObserverWrapper {
    inner: Box<dyn DataChannelObserverImpl>,
}

impl ArcasDataChannelObserverWrapper {
    pub fn new(inner: Box<dyn DataChannelObserverImpl>) -> Self {
        Self { inner }
    }

    pub fn on_state_change(&self) {
        self.inner.on_state_change()
    }

    pub fn on_message(&self, buffer: ffi::ArcasDataBuffer) {
        let slice = unsafe { std::slice::from_raw_parts(buffer.ptr, buffer.len) };
        self.inner.on_message(slice, buffer.binary)
    }

    pub fn on_buffered_amount_change(&self, sent_data_size: u64) {
        self.inner.on_buffered_amount_change(sent_data_size)
    }
}
