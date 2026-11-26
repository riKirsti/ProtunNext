//! Readiness, budgets, timers and delivery order for the serial packet worker.
use super::{
    BUDGET, CONTROL, NET, TUN, detach_tun,
    packets::{decrypt_owned, encrypt_owned, enqueue, enqueue_network},
    policy::{Decision, PacketPolicy},
};
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
use mio::{Events, Interest, Poll};
use std::{
    io::Read,
    sync::atomic::{AtomicBool, Ordering},
    time::{Duration, SystemTime, UNIX_EPOCH},
};
pub(crate) fn run<N: Network>(
    tun_file: Option<TunHandle>,
    config: &WireguardConfig,
    mut network: N,
    stop: &AtomicBool,
    observer: Option<&Observer>,
) -> Result<Counters> {
    let mut tun = tun_file.map(TunDevice::new).transpose()?;
    let compatibility_policy = observer.is_some_and(Observer::compatibility_policy);
    let addresses = observer.map_or_else(Default::default, Observer::addresses);
    let policy = PacketPolicy::new(config, compatibility_policy, addresses)?;
    #[cfg(feature = "control-plane")]
    let control = observer.map(Observer::control_network);
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
    let mut pool = PacketPool::compact(132, (usize::from(config.mtu) + 32).max(2048), 65535)?;

    let mut to_net = PacketQueue::new(64)?;
    let mut to_tun = PacketQueue::new(64)?;
    let mut counters = Counters {
        pool_reserved_bytes: pool.reserved_bytes() as u64,
        ..Default::default()
    };
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
        let batch_now = boottime()?;
        if let Some(replacement) = observer.and_then(|o| o.take_tun_update()) {
            if let Some(old) = &tun {
                old.deregister(poll.registry())?;
            }
            tun = replacement.map(TunDevice::new).transpose()?;
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
            let timed = session.advance(batch_now, timer_packet.writable())?;
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
            let mut input = pool.take_sized(65535).ok_or(Error::Full)?;
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
                    match policy.outbound(
                        &input,
                        #[cfg(feature = "control-plane")]
                        control,
                        batch_now,
                    )? {
                        Decision::Forward => {}
                        Decision::Consumed => {
                            pool.recycle(input)?;
                            continue;
                        }
                        Decision::Filtered => {
                            counters.outbound_filtered += 1;
                            pool.recycle(input)?;
                            continue;
                        }
                    }
                    if let Some(observer) = observer {
                        observer.capture(input.data());
                    }
                    #[cfg(feature = "control-plane")]
                    if compatibility_policy
                        && let Some(control) = control
                        && control.customer_outbound(input.data_mut()).is_err()
                    {
                        counters.outbound_filtered += 1;
                        pool.recycle(input)?;
                        continue;
                    }
                    let (result, mut output) = encrypt_owned(&mut session, input)?;
                    counters.in_place_encryptions += 1;
                    match result {
                        Output::Network(n) => {
                            output.set_data_len(n)?;
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
        #[cfg(feature = "control-plane")]
        for _ in 0..BUDGET {
            let Some(control) = control else {
                break;
            };
            if network_error.is_some()
                || to_net.len() + network.pending() >= 63
                || pool.available() < 2
            {
                break;
            }
            let mut input = pool.take_sized(65535).ok_or(Error::Full)?;
            let Some(n) = control.read_packet(input.writable(), batch_now)? else {
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
            if let Some(observer) = observer {
                observer.capture(input.data());
            }
            let (result, mut output) = encrypt_owned(&mut session, input)?;
            counters.in_place_encryptions += 1;
            match result {
                Output::Network(n) => {
                    output.set_data_len(n)?;
                    enqueue_network(&mut pool, &mut to_net, output, observer, &mut probe)?;
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
            let mut input = pool.take_sized(65535).ok_or(Error::Full)?;
            match network.receive(&mut input, &mut pool) {
                Ok(Receive::Packet) => {
                    counters.network_reads += 1;
                    if let Some(observer) = observer {
                        observer.capture_wireguard(input.data(), false);
                    }
                    let in_place = input.data().starts_with(&[4, 0, 0, 0]);
                    let (result, mut output) =
                        decrypt_owned(&mut session, &mut pool, input, in_place)?;
                    if in_place {
                        counters.in_place_decryptions += 1;
                    }
                    match result {
                        Ok(Output::Network(n)) => {
                            output.set_data_len(n)?;
                            enqueue_network(&mut pool, &mut to_net, output, observer, &mut probe)?;
                            drain = true;
                        }
                        Ok(Output::Tunnel(n)) => {
                            output.set_data_len(n)?;
                            counters.decrypted_packets += 1;
                            if probe.as_mut().is_some_and(PathProbe::received)
                                && let Some(observer) = observer
                            {
                                observer.confirm_path();
                            }
                            if let Some(observer) = observer {
                                observer.capture(output.data());
                            }
                            match policy.inbound(
                                &mut output,
                                #[cfg(feature = "control-plane")]
                                control,
                                batch_now,
                            )? {
                                Decision::Forward if tun.is_some() => {
                                    enqueue(&mut pool, &mut to_tun, output)?
                                }
                                Decision::Consumed => pool.recycle(output)?,
                                Decision::Forward | Decision::Filtered => {
                                    counters.inbound_filtered += 1;
                                    pool.recycle(output)?;
                                }
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
                    counters.outer_eof += 1;
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
                    counters.outer_io_errors += 1;
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
            let mut output = pool.take_sized(65535).ok_or(Error::Full)?;
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
        counters.to_network_high_water = counters
            .to_network_high_water
            .max((to_net.len() + network.pending()) as u64);
        counters.to_tun_high_water = counters.to_tun_high_water.max(to_tun.len() as u64);
        #[cfg(feature = "debug-counters")]
        {
            counters.to_network_bytes_high_water = counters
                .to_network_bytes_high_water
                .max(to_net.queued_bytes());
            counters.to_tun_bytes_high_water =
                counters.to_tun_bytes_high_water.max(to_tun.queued_bytes());
        }
        if pool.available() < 2 {
            counters.pool_pressure += 1;
        }
        if network_error.is_none() {
            match network.flush(&mut to_net, &mut pool) {
                Ok(true) => counters.network_backpressure += 1,
                Ok(false) => {}
                Err(e) => {
                    counters.outer_io_errors += 1;
                    network_error = Some(e);
                    tun_ready = false;
                    net_ready = false;
                    drain = false;
                }
            }
        }
        for _ in 0..BUDGET {
            if to_tun.is_empty() {
                break;
            }
            let Some(device) = tun.as_mut() else {
                while let Some(packet) = to_tun.pop() {
                    pool.recycle(packet)?;
                }
                break;
            };
            let fallbacks = device.offload_fallbacks();
            let delivery = device.write_queued(&to_tun);
            let fallbacks = device.offload_fallbacks() - fallbacks;
            counters.tun_write_calls += 1 + fallbacks;
            counters.tun_offload_fallbacks += fallbacks;
            match delivery {
                Ok(count) => {
                    counters.tun_written_packets += count as u64;
                    counters.tun_max_write_batch = counters.tun_max_write_batch.max(count as u64);
                    if count > 1 {
                        counters.tun_coalesced_packets += count as u64;
                    }
                    for _ in 0..count {
                        pool.recycle(to_tun.pop().expect("accepted prefix"))?;
                    }
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
        counters.io = network.diagnostics();
        #[cfg(feature = "debug-counters")]
        {
            counters.successful_crypto_copy_bytes = session.successful_crypto_copy_bytes();
            counters.pool_recycle_wipe_bytes = pool.recycled_wipe_bytes();
        }
        counters.to_network_high_water = counters
            .to_network_high_water
            .max((to_net.len() + network.pending()) as u64);
        counters.to_tun_high_water = counters.to_tun_high_water.max(to_tun.len() as u64);
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
            && network.retry_delay().is_none()
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
            && network.retry_delay().is_none()
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
        #[cfg(feature = "control-plane")]
        let timeout = control
            .and_then(|control| boottime().ok().and_then(|now| control.next_wakeup(now)))
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
        let timeout = network
            .retry_delay()
            .map_or(timeout, |delay| delay.min(timeout));
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
