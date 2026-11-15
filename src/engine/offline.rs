//! Local-service worker while no authenticated uplink exists.
use super::{BUDGET, CONTROL, TUN, detach_tun, packets::enqueue};
use crate::{
    Error, Result,
    connection::Observer,
    packet::{PacketPool, PacketQueue},
    platform::{TunHandle, boottime},
    transport::Counters,
    tun_device::TunDevice,
};
use mio::{Events, Interest, Poll};
use std::{
    io::{Read, Write},
    sync::atomic::{AtomicBool, Ordering},
    time::Duration,
};

/// Keeps host-to-local-service traffic alive without creating a socket or a
/// cryptographic session. External packets are discarded until an uplink exists.
pub(crate) fn run_offline(
    tun_file: Option<TunHandle>,
    stop: &AtomicBool,
    observer: &Observer,
) -> Result<Counters> {
    let mut tun = tun_file.map(TunDevice::new).transpose()?;
    let mut poll = Poll::new()?;
    let waker = std::sync::Arc::new(mio::Waker::new(poll.registry(), CONTROL)?);
    observer.set_waker(waker.clone());
    #[cfg(windows)]
    if let Some(tun) = &mut tun {
        tun.attach_waker(waker.clone())?;
    }
    if let Some(tun) = &tun {
        tun.register(poll.registry(), TUN, Interest::READABLE)?;
    }
    let mut events = Events::with_capacity(8);
    let mut pool = PacketPool::new(66, 65535)?;
    let mut replies = PacketQueue::new(64)?;
    let mut ready = tun.is_some();
    let mut writable = false;
    let mut counters = Counters::default();
    loop {
        if stop.load(Ordering::Relaxed) {
            return Ok(counters);
        }
        if let Some(replacement) = observer.take_tun_update() {
            if let Some(old) = &tun {
                old.deregister(poll.registry())?;
            }
            // Pending replies belong to the previous interface.
            while let Some(packet) = replies.pop() {
                pool.recycle(packet)?;
            }
            tun = replacement.map(TunDevice::new).transpose()?;
            #[cfg(windows)]
            if let Some(tun) = &mut tun {
                tun.attach_waker(waker.clone())?;
            }
            if let Some(tun) = &tun {
                tun.register(poll.registry(), TUN, Interest::READABLE)?;
            }
            ready = tun.is_some();
            writable = false;
            observer.interface(tun.is_some(), None);
        }
        for _ in 0..BUDGET {
            if !ready {
                break;
            }
            let mut packet = pool.take().ok_or(Error::Full)?;
            let Some(device) = tun.as_mut() else {
                pool.recycle(packet)?;
                break;
            };
            match device.read(packet.writable()) {
                Ok(0) => {
                    pool.recycle(packet)?;
                    detach_tun(&mut tun, &poll, Some(observer), Error::Invalid("TUN EOF"))?;
                    ready = false;
                    writable = false;
                    break;
                }
                Ok(n) => {
                    counters.tun_reads += 1;
                    packet.set_len(n)?;
                    #[cfg(feature = "control-plane")]
                    let local =
                        crate::virtual_policy::outbound_with(packet.data(), observer.addresses())
                            == crate::virtual_policy::Route::LocalService
                            && observer.internal_local(packet.data(), boottime()?)?;
                    #[cfg(not(feature = "control-plane"))]
                    let local = false;
                    if !local {
                        counters.outbound_filtered += 1;
                    }
                    pool.recycle(packet)?;
                }
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    pool.recycle(packet)?;
                    ready = false;
                    break;
                }
                Err(e) if e.kind() == std::io::ErrorKind::Interrupted => {
                    pool.recycle(packet)?;
                }
                Err(e) => {
                    pool.recycle(packet)?;
                    detach_tun(&mut tun, &poll, Some(observer), e.into())?;
                    ready = false;
                    writable = false;
                    break;
                }
            }
        }
        #[cfg(feature = "control-plane")]
        for _ in 0..BUDGET {
            if replies.len() >= 64 {
                break;
            }
            let mut packet = pool.take().ok_or(Error::Full)?;
            let Some(n) = observer.internal_read(packet.writable(), boottime()?)? else {
                pool.recycle(packet)?;
                break;
            };
            packet.set_len(n)?;
            if tun.is_some()
                && crate::api::internal::local_service_source_with(
                    packet.data(),
                    observer.addresses(),
                )
            {
                enqueue(&mut pool, &mut replies, packet)?;
            } else {
                pool.recycle(packet)?;
            }
        }
        for _ in 0..BUDGET {
            let Some(packet) = replies.front() else {
                break;
            };
            let Some(device) = tun.as_mut() else {
                while let Some(packet) = replies.pop() {
                    pool.recycle(packet)?;
                }
                break;
            };
            match device.write(packet.data()) {
                Ok(n) if n == packet.data().len() => {
                    pool.recycle(replies.pop().expect("front exists"))?;
                }
                Err(e)
                    if e.kind() == std::io::ErrorKind::WouldBlock
                        || crate::platform::send_pressure(&e) =>
                {
                    counters.tun_backpressure += 1;
                    break;
                }
                Err(e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
                result => {
                    let error = result
                        .err()
                        .map_or(Error::Invalid("partial TUN write"), Error::from);
                    detach_tun(&mut tun, &poll, Some(observer), error)?;
                    ready = false;
                    writable = false;
                    break;
                }
            }
        }
        observer.observe_offline(&counters, boottime()?);
        let wants_write = !replies.is_empty();
        if let Some(tun) = &tun
            && writable != wants_write
        {
            tun.reregister(
                poll.registry(),
                TUN,
                if wants_write {
                    Interest::READABLE | Interest::WRITABLE
                } else {
                    Interest::READABLE
                },
            )?;
            writable = wants_write;
        }
        let timeout = if ready {
            Duration::ZERO
        } else {
            Duration::from_millis(250)
        };
        #[cfg(feature = "control-plane")]
        let timeout = observer
            .internal_wakeup(boottime()?)
            .map_or(timeout, |delay| delay.min(timeout));
        #[cfg(windows)]
        let timeout = if wants_write {
            timeout.min(Duration::from_millis(1))
        } else {
            timeout
        };
        match poll.poll(&mut events, Some(timeout)) {
            Ok(()) => {}
            Err(e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
            Err(e) => return Err(e.into()),
        }
        counters.polls += 1;
        for event in &events {
            if event.is_readable() && event.token() == TUN {
                ready = true;
            }
            #[cfg(windows)]
            if event.token() == CONTROL {
                ready = tun.is_some();
            }
        }
    }
}
