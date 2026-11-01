//! IP Helper callbacks only signal a coalescing control channel. No connection
//! joins, callbacks into the host, route enumeration or packet work run in them.
use crate::{
    Result,
    api::{supervisor::Connection, types::ConnectivityEvent},
};
use std::{
    ffi::c_void,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
        mpsc,
    },
    thread::{self, JoinHandle},
};
use windows_sys::Win32::{
    Foundation::HANDLE, NetworkManagement::IpHelper::*, Networking::WinSock::AF_UNSPEC,
};
struct Context {
    index: u32,
    luid: u64,
    sender: mpsc::SyncSender<()>,
}
pub(crate) struct Monitor {
    context: Box<Context>,
    interface: HANDLE,
    route: HANDLE,
    stop: Arc<AtomicBool>,
    worker: Option<JoinHandle<()>>,
}
impl Monitor {
    pub(crate) fn new(connection: Arc<Connection>, index: u32, luid: u64) -> Result<Self> {
        let (sender, receiver) = mpsc::sync_channel(1);
        let mut monitor = Self {
            context: Box::new(Context {
                index,
                luid,
                sender,
            }),
            interface: 0,
            route: 0,
            stop: Arc::new(AtomicBool::new(false)),
            worker: None,
        };
        let context = (&*monitor.context as *const Context).cast();
        // SAFETY: boxed context has a stable address. Drop cancels registrations
        // (waiting for callbacks to finish) before releasing that context.
        unsafe {
            super::status(NotifyIpInterfaceChange(
                AF_UNSPEC,
                Some(interface_change),
                context,
                0,
                &mut monitor.interface,
            ))?;
            super::status(NotifyRouteChange2(
                AF_UNSPEC,
                Some(route_change),
                context,
                0,
                &mut monitor.route,
            ))?;
        }
        let stop = monitor.stop.clone();
        monitor.worker = Some(
            thread::Builder::new()
                .name("protun-next-network".into())
                .spawn(move || {
                    let mut was_available = super::net::available(index);
                    while receiver.recv().is_ok() {
                        if stop.load(Ordering::Relaxed) {
                            break;
                        }
                        // One bounded debounce per control-event burst, never per packet.
                        thread::sleep(std::time::Duration::from_millis(100));
                        while receiver.try_recv().is_ok() {}
                        if stop.load(Ordering::Relaxed) {
                            break;
                        }
                        let available = super::net::available(index);
                        connection.on_connectivity_change(if !available {
                            ConnectivityEvent::Down
                        } else if !was_available {
                            ConnectivityEvent::Up
                        } else {
                            ConnectivityEvent::NetworkSwitch
                        });
                        was_available = available;
                    }
                })?,
        );
        Ok(monitor)
    }
}
unsafe extern "system" fn interface_change(
    context: *const c_void,
    row: *const MIB_IPINTERFACE_ROW,
    _: MIB_NOTIFICATION_TYPE,
) {
    if context.is_null() || row.is_null() {
        return;
    }
    // SAFETY: registration retains Context and Windows retains the callback row
    // for this call; neither reference escapes this function.
    let (context, row) = unsafe { (&*context.cast::<Context>(), &*row) };
    // SAFETY: LUID is an integer union; Value is the complete 64-bit identity.
    if row.InterfaceIndex != context.index && unsafe { row.InterfaceLuid.Value } != context.luid {
        let _ = context.sender.try_send(());
    }
}
unsafe extern "system" fn route_change(
    context: *const c_void,
    row: *const MIB_IPFORWARD_ROW2,
    _: MIB_NOTIFICATION_TYPE,
) {
    if context.is_null() || row.is_null() {
        return;
    }
    // SAFETY: same registration/row lifetime as interface_change; no escape.
    let (context, row) = unsafe { (&*context.cast::<Context>(), &*row) };
    // SAFETY: LUID is an integer union; Value is the complete 64-bit identity.
    if row.InterfaceIndex != context.index && unsafe { row.InterfaceLuid.Value } != context.luid {
        let _ = context.sender.try_send(());
    }
}
impl Drop for Monitor {
    fn drop(&mut self) {
        // SAFETY: handles come only from successful notification registrations;
        // cancellation waits for outstanding callbacks, keeping Context alive.
        unsafe {
            if self.interface != 0 {
                CancelMibChangeNotify2(self.interface);
            }
            if self.route != 0 {
                CancelMibChangeNotify2(self.route);
            }
        }
        self.stop.store(true, Ordering::Relaxed);
        let _ = self.context.sender.try_send(());
        if let Some(worker) = self.worker.take() {
            let _ = worker.join();
        }
    }
}
