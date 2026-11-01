//! Bridge Wintun's wait event into the one Mio reactor waker. Acknowledging
//! after the receive ring drains avoids spinning on a signaled manual event.
use std::{
    io,
    sync::{
        Arc,
        atomic::{AtomicU32, Ordering},
    },
    thread::{self, JoinHandle},
};
use windows_sys::Win32::{
    Foundation::{CloseHandle, HANDLE, WAIT_FAILED, WAIT_OBJECT_0},
    System::Threading::{CreateEventW, INFINITE, SetEvent, WaitForMultipleObjects},
};

struct Event(HANDLE);
impl Event {
    fn new(manual: bool) -> io::Result<Self> {
        // SAFETY: unnamed event, default security, initially nonsignaled.
        let handle = unsafe { CreateEventW(std::ptr::null(), manual.into(), 0, std::ptr::null()) };
        if handle == 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(Self(handle))
    }
    fn signal(&self) {
        // SAFETY: this owned event remains open for the duration of the call.
        unsafe {
            SetEvent(self.0);
        }
    }
}
impl Drop for Event {
    fn drop(&mut self) {
        // SAFETY: only this final owner closes the event, after any waiter exits.
        unsafe {
            CloseHandle(self.0);
        }
    }
}
pub(crate) struct EventBridge {
    stop: Arc<Event>,
    ack: Arc<Event>,
    error: Arc<AtomicU32>,
    join: Option<JoinHandle<()>>,
}
impl EventBridge {
    /// The caller must retain the Wintun session until this bridge is dropped.
    pub(crate) fn new(read: HANDLE, waker: Arc<mio::Waker>) -> io::Result<Self> {
        if read == 0 {
            return Err(io::ErrorKind::InvalidInput.into());
        }
        let stop = Arc::new(Event::new(true)?);
        let ack = Arc::new(Event::new(false)?);
        let error = Arc::new(AtomicU32::new(0));
        let (thread_stop, thread_ack, thread_error) = (stop.clone(), ack.clone(), error.clone());
        let join = thread::Builder::new()
            .name("protun-next-wintun-ready".into())
            .spawn(move || {
                loop {
                    if !wait(thread_stop.0, read, &thread_error) {
                        break;
                    }
                    if waker.wake().is_err() {
                        break;
                    }
                    if !wait(thread_stop.0, thread_ack.0, &thread_error) {
                        break;
                    }
                }
                if thread_error.load(Ordering::Relaxed) != 0 {
                    let _ = waker.wake();
                }
            })?;
        Ok(Self {
            stop,
            ack,
            error,
            join: Some(join),
        })
    }
    pub(crate) fn drained(&self) {
        self.ack.signal();
    }
    pub(crate) fn check(&self) -> io::Result<()> {
        match self.error.load(Ordering::Relaxed) {
            0 => Ok(()),
            code => Err(io::Error::from_raw_os_error(code as i32)),
        }
    }
}
fn wait(stop: HANDLE, ready: HANDLE, error: &AtomicU32) -> bool {
    let handles = [stop, ready];
    // SAFETY: both handles are retained by the bridge/session owners until join.
    let result = unsafe { WaitForMultipleObjects(2, handles.as_ptr(), 0, INFINITE) };
    if result == WAIT_OBJECT_0 + 1 {
        return true;
    }
    if result == WAIT_FAILED {
        error.store(
            io::Error::last_os_error().raw_os_error().unwrap_or(6) as u32,
            Ordering::Relaxed,
        );
    }
    false
}
impl Drop for EventBridge {
    fn drop(&mut self) {
        self.stop.signal();
        if let Some(join) = self.join.take() {
            let _ = join.join();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, Instant};
    #[test]
    fn actual_windows_events_wake_once_per_drain_and_shutdown_joins() {
        let mut poll = mio::Poll::new().unwrap();
        let waker = Arc::new(mio::Waker::new(poll.registry(), mio::Token(3)).unwrap());
        let read = Event::new(false).unwrap();
        let bridge = EventBridge::new(read.0, waker).unwrap();
        let mut events = mio::Events::with_capacity(4);
        for _ in 0..3 {
            read.signal();
            poll.poll(&mut events, Some(Duration::from_secs(2)))
                .unwrap();
            assert!(events.iter().any(|event| event.token() == mio::Token(3)));
            bridge.drained();
        }
        let start = Instant::now();
        drop(bridge);
        assert!(start.elapsed() < Duration::from_secs(2));
    }
}
