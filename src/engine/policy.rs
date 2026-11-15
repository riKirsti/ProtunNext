//! Routing and service decisions; the reactor retains packet/queue ownership.
use crate::{
    Result, addressing::Addresses, config::WireguardConfig, connection::Observer, packet::Packet,
    path_probe::PathProbe, routing::AllowedIps,
};
use std::time::Duration;

pub(super) enum Decision {
    Forward,
    Consumed,
    Filtered,
}

pub(super) struct PacketPolicy<'a> {
    allowed: AllowedIps,
    pub compatibility: bool,
    pub addresses: Addresses,
    observer: Option<&'a Observer>,
}
impl<'a> PacketPolicy<'a> {
    pub fn new(config: &WireguardConfig, observer: Option<&'a Observer>) -> Result<Self> {
        let allowed = AllowedIps::new(&config.allowed_ips)?;
        let compatibility = observer.is_some_and(|o| o.compatibility_policy());
        let addresses = observer.map_or_else(Default::default, Observer::addresses);
        Ok(Self {
            allowed,
            compatibility,
            addresses,
            observer,
        })
    }

    pub fn outbound(&self, packet: &mut Packet, now: Duration) -> Result<Decision> {
        #[cfg(not(feature = "control-plane"))]
        let _ = now;
        if self.compatibility
            && crate::virtual_policy::outbound_with(packet.data(), self.addresses)
                == crate::virtual_policy::Route::Drop
        {
            return Ok(Decision::Filtered);
        }
        if !self.allowed.outbound(packet.data()) {
            return Ok(Decision::Filtered);
        }
        #[cfg(feature = "control-plane")]
        if let Some(observer) = self.observer
            && observer.internal_local(packet.data(), now)?
        {
            return Ok(Decision::Consumed);
        }
        if let Some(observer) = self.observer {
            observer.capture(packet.data());
        }
        #[cfg(feature = "control-plane")]
        if let Some(observer) = self.observer
            && observer.customer_outbound(packet.data_mut()).is_err()
        {
            return Ok(Decision::Filtered);
        }
        Ok(Decision::Forward)
    }

    // Called only after successful authentication/replay commit and path confirmation.
    pub fn inbound(&self, packet: &mut Packet, now: Duration) -> Result<Decision> {
        #[cfg(not(feature = "control-plane"))]
        let _ = now;
        if let Some(observer) = self.observer {
            observer.capture(packet.data());
        }
        if self.compatibility && PathProbe::is_reply(packet.data()) {
            return Ok(Decision::Consumed);
        }
        if self.compatibility
            && !crate::virtual_policy::inbound_allowed(packet.data(), self.addresses)
        {
            return Ok(Decision::Filtered);
        }
        #[cfg(feature = "control-plane")]
        let disposition = if let Some(observer) = self.observer {
            observer.internal_receive(packet.data_mut(), now)?
        } else {
            crate::api::internal::PacketDisposition::Untracked
        };
        #[cfg(feature = "control-plane")]
        match disposition {
            crate::api::internal::PacketDisposition::Private => return Ok(Decision::Consumed),
            crate::api::internal::PacketDisposition::Drop => return Ok(Decision::Filtered),
            _ => {}
        }
        #[cfg(feature = "control-plane")]
        let customer_tracked = disposition == crate::api::internal::PacketDisposition::Customer;
        #[cfg(not(feature = "control-plane"))]
        let customer_tracked = false;
        if self.compatibility
            && !customer_tracked
            && crate::virtual_policy::inbound_with(packet.data(), self.addresses)
                != crate::virtual_policy::Route::Customer
        {
            return Ok(Decision::Filtered);
        }
        Ok(if self.allowed.inbound(packet.data()) {
            Decision::Forward
        } else {
            Decision::Filtered
        })
    }
}
