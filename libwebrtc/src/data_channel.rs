use cxx::UniquePtr;
use libwebrtc_sys::data_channel::{
    ffi::{
        create_arcas_data_channel_observer, ArcasCxxDataState, ArcasDataBuffer, ArcasDataChannel,
        ArcasDataChannelInit, ArcasDataChannelObserver, Priority,
    },
    ArcasDataChannelObserverWrapper, DataChannelObserverImpl,
};
use tokio::sync::mpsc::Sender;

use crate::{error::Result, send_event};

#[derive(Debug, Default)]
pub struct DataChannelInit {
    /// Deprecated. Reliability is assumed, and channel will be unreliable if
    /// maxRetransmitTime or MaxRetransmits is set.
    pub reliable: Option<bool>,

    /// True if ordered delivery is required.
    pub ordered: Option<bool>,

    /// The max period of time in milliseconds in which retransmissions will be
    /// sent. After this time, no more retransmissions will be sent.
    ///
    /// Cannot be set along with `maxRetransmits`.
    /// This is called `maxPacketLifeTime` in the WebRTC JS API.
    /// Negative values are ignored, and positive values are clamped to [0-65535]
    pub max_retransmit_time: Option<i32>,

    /// The max number of retransmissions.
    ///
    /// Cannot be set along with `maxRetransmitTime`.
    /// Negative values are ignored, and positive values are clamped to [0-65535]
    pub max_retransmits: Option<i32>,

    /// This is set by the application and opaque to the WebRTC implementation.
    pub protocol: String,
    // Session id ... if not provided one will be automatically generated.
    pub id: Option<u16>,

    /// https://w3c.github.io/webrtc-priority/#new-rtcdatachannelinit-member
    pub priority: Option<Priority>,
}

impl From<DataChannelInit> for ArcasDataChannelInit {
    fn from(init: DataChannelInit) -> Self {
        ArcasDataChannelInit {
            reliable: init.reliable.unwrap_or(true),
            ordered: init.ordered.unwrap_or(false),
            max_retransmit_time: match init.max_retransmit_time {
                Some(v) => vec![v],
                None => vec![],
            },
            max_retransmits: match init.max_retransmits {
                Some(v) => vec![v],
                None => vec![],
            },
            protocol: init.protocol,
            id: match init.id {
                Some(v) => vec![v],
                None => vec![],
            },
            priority: match init.priority {
                Some(v) => vec![v],
                None => vec![],
            },
        }
    }
}

pub struct DataChannelMessage {
    pub data: Vec<u8>,
    pub binary: bool,
}

type OnMessageCallback = Box<dyn Fn(&[u8], bool)>;

#[derive(Default)]
pub struct DataChannelSenders {
    pub on_state_change: Option<Sender<()>>,
    // A boxed function is used here so that messages can be passed
    // by reference.
    pub on_message: Option<OnMessageCallback>,
    pub on_buffered_amount_change: Option<Sender<u64>>,
}

impl DataChannelObserverImpl for DataChannelSenders {
    fn on_state_change(&self) {
        send_event!(self.on_state_change, ());
    }

    fn on_message(&self, slice: &[u8], binary: bool) {
        if let Some(ref callback) = self.on_message {
            (callback)(slice, binary)
        }
    }

    fn on_buffered_amount_change(&self, sent_data_size: u64) {
        send_event!(self.on_buffered_amount_change, sent_data_size);
    }
}

pub struct DataChannel {
    inner: UniquePtr<ArcasDataChannel>,

    // Held for C++
    #[allow(unused)]
    current_observer: Option<UniquePtr<ArcasDataChannelObserver>>,
}

impl DataChannel {
    pub(crate) fn new(inner: UniquePtr<ArcasDataChannel>) -> Self {
        Self {
            inner,
            current_observer: None,
        }
    }

    pub fn observe(&mut self, senders: DataChannelSenders) {
        let mut wrapper = create_arcas_data_channel_observer(Box::new(
            ArcasDataChannelObserverWrapper::new(Box::new(senders)),
        ));
        self.inner.pin_mut().unregister_observer();
        unsafe {
            self.inner
                .pin_mut()
                .register_observer(wrapper.pin_mut().get_unchecked_mut());
        }
        self.current_observer = Some(wrapper);
    }

    pub fn label(&self) -> String {
        self.inner.label()
    }
    pub fn reliable(&self) -> bool {
        self.inner.reliable()
    }
    pub fn ordered(&self) -> bool {
        self.inner.ordered()
    }
    pub fn max_retransmit_time(&self) -> u16 {
        self.inner.max_retransmit_time()
    }
    pub fn max_retransmits(&self) -> u16 {
        self.inner.max_retransmits()
    }
    pub fn protocol(&self) -> String {
        self.inner.protocol()
    }
    pub fn negotiated(&self) -> bool {
        self.inner.negotiated()
    }
    pub fn id(&self) -> i32 {
        self.inner.id()
    }
    pub fn priority(&self) -> Priority {
        self.inner.priority()
    }
    pub fn state(&self) -> ArcasCxxDataState {
        self.inner.state()
    }

    /// Last error or ()
    pub fn error(&self) -> Result<()> {
        let err = self.inner.error();
        match err.ok() {
            false => Err(err.into()),
            true => Ok(()),
        }
    }
    pub fn messages_sent(&self) -> u32 {
        self.inner.messages_sent()
    }
    pub fn bytes_sent(&self) -> u64 {
        self.inner.bytes_sent()
    }
    pub fn messages_received(&self) -> u32 {
        self.inner.messages_received()
    }
    pub fn bytes_received(&self) -> u64 {
        self.inner.bytes_received()
    }

    pub fn buffered_bytes(&self) -> u64 {
        self.inner.buffered_amount()
    }

    /// Explicitly close the data channel.
    ///
    /// NOTE: This does not need to be called as on drop this will be invoked.
    pub fn close(&mut self) {
        self.inner.pin_mut().close();
    }

    /// Send a message over the data channel.
    ///
    /// # Arguments
    ///
    /// * `data` - The data to send.
    /// * `binary` - true = binary, false = utf8 string.
    ///
    pub fn send(&mut self, data: &[u8], binary: bool) {
        let buffer = ArcasDataBuffer {
            ptr: data.as_ptr(),
            len: data.len(),
            binary,
        };
        self.inner.pin_mut().send(&buffer);
    }
}

impl Drop for DataChannel {
    fn drop(&mut self) {
        self.inner.pin_mut().close();
    }
}
