use cxx::UniquePtr;
use libwebrtc_sys::rtc_base::base::{
    ffi::{arcas_cxx_thread_post_task, create_arcas_cxx_thread, Thread},
    QueuedTask,
};

pub struct RTCThread {
    // Holds reference into C++
    #[allow(unused)]
    inner: UniquePtr<Thread>,
}

impl RTCThread {
    pub fn new() -> Self {
        Self {
            inner: create_arcas_cxx_thread(),
        }
    }

    pub fn quit(&mut self) {
        self.inner.pin_mut().quit();
    }

    pub fn is_quitting(&mut self) -> bool {
        self.inner.pin_mut().is_quitting()
    }

    pub fn restart(&mut self) {
        self.inner.pin_mut().restart();
    }

    pub fn is_current(&self) -> bool {
        self.inner.is_current()
    }

    pub fn start(&mut self) -> bool {
        self.inner.pin_mut().start()
    }

    pub fn run(&mut self) {
        self.inner.pin_mut().run();
    }

    pub fn post_task(&mut self, task: Box<QueuedTask>) {
        // self.inner.pin_mut().post_task(task);
        unsafe {
            arcas_cxx_thread_post_task(self.inner.pin_mut().get_unchecked_mut(), task);
        }
    }
}

impl Default for RTCThread {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for RTCThread {
    fn drop(&mut self) {
        self.quit();
    }
}

#[cfg(test)]
pub mod test {
    use std::sync::mpsc::channel;

    use super::*;

    #[test]
    fn test_threads() {
        let mut thread = RTCThread::new();
        assert!(!thread.is_current());
        let (tx, rx) = channel::<()>();
        thread.start();

        thread.post_task(Box::new(QueuedTask::new(Box::new(move || {
            tx.send(()).unwrap();
            true
        }))));

        rx.recv().unwrap();
        thread.quit();
    }
}
