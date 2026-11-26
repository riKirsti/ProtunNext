//! Routing and service decisions; the reactor retains packet/queue ownership.
#[cfg(feature = "control-plane")]
use crate::connection::ControlNetwork;
use crate::{
    Result, addressing::Addresses, config::WireguardConfig, packet::Packet, path_probe::PathProbe,
    routing::AllowedIps,
};
use std::time::Duration;

pub(super) enum Decision {
    Forward,
    Consumed,
    Filtered,
}

pub(super) struct PacketPolicy {
    allowed: AllowedIps,
    compatibility: bool,
    addresses: Addresses,
}
impl PacketPolicy {
    pub fn new(
        config: &WireguardConfig,
        compatibility: bool,
        addresses: Addresses,
    ) -> Result<Self> {
        Ok(Self {
            allowed: AllowedIps::new(&config.allowed_ips)?,
            compatibility,
            addresses,
        })
    }

    pub fn outbound(
        &self,
        packet: &Packet,
        #[cfg(feature = "control-plane")] control: Option<&ControlNetwork>,
        now: Duration,
    ) -> Result<Decision> {
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
        if let Some(control) = control
            && control.receive_local(packet.data(), now, self.addresses)?
        {
            return Ok(Decision::Consumed);
        }
        Ok(Decision::Forward)
    }

    // Called only after successful authentication/replay commit and path confirmation.
    pub fn inbound(
        &self,
        packet: &mut Packet,
        #[cfg(feature = "control-plane")] control: Option<&ControlNetwork>,
        now: Duration,
    ) -> Result<Decision> {
        #[cfg(not(feature = "control-plane"))]
        let _ = now;
        if self.compatibility && PathProbe::is_reply(packet.data()) {
            return Ok(Decision::Consumed);
        }
        if self.compatibility
            && !crate::virtual_policy::inbound_allowed(packet.data(), self.addresses)
        {
            return Ok(Decision::Filtered);
        }
        #[cfg(feature = "control-plane")]
        let disposition = if let Some(control) = control {
            control.receive_routed(packet.data_mut(), now)?
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
