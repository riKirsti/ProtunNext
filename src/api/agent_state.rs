//! Server-observed state, never a mirror of requested settings. Protocol enum
//! values come from Proton's published Local Agent 0.13.0 implementation.
use super::types::*;
use proton_vpn_local_agent::{
    Selector, Value,
    managed::ServerError,
    types::{HandledJail, Jail, ToHandleJail},
};
#[derive(Default)]
pub struct AgentState {
    pub connected: bool,
    pub info: AgentConnectionInfo,
    pub jails: Vec<WaitJail>,
    pub exit_label: Option<String>,
    statistics: [Option<Option<u64>>; 6],
}
impl AgentState {
    pub fn watched() -> Vec<Selector> {
        vec![
            Selector::InfoEstablished,
            Selector::InfoGroups,
            Selector::InfoRemoteReal,
            Selector::InfoRemoteRealLocation,
            Selector::InfoRemoteRealLocationCode,
            Selector::InfoExitIpv4,
            Selector::InfoExitIpv6,
            Selector::InfoTunnelMtu,
            Selector::SettingsSoftjail,
            Selector::SettingsNetshieldLevel,
            Selector::SettingsPortForwarding,
            Selector::SettingsRandomNat,
            Selector::SettingsSplitTcp,
            Selector::SettingsCircumventionRouting,
            Selector::SettingsLabel,
            Selector::Jails,
            Selector::Restrictions,
        ]
    }
    pub fn stats_selectors() -> [Selector; 6] {
        [
            Selector::StatsBytesReceived,
            Selector::StatsBytesSent,
            Selector::StatsNetshieldBlockCountMalicious,
            Selector::StatsNetshieldBlockCountAds,
            Selector::StatsNetshieldBlockCountTracking,
            Selector::StatsNetshieldBlockCountAdult,
        ]
    }
    pub fn wait_reason(&self) -> Option<AgentConnectionWaitReason> {
        if self.info.settings.soft_jail == Some(true) {
            Some(AgentConnectionWaitReason::SoftJailed)
        } else if !self.jails.is_empty() {
            Some(AgentConnectionWaitReason::HardJailed {
                jails: self.jails.clone(),
            })
        } else {
            None
        }
    }
    pub fn state(&self, peer: PeerConnectionInfo) -> ConnectionState {
        if self.connected && self.wait_reason().is_none() {
            ConnectionState::Connected {
                peer,
                agent_info: Some(self.info.clone()),
            }
        } else {
            ConnectionState::ConnectingToLocalAgent {
                peer,
                wait_reason: self.wait_reason(),
            }
        }
    }
    pub fn accept(&mut self, value: Value) -> Option<Event> {
        let mut statistic = None;
        match value {
            Value::InfoGroups(v) => self.info.groups = v.map(|v| v.0).unwrap_or_default(),
            Value::InfoRemoteReal(v) => self.info.user_isp_ip = v,
            Value::InfoRemoteRealLocationCode(v) => self.info.user_isp_country_code = v,
            Value::InfoRemoteRealLocation(v) => {
                self.info.user_isp_coordinates = v.map(|v| Coordinates {
                    latitude: v.latitude,
                    longitude: v.longitude,
                })
            }
            Value::InfoExitIpv4(v) => self.info.server_exit_v4 = v.map(|v| IpAddress(v.into())),
            Value::InfoExitIpv6(v) => self.info.server_exit_v6 = v.map(|v| IpAddress(v.into())),
            Value::InfoTunnelMtu(v) => self.info.server_mtu = v,
            Value::SettingsSoftjail(v) => self.info.settings.soft_jail = v,
            Value::SettingsPortForwarding(v) => self.info.settings.port_forwarding = v,
            Value::SettingsRandomNat(v) => self.info.settings.random_nat = v,
            Value::SettingsSplitTcp(v) => self.info.settings.split_tcp = v,
            Value::SettingsCircumventionRouting(v) => self.info.settings.circumvention_routing = v,
            Value::SettingsNetshieldLevel(v) => {
                self.info.settings.netshield_level = v.map(|n| match n {
                    proton_vpn_local_agent::types::NetshieldLevel::None => NetshieldLevel::None,
                    proton_vpn_local_agent::types::NetshieldLevel::MalwareFilter => {
                        NetshieldLevel::MalwareFilter
                    }
                    proton_vpn_local_agent::types::NetshieldLevel::AdsAndMalwareFilter => {
                        NetshieldLevel::AdsAndMalwareFilter
                    }
                    proton_vpn_local_agent::types::NetshieldLevel::AdultAndAdsAndMalwareFilter => {
                        NetshieldLevel::AdultAndAdsAndMalwareFilter
                    }
                })
            }
            Value::SettingsLabel(v) => self.exit_label = v,
            Value::Restrictions(v) => {
                self.info.restrictions = v
                    .map(|v| {
                        v.0.into_iter()
                            .map(|r| match r {
                                proton_vpn_local_agent::types::Restriction::Streaming(reason) => {
                                    Restriction::Streaming { reason }
                                }
                                proton_vpn_local_agent::types::Restriction::Torrent(reason) => {
                                    Restriction::Torrent { reason }
                                }
                                proton_vpn_local_agent::types::Restriction::Other {
                                    name,
                                    reason,
                                } => Restriction::Other { name, reason },
                            })
                            .collect()
                    })
                    .unwrap_or_default()
            }
            Value::Jails(v) => {
                self.jails = v
                    .map(|v| v.0.into_iter().map(jail).collect())
                    .unwrap_or_default()
            }
            Value::StatsBytesReceived(v) => statistic = Some((0, v)),
            Value::StatsBytesSent(v) => statistic = Some((1, v)),
            Value::StatsNetshieldBlockCountMalicious(v) => statistic = Some((2, v)),
            Value::StatsNetshieldBlockCountAds(v) => statistic = Some((3, v)),
            Value::StatsNetshieldBlockCountTracking(v) => statistic = Some((4, v)),
            Value::StatsNetshieldBlockCountAdult(v) => statistic = Some((5, v)),
            Value::InfoEstablished(_) | Value::InfoPlatform(_) | Value::InfoRemote(_) => {}
            // Future protocol fields must not prevent known subscriptions working.
            _ => {}
        }
        if let Some((index, value)) = statistic {
            self.statistics[index] = Some(value);
        }
        if self.statistics.iter().all(Option::is_some) {
            let [rx, tx, malware, ads, tracking, adult] =
                self.statistics.map(|v| v.expect("complete sample"));
            self.statistics = [None; 6];
            // v3.0.5 exposes a category-based estimate, not measured saved bytes.
            let saved = [(malware, 750u64), (ads, 200), (tracking, 50), (adult, 1454)]
                .into_iter()
                .fold(0u64, |sum, (count, kib)| {
                    sum.saturating_add(count.unwrap_or(0).saturating_mul(kib * 1024))
                });
            return Some(Event::LocalAgentStats {
                bytes_received: rx,
                bytes_sent: tx,
                malicious_blocked: malware,
                ads_blocked: ads,
                trackers_blocked: tracking,
                adult_content_blocked: adult,
                data_saved: Some(saved),
            });
        }
        None
    }
    pub fn policy_error(error: &ServerError) -> Option<Event> {
        let setting = match error {
            ServerError::NetshieldLevelPolicyRefused => LocalAgentSettingType::NetshieldLevel,
            ServerError::BouncingPolicyRefused => LocalAgentSettingType::Bouncing,
            ServerError::PortFwPolicyRefused => LocalAgentSettingType::PortForwarding,
            ServerError::RandomNatPolicyRefused => LocalAgentSettingType::RandomNat,
            ServerError::SplitTcpPolicyRefused => LocalAgentSettingType::SplitTcp,
            ServerError::SafeModePolicyRefused => LocalAgentSettingType::SafeMode,
            _ => return None,
        };
        Some(Event::LocalAgentError {
            error: ErrorEvent::LocalAgentSettingPolicyRefused { setting },
        })
    }
}
fn jail(value: Jail) -> WaitJail {
    let code = value.code();
    let (reason, message) = match value {
        Jail::InternallyHandled(
            HandledJail::SystemError(s)
            | HandledJail::ExpiredCertificate(s)
            | HandledJail::RevokedCertificate(s)
            | HandledJail::KeyAlreadyUsed(s)
            | HandledJail::InvalidCertificateSignature(s),
        ) => (WaitJailReason::Internal, s),
        Jail::ToHandle(j) => match j {
            ToHandleJail::RequireRecent2FA(s)
            | ToHandleJail::Expired2FA(s)
            | ToHandleJail::Require2FA(s) => (WaitJailReason::Need2FA, s),
            ToHandleJail::WaitingClientChallengeReply(s) => {
                (WaitJailReason::WaitingClientChallengeReply, s)
            }
            ToHandleJail::PolicyViolation1(s) => (WaitJailReason::LowPlan, s),
            ToHandleJail::PolicyViolation2(s) => (WaitJailReason::PendingInvoice, s),
            ToHandleJail::BadUserBehavior(s) => (WaitJailReason::BadUserBehavior, s),
            ToHandleJail::DisabledUser(s) => (WaitJailReason::DisabledUser, s),
            ToHandleJail::SessionOverLimit(s)
            | ToHandleJail::FreeSessionOverLimit(s)
            | ToHandleJail::BasicSessionOverLimit(s)
            | ToHandleJail::PlusSessionOverLimit(s)
            | ToHandleJail::VisionarySessionOverLimit(s)
            | ToHandleJail::ProSessionOverLimit(s) => (WaitJailReason::SessionOverLimit, s),
            ToHandleJail::GuestSession(s)
            | ToHandleJail::RestrictedServer(s)
            | ToHandleJail::NoCertificateProvided(s)
            | ToHandleJail::SessionInstallationInProgress(s) => (WaitJailReason::Internal, s),
            ToHandleJail::Unknown(_, s) => (WaitJailReason::Other, s),
        },
    };
    WaitJail {
        reason,
        code,
        message,
    }
}
pub fn settings(
    request: &LocalAgentSettings,
    label: Option<String>,
) -> proton_vpn_local_agent::types::Settings {
    proton_vpn_local_agent::types::Settings {
        split_tcp: request.split_tcp,
        netshield_level: request.netshield_level.map(|n| match n {
            NetshieldLevel::None => proton_vpn_local_agent::types::NetshieldLevel::None,
            NetshieldLevel::MalwareFilter => {
                proton_vpn_local_agent::types::NetshieldLevel::MalwareFilter
            }
            NetshieldLevel::AdsAndMalwareFilter => {
                proton_vpn_local_agent::types::NetshieldLevel::AdsAndMalwareFilter
            }
            NetshieldLevel::AdultAndAdsAndMalwareFilter => {
                proton_vpn_local_agent::types::NetshieldLevel::AdultAndAdsAndMalwareFilter
            }
        }),
        softjail: request.soft_jail,
        port_forwarding: request.port_forwarding,
        random_nat: request.random_nat,
        circumvention_routing: request.circumvention_routing,
        label,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn settings_are_server_observed_and_jails_gate_connected_state() {
        let mut state = AgentState::default();
        assert_eq!(state.info.settings.port_forwarding, None);
        state.accept(Value::SettingsPortForwarding(Some(false)));
        assert_eq!(state.info.settings.port_forwarding, Some(false));
        state.accept(Value::SettingsSoftjail(Some(true)));
        assert_eq!(
            state.wait_reason(),
            Some(AgentConnectionWaitReason::SoftJailed)
        );
        state.accept(Value::SettingsSoftjail(Some(false)));
        state.accept(Value::Jails(Some(proton_vpn_local_agent::types::Jails(
            std::collections::HashSet::from([Jail::ToHandle(ToHandleJail::Require2FA(
                "challenge".into(),
            ))]),
        ))));
        assert_eq!(state.jails[0].reason, WaitJailReason::Need2FA);
        assert!(matches!(
            state.wait_reason(),
            Some(AgentConnectionWaitReason::HardJailed { .. })
        ));
        assert!(matches!(
            AgentState::policy_error(&ServerError::PortFwPolicyRefused),
            Some(Event::LocalAgentError {
                error: ErrorEvent::LocalAgentSettingPolicyRefused {
                    setting: LocalAgentSettingType::PortForwarding
                }
            })
        ));
    }
    #[test]
    fn agent_statistics_wait_for_all_six_server_fields() {
        let mut state = AgentState::default();
        for value in [
            Value::StatsBytesReceived(Some(10)),
            Value::StatsBytesSent(Some(20)),
            Value::StatsNetshieldBlockCountMalicious(Some(1)),
            Value::StatsNetshieldBlockCountAds(Some(2)),
            Value::StatsNetshieldBlockCountTracking(Some(3)),
        ] {
            assert!(state.accept(value).is_none());
        }
        assert!(matches!(
            state.accept(Value::StatsNetshieldBlockCountAdult(Some(4))),
            Some(Event::LocalAgentStats {
                bytes_received: Some(10),
                bytes_sent: Some(20),
                data_saved: Some(7_286_784),
                ..
            })
        ));
        assert!(state.accept(Value::StatsBytesSent(None)).is_none());
    }
}
