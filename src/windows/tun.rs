//! Shared engine's Wintun device implementation. The bridge drops/joins before
//! the session Arc, so its borrowed driver event handle never outlives the DLL.
use crate::{platform::TunHandle, windows::EventBridge};
use std::{
    io::{self, Read, Write},
    sync::Arc,
};
pub(crate) struct TunDevice {
    bridge: Option<EventBridge>,
    handle: TunHandle,
}
impl TunDevice {
    pub(crate) fn new(handle: TunHandle) -> io::Result<Self> {
        Ok(Self {
            bridge: None,
            handle,
        })
    }
    pub(crate) fn write_queued(&mut self, queue: &crate::packet::PacketQueue) -> io::Result<usize> {
        let packet = queue.front().ok_or(io::ErrorKind::InvalidInput)?;
        if self.write(packet.data())? != packet.data().len() {
            return Err(io::ErrorKind::WriteZero.into());
        }
        Ok(1)
    }
    pub(crate) fn offload_fallbacks(&self) -> u64 {
        0
    }
    pub(crate) fn attach_waker(&mut self, waker: Arc<mio::Waker>) -> io::Result<()> {
        self.bridge = Some(EventBridge::new(self.handle.read_event()?, waker)?);
        Ok(())
    }
    pub(crate) fn register(
        &self,
        _: &mio::Registry,
        _: mio::Token,
        _: mio::Interest,
    ) -> io::Result<()> {
        Ok(())
    }
    pub(crate) fn reregister(
        &self,
        _: &mio::Registry,
        _: mio::Token,
        _: mio::Interest,
    ) -> io::Result<()> {
        Ok(())
    }
    pub(crate) fn deregister(&self, _: &mio::Registry) -> io::Result<()> {
        Ok(())
    }
}
impl Read for TunDevice {
    fn read(&mut self, output: &mut [u8]) -> io::Result<usize> {
        let bridge = self.bridge.as_ref();
        if let Some(bridge) = bridge {
            bridge.check()?;
        }
        let result = self.handle.read(output);
        if result
            .as_ref()
            .is_err_and(|e| e.kind() == io::ErrorKind::WouldBlock)
            && let Some(bridge) = bridge
        {
            bridge.drained();
        }
        result
    }
}
impl Write for TunDevice {
    fn write(&mut self, input: &[u8]) -> io::Result<usize> {
        self.handle.write(input)
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
