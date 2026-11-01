//! Shared bounded packet worker. Platform adapters own devices and readiness.
use crate::{
    Error, Result,
    config::WireguardConfig,
    connection::Observer,
    network::Network,
    packet::{PacketPool, PacketQueue},
    path_probe::{Action as ProbeAction, PathProbe},
    platform::{TunHandle, boottime},
    session::{Output, Session},
    stream::Receive,
    transport::Counters,
    tun_device::TunDevice,
};
use mio::{Events, Interest, Poll, Token};
use std::{
    io::{Read, Write},
    sync::atomic::{AtomicBool, Ordering},
    time::{Duration, SystemTime, UNIX_EPOCH},
};
const TUN: Token = Token(1);
const NET: Token = Token(2);
const CONTROL: Token = Token(3);
const BUDGET: usize = 64;
fn enqueue(
    pool: &mut PacketPool,
    queue: &mut PacketQueue,
    packet: crate::packet::Packet,
) -> Result<()> {
    if let Err(packet) = queue.push(packet) {
        pool.recycle(packet)?;
        return Err(Error::Full);
    }
    Ok(())
}
fn enqueue_network(
    pool: &mut PacketPool,
    queue: &mut PacketQueue,
    packet: crate::packet::Packet,
    observer: Option<&Observer>,
    probe: &mut Option<PathProbe>,
) -> Result<()> {
    if let Some(probe) = probe {
        probe.sent();
    }
    if let Some(observer) = observer {
        observer.capture_wireguard(packet.data(), true);
    }
    enqueue(pool, queue, packet)
}
pub(crate) fn run<N: Network>(
    tun_file: Option<TunHandle>,
    config: &WireguardConfig,
    mut network: N,
    stop: &AtomicBool,
    observer: Option<&crate::connection::Observer>,
) -> Result<Counters> {
    let mut tun = tun_file.map(TunDevice::new);
    let allowed_ips = crate::routing::AllowedIps::new(&config.allowed_ips)?;
    let compatibility_policy = observer.is_some_and(|o| o.compatibility_policy());
    let addresses = observer.map_or_else(Default::default, Observer::addresses);
    let handshake_deadline = boottime()? + Duration::from_secs(10);
    let mut poll = Poll::new()?;
    let waker = std::sync::Arc::new(mio::Waker::new(poll.registry(), CONTROL)?);
    if let Some(observer) = observer {
        observer.set_waker(waker.clone());
    }
    #[cfg(windows)]
    if let Some(tun) = &mut tun {
        tun.attach_waker(waker.clone())?;
    }
    if let Some(tun) = &tun {
        tun.register(poll.registry(), TUN, Interest::READABLE)?;
    }
    network.register(poll.registry(), NET, Interest::READABLE)?;
    let mut events = Events::with_capacity(16);
    let mut tun_writable = false;
    let mut net_writable = false;
    let mut session = Session::new(
        config.private_key.expose(),
        config.peer_key,
        config.keepalive,
        boottime()?,
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| Error::Invalid("wall clock"))?,
    )?;
    let mut pool = PacketPool::new(132, 65535)?;
    let mut to_net = PacketQueue::new(64)?;
    let mut to_tun = PacketQueue::new(64)?;
    let mut counters = Counters::default();
    let mut probe = if compatibility_policy {
        Some(PathProbe::with_addresses(boottime()?, addresses)?)
    } else {
        None
    };
    let mut initiation = pool.take().ok_or(Error::Full)?;
    match session.initiate(initiation.writable())? {
        Output::Network(n) => {
            initiation.set_len(n)?;
            enqueue_network(&mut pool, &mut to_net, initiation, observer, &mut probe)?;
        }
        _ => pool.recycle(initiation)?,
    }
    let mut tun_ready = tun.is_some();
    let mut net_ready = true;
    let mut drain = false;
    let mut network_error = None;
    while !stop.load(Ordering::Relaxed) {
        if let Some(replacement) = observer.and_then(|o| o.take_tun_update()) {
            if let Some(old) = &tun {
                old.deregister(poll.registry())?;
            }
            tun = replacement.map(TunDevice::new);
            #[cfg(windows)]
            if let Some(tun) = &mut tun {
                tun.attach_waker(waker.clone())?;
            }
            tun_ready = tun.is_some();
            tun_writable = false;
            if let Some(new) = &tun {
                new.register(poll.registry(), TUN, Interest::READABLE)?;
            }
            if let Some(observer) = observer {
                observer.interface(tun.is_some(), None);
            }
            if tun.is_none() {
                while let Some(packet) = to_tun.pop() {
                    pool.recycle(packet)?;
                }
            }
        }
        if network.handshaking() && boottime()? >= handshake_deadline {
            return Err(std::io::Error::from(std::io::ErrorKind::TimedOut).into());
        }
        if network_error.is_none() {
            let mut timer_packet = pool.take().ok_or(Error::Full)?;
            let timed = session.advance(boottime()?, timer_packet.writable())?;
            match timed {
                Output::Network(n) => {
                    timer_packet.set_len(n)?;
                    enqueue_network(&mut pool, &mut to_net, timer_packet, observer, &mut probe)?;
                }
                _ => pool.recycle(timer_packet)?,
            }
        }
        // Bounded work per source, with immediate continuation if a budget expires.
        if network_error.is_none() && !network.handshaking() {
            let action = probe.as_mut().map_or(ProbeAction::None, |p| {
                p.action(boottime().expect("continuous clock"))
            });
            match action {
                ProbeAction::Failed => {
                    return Err(std::io::Error::from(std::io::ErrorKind::TimedOut).into());
                }
                ProbeAction::Send
                    if pool.available() >= 2 && to_net.len() + network.pending() < 62 =>
                {
                    if probe.as_ref().is_some_and(PathProbe::retry_handshake)
                        && session.statistics().handshake_age.is_none()
                    {
                        let mut packet = pool.take().ok_or(Error::Full)?;
                        match session.initiate_forced(packet.writable())? {
                            Output::Network(n) => {
                                packet.set_len(n)?;
                                enqueue_network(
                                    &mut pool,
                                    &mut to_net,
                                    packet,
                                    observer,
                                    &mut probe,
                                )?;
                            }
                            _ => pool.recycle(packet)?,
                        }
                    }
                    let ping = probe.as_ref().expect("scheduled probe").packet();
                    if let Some(observer) = observer {
                        observer.capture(&ping);
                    }
                    let mut packet = pool.take().ok_or(Error::Full)?;
                    match session.encrypt(&ping, packet.writable())? {
                        Output::Network(n) => {
                            packet.set_len(n)?;
                            enqueue_network(&mut pool, &mut to_net, packet, observer, &mut probe)?;
                            drain = true;
                        }
                        _ => pool.recycle(packet)?,
                    }
                }
                _ => {}
            }
        }
        for _ in 0..BUDGET {
            if network_error.is_some()
                || !tun_ready
                || pool.available() < 2
                || to_net.len() + network.pending() >= 63
            {
                break;
            }
            let mut input = pool.take().ok_or(Error::Full)?;
            let Some(device) = tun.as_mut() else {
                pool.recycle(input)?;
                break;
            };
            // The embedding host can replace/reconfigure TUN independently of
            // this worker's initial config. Read a whole packet rather than
            // silently truncating it to a stale MTU (a TUN read discards the
            // unread tail). Apply transport wire bounds after the read.
            match device.read(input.writable()) {
                Ok(0) => {
                    pool.recycle(input)?;
                    detach_tun(&mut tun, &poll, observer, Error::Invalid("TUN EOF"))?;
                    tun_ready = false;
                    tun_writable = false;
                    break;
                }
                Ok(n) => {
                    counters.tun_reads += 1;
                    input.set_len(n)?;
                    if n > network.max_ip_packet() {
                        counters.outbound_filtered += 1;
                        pool.recycle(input)?;
                        continue;
                    }
                    if compatibility_policy
                        && crate::virtual_policy::outbound_with(input.data(), addresses)
                            == crate::virtual_policy::Route::Drop
                    {
                        counters.outbound_filtered += 1;
                        pool.recycle(input)?;
                        continue;
                    }
                    if !allowed_ips.outbound(input.data()) {
                        counters.outbound_filtered += 1;
                        pool.recycle(input)?;
                        continue;
                    }
                    #[cfg(feature = "local-agent")]
                    if let Some(observer) = observer
                        && observer.internal_local(input.data(), boottime()?)?
                    {
                        pool.recycle(input)?;
                        continue;
                    }
                    if let Some(observer) = observer {
                        observer.capture(input.data());
                    }
                    #[cfg(feature = "local-agent")]
                    if let Some(observer) = observer
                        && observer.customer_outbound(input.data_mut()).is_err()
                    {
                        counters.outbound_filtered += 1;
                        pool.recycle(input)?;
                        continue;
                    }
                    let mut output = pool.take().ok_or(Error::Full)?;
                    let result = session.encrypt(input.data(), output.writable());
                    pool.recycle(input)?;
                    match result? {
                        Output::Network(n) => {
                            output.set_len(n)?;
                            enqueue_network(&mut pool, &mut to_net, output, observer, &mut probe)?;
                            drain = true;
                        }
                        _ => pool.recycle(output)?,
                    }
                }
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    tun_ready = false;
                    pool.recycle(input)?;
                    break;
                }
                Err(e) if e.kind() == std::io::ErrorKind::Interrupted => {
                    pool.recycle(input)?;
                }
                Err(e) => {
                    pool.recycle(input)?;
                    detach_tun(&mut tun, &poll, observer, e.into())?;
                    tun_ready = false;
                    tun_writable = false;
                    break;
                }
            }
        }
        #[cfg(feature = "local-agent")]
        for _ in 0..BUDGET {
            let Some(observer) = observer else {
                break;
            };
            if network_error.is_some()
                || to_net.len() + network.pending() >= 63
                || pool.available() < 2
            {
                break;
            }
            let mut input = pool.take().ok_or(Error::Full)?;
            let Some(n) = observer.internal_read(input.writable(), boottime()?)? else {
                pool.recycle(input)?;
                break;
            };
            input.set_len(n)?;
            if crate::api::internal::local_service_source_with(input.data(), addresses) {
                if tun.is_some() && to_tun.len() < 63 {
                    enqueue(&mut pool, &mut to_tun, input)?;
                } else {
                    pool.recycle(input)?;
                }
                continue;
            }
            observer.capture(input.data());
            let mut output = pool.take().ok_or(Error::Full)?;
            let result = session.encrypt(input.data(), output.writable());
            pool.recycle(input)?;
            match result? {
                Output::Network(n) => {
                    output.set_len(n)?;
                    enqueue_network(&mut pool, &mut to_net, output, Some(observer), &mut probe)?;
                    drain = true;
                }
                _ => pool.recycle(output)?,
            }
        }
        for _ in 0..BUDGET {
            if network_error.is_some()
                || !net_ready
                || pool.available() < 2
                || to_tun.len() >= 63
                || to_net.len() + network.pending() >= 63
            {
                break;
            }
            let mut input = pool.take().ok_or(Error::Full)?;
            match network.receive(&mut input) {
                Ok(Receive::Packet) => {
                    counters.network_reads += 1;
                    if let Some(observer) = observer {
                        observer.capture_wireguard(input.data(), false);
                    }
                    let mut output = pool.take().ok_or(Error::Full)?;
                    let result = session.decrypt(input.data(), output.writable());
                    pool.recycle(input)?;
                    match result {
                        Ok(Output::Network(n)) => {
                            output.set_len(n)?;
                            enqueue_network(&mut pool, &mut to_net, output, observer, &mut probe)?;
                            drain = true;
                        }
                        Ok(Output::Tunnel(n)) => {
                            output.set_len(n)?;
                            counters.decrypted_packets += 1;
                            if probe.as_mut().is_some_and(PathProbe::received)
                                && let Some(observer) = observer
                            {
                                observer.confirm_path();
                            }
                            if let Some(observer) = observer {
                                observer.capture(output.data());
                            }
                            if compatibility_policy && PathProbe::is_reply(output.data()) {
                                pool.recycle(output)?;
                                continue;
                            }
                            if compatibility_policy
                                && !crate::virtual_policy::inbound_allowed(output.data(), addresses)
                            {
                                counters.inbound_filtered += 1;
                                pool.recycle(output)?;
                                continue;
                            }
                            #[cfg(feature = "local-agent")]
                            let disposition = if let Some(observer) = observer {
                                observer.internal_receive(output.data_mut(), boottime()?)?
                            } else {
                                crate::api::internal::PacketDisposition::Untracked
                            };
                            #[cfg(feature = "local-agent")]
                            if matches!(
                                disposition,
                                crate::api::internal::PacketDisposition::Private
                                    | crate::api::internal::PacketDisposition::Drop
                            ) {
                                if disposition == crate::api::internal::PacketDisposition::Drop {
                                    counters.inbound_filtered += 1;
                                }
                                pool.recycle(output)?;
                                continue;
                            }
                            #[cfg(feature = "local-agent")]
                            let customer_tracked =
                                disposition == crate::api::internal::PacketDisposition::Customer;
                            #[cfg(not(feature = "local-agent"))]
                            let customer_tracked = false;
                            if compatibility_policy
                                && !customer_tracked
                                && crate::virtual_policy::inbound_with(output.data(), addresses)
                                    != crate::virtual_policy::Route::Customer
                            {
                                counters.inbound_filtered += 1;
                                pool.recycle(output)?;
                                continue;
                            }
                            if allowed_ips.inbound(output.data()) && tun.is_some() {
                                enqueue(&mut pool, &mut to_tun, output)?;
                            } else {
                                counters.inbound_filtered += 1;
                                pool.recycle(output)?;
                            }
                        }
                        Ok(Output::None) => pool.recycle(output)?,
                        Err(_) => {
                            counters.invalid_packets += 1;
                            pool.recycle(output)?;
                        }
                    }
                }
                Ok(Receive::WouldBlock) => {
                    net_ready = false;
                    pool.recycle(input)?;
                    break;
                }
                Ok(Receive::Yield) => {
                    pool.recycle(input)?;
                }
                Ok(Receive::Eof) => {
                    pool.recycle(input)?;
                    network_error =
                        Some(std::io::Error::from(std::io::ErrorKind::UnexpectedEof).into());
                    tun_ready = false;
                    net_ready = false;
                    drain = false;
                    break;
                }
                Err(e) => {
                    pool.recycle(input)?;
                    network_error = Some(e);
                    tun_ready = false;
                    net_ready = false;
                    drain = false;
                    break;
                }
            }
        }
        for _ in 0..BUDGET {
            if !drain || to_net.len() + network.pending() >= 63 {
                break;
            }
            let mut output = pool.take().ok_or(Error::Full)?;
            match session.drain(output.writable())? {
                Output::Network(n) => {
                    output.set_len(n)?;
                    enqueue_network(&mut pool, &mut to_net, output, observer, &mut probe)?;
                }
                _ => {
                    pool.recycle(output)?;
                    drain = false;
                    break;
                }
            }
        }
        if network_error.is_none() {
            match network.flush(&mut to_net, &mut pool) {
                Ok(true) => counters.network_backpressure += 1,
                Ok(false) => {}
                Err(e) => {
                    network_error = Some(e);
                    tun_ready = false;
                    net_ready = false;
                    drain = false;
                }
            }
        }
        for _ in 0..BUDGET {
            let Some(packet) = to_tun.front() else {
                break;
            };
            let Some(device) = tun.as_mut() else {
                while let Some(packet) = to_tun.pop() {
                    pool.recycle(packet)?;
                }
                break;
            };
            match device.write(packet.data()) {
                Ok(n) if n == packet.data().len() => {
                    pool.recycle(to_tun.pop().expect("front exists"))?
                }
                Ok(_) => {
                    detach_tun(
                        &mut tun,
                        &poll,
                        observer,
                        Error::Invalid("partial TUN write"),
                    )?;
                    tun_ready = false;
                    tun_writable = false;
                    break;
                }
                Err(e)
                    if e.kind() == std::io::ErrorKind::WouldBlock
                        || crate::platform::send_pressure(&e) =>
                {
                    counters.tun_backpressure += 1;
                    break;
                }
                Err(e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
                Err(e) => {
                    detach_tun(&mut tun, &poll, observer, e.into())?;
                    tun_ready = false;
                    tun_writable = false;
                    break;
                }
            }
        }
        if let Some(observer) = observer {
            observer.observe(&session, &counters, boottime()?);
        }
        // Preserve already authenticated inbound packets before reporting a
        // terminal stream error. Otherwise a reply followed by FIN in the same
        // readiness batch would be discarded before its TUN write.
        if to_tun.is_empty()
            && let Some(error) = network_error.take()
        {
            return Err(error);
        }
        let wants_net = network_error.is_none()
            && (!to_net.is_empty() || network.pending() > 0 || network.wants_write());
        let wants_tun = !to_tun.is_empty();
        if wants_net != net_writable {
            network.reregister(
                poll.registry(),
                NET,
                if wants_net {
                    Interest::READABLE | Interest::WRITABLE
                } else {
                    Interest::READABLE
                },
            )?;
            net_writable = wants_net;
        }
        if let Some(tun) = &tun
            && wants_tun != tun_writable
        {
            tun.reregister(
                poll.registry(),
                TUN,
                if wants_tun {
                    Interest::READABLE | Interest::WRITABLE
                } else {
                    Interest::READABLE
                },
            )?;
            tun_writable = wants_tun;
        }
        let can_continue = network_error.is_none()
            && ((tun_ready && to_net.len() + network.pending() < 63)
                || (net_ready && to_tun.len() < 63 && to_net.len() + network.pending() < 63)
                || (drain && to_net.len() + network.pending() < 63));
        let timeout = if can_continue {
            Duration::ZERO
        } else {
            session
                .next_wakeup()
                .unwrap_or(Duration::from_millis(250))
                .min(Duration::from_millis(250))
        };
        #[cfg(feature = "local-agent")]
        let timeout = observer
            .and_then(|o| boottime().ok().and_then(|now| o.internal_wakeup(now)))
            .map_or(timeout, |internal| internal.min(timeout));
        let timeout = if network.handshaking() {
            timeout
        } else {
            probe
                .as_ref()
                .and_then(|p| p.delay(boottime().expect("continuous clock")))
                .map_or(timeout, |delay| delay.min(timeout))
        };
        #[cfg(windows)]
        let timeout = if wants_tun {
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
            if event.is_readable() {
                if event.token() == TUN {
                    tun_ready = true;
                } else if event.token() == NET {
                    net_ready = true;
                }
                #[cfg(windows)]
                if event.token() == CONTROL {
                    tun_ready = tun.is_some();
                }
            }
        }
    }
    Ok(counters)
}
fn detach_tun(
    tun: &mut Option<TunDevice>,
    poll: &Poll,
    observer: Option<&crate::connection::Observer>,
    error: Error,
) -> Result<()> {
    let Some(observer) = observer else {
        return Err(error);
    };
    if let Some(old) = tun.take() {
        let _ = old.deregister(poll.registry());
    }
    observer.interface(false, Some(error.to_string()));
    Ok(())
}
