//! Optional control-plane integration with Proton's published Local Agent client.
//! No control operation runs inside the packet worker. An mTLS client certificate,
//! entitled account and an explicitly routed/protected stream must be supplied.
use crate::{
    Error, Result,
    control::{AgentSettings, Netshield},
};
use futures::{AsyncRead, AsyncWrite};
pub use proton_vpn_local_agent::{
    CertificateDer, Config, LocalAgentError, PrivateKeyDer, Selector, Setting, Value,
};
use proton_vpn_local_agent::{LocalAgent, types::NetshieldLevel};

pub struct Agent<S> {
    client: LocalAgent<S>,
}
pub struct Applied {
    pub requested: Setting,
    pub observed: Value,
    pub matches_request: bool,
}
pub fn settings(request: &AgentSettings) -> Result<Vec<Setting>> {
    // The server owns policy compatibility and entitlement decisions.
    let mut result = Vec::with_capacity(6);
    if let Some(v) = request.split_tcp {
        result.push(Setting::SettingsSplitTcp(v));
    }
    if let Some(v) = request.netshield {
        result.push(Setting::SettingsNetshieldLevel(match v {
            Netshield::Off => NetshieldLevel::None,
            Netshield::Malware => NetshieldLevel::MalwareFilter,
            Netshield::AdsAndMalware => NetshieldLevel::AdsAndMalwareFilter,
            Netshield::AdultAdsAndMalware => NetshieldLevel::AdultAndAdsAndMalwareFilter,
        }));
    }
    if let Some(v) = request.soft_jail {
        result.push(Setting::SettingsSoftjail(v));
    }
    if let Some(v) = request.port_forwarding {
        result.push(Setting::SettingsPortForwarding(v));
    }
    if let Some(v) = request.random_nat {
        result.push(Setting::SettingsRandomNat(v));
    }
    if let Some(v) = request.circumvention_routing {
        result.push(Setting::SettingsCircumventionRouting(v));
    }
    Ok(result)
}
fn matches(request: &Setting, observed: &Value) -> bool {
    match (request, observed) {
        (Setting::SettingsSplitTcp(a), Value::SettingsSplitTcp(Some(b))) => a == b,
        (Setting::SettingsNetshieldLevel(a), Value::SettingsNetshieldLevel(Some(b))) => a == b,
        (Setting::SettingsSoftjail(a), Value::SettingsSoftjail(Some(b))) => a == b,
        (Setting::SettingsPortForwarding(a), Value::SettingsPortForwarding(Some(b))) => a == b,
        (Setting::SettingsRandomNat(a), Value::SettingsRandomNat(Some(b))) => a == b,
        (
            Setting::SettingsCircumventionRouting(a),
            Value::SettingsCircumventionRouting(Some(b)),
        ) => a == b,
        _ => false,
    }
}
impl<S: AsyncRead + AsyncWrite + Unpin> Agent<S> {
    pub async fn connect(stream: S, config: Config) -> std::result::Result<Self, LocalAgentError> {
        Ok(Self {
            client: LocalAgent::new(stream, config).await?,
        })
    }
    pub async fn get(&mut self, selector: Selector) -> std::result::Result<Value, LocalAgentError> {
        self.client.get(selector).await
    }
    /// A server acknowledgement is followed by readback; absent/different values
    /// are never reported as applied. Policy/connection errors propagate to caller.
    pub async fn apply(&mut self, request: &AgentSettings) -> Result<Vec<Applied>> {
        let mut outcomes = Vec::new();
        for setting in settings(request)? {
            self.client
                .set(setting.clone())
                .await
                .map_err(|e| Error::Protocol(format!("Local Agent setting: {e}")))?;
            let observed = self
                .client
                .get(setting.selector())
                .await
                .map_err(|e| Error::Protocol(format!("Local Agent readback: {e}")))?;
            let matches_request = matches(&setting, &observed);
            outcomes.push(Applied {
                requested: setting,
                observed,
                matches_request,
            });
        }
        Ok(outcomes)
    }
    pub async fn subscribe(
        &mut self,
        selectors: &[Selector],
    ) -> std::result::Result<(), LocalAgentError> {
        self.client.subscribe(selectors).await
    }
    pub async fn next_event(&mut self) -> std::result::Result<Value, LocalAgentError> {
        self.client.recv_next_from_subscriptions().await
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn port_forwarding_and_nat_policy_are_left_to_the_server() {
        let request = AgentSettings {
            port_forwarding: Some(true),
            random_nat: Some(false),
            ..Default::default()
        };
        let result = settings(&request).expect("typed requests must reach server policy");
        assert!(result.contains(&Setting::SettingsPortForwarding(true)));
        assert!(result.contains(&Setting::SettingsRandomNat(false)));
    }
    #[test]
    fn typed_requests_match_published_selector_values() {
        let request = AgentSettings {
            split_tcp: Some(true),
            netshield: Some(Netshield::AdultAdsAndMalware),
            soft_jail: Some(false),
            port_forwarding: Some(true),
            random_nat: Some(true),
            circumvention_routing: Some(true),
        };
        let result = settings(&request).unwrap();
        assert_eq!(result.len(), 6);
        assert!(result.contains(&Setting::SettingsNetshieldLevel(
            NetshieldLevel::AdultAndAdsAndMalwareFilter
        )));
        assert!(matches(
            &Setting::SettingsPortForwarding(true),
            &Value::SettingsPortForwarding(Some(true))
        ));
        assert!(!matches(
            &Setting::SettingsPortForwarding(true),
            &Value::SettingsPortForwarding(None)
        ));
        assert!(!matches(
            &Setting::SettingsPortForwarding(true),
            &Value::SettingsPortForwarding(Some(false))
        ));
    }
}
