use crate::{Error, Result};
use base64::{Engine, engine::general_purpose::STANDARD};
use serde::{Deserialize, Serialize};
use std::net::{IpAddr, SocketAddr};
use zeroize::{Zeroize, ZeroizeOnDrop};

pub fn load(path: &std::path::Path) -> Result<WireguardConfig> {
    use std::io::Read;
    let mut options = std::fs::OpenOptions::new();
    options.read(true);
    #[cfg(target_os = "linux")]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.custom_flags(libc::O_NOFOLLOW | libc::O_CLOEXEC);
    }
    let file = options.open(path)?;
    let metadata = file.metadata()?;
    if !metadata.is_file() || metadata.len() > 65536 {
        return Err(Error::Invalid(
            "regular bounded configuration file required",
        ));
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if metadata.permissions().mode() & 0o077 != 0 {
            return Err(Error::Invalid(
                "private configuration requires owner-only permissions",
            ));
        }
    }
    let mut bytes = zeroize::Zeroizing::new(Vec::new());
    file.take(65537).read_to_end(&mut bytes)?;
    if bytes.len() > 65536 {
        return Err(Error::Invalid("configuration too large"));
    }
    WireguardConfig::parse(
        std::str::from_utf8(&bytes).map_err(|_| Error::Invalid("UTF-8 configuration"))?,
    )
}

#[derive(Zeroize, ZeroizeOnDrop)]
pub struct PrivateKey([u8; 32]);
impl PrivateKey {
    pub fn decode(text: &str) -> Result<Self> {
        let mut bytes = STANDARD
            .decode(text)
            .map_err(|_| Error::Invalid("key encoding"))?;
        let value: std::result::Result<[u8; 32], _> = bytes.as_slice().try_into();
        bytes.zeroize();
        let value = value.map_err(|_| Error::Invalid("key length"))?;
        if value == [0; 32] {
            return Err(Error::Invalid("zero key"));
        }
        Ok(Self(value))
    }
    pub fn expose(&self) -> [u8; 32] {
        self.0
    }
}
impl std::fmt::Debug for PrivateKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("PrivateKey([REDACTED])")
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Protocol {
    WireguardUdp,
    WireguardTcp,
    Stealth,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Peer {
    pub id: String,
    pub address: IpAddr,
    pub public_key: [u8; 32],
    pub udp_ports: Vec<u16>,
    pub tcp_ports: Vec<u16>,
    pub tls_ports: Vec<u16>,
    pub priority: i32,
    pub exit_label: Option<String>,
    pub server_name: Option<String>,
}
impl Peer {
    pub fn validate(&self) -> Result<()> {
        if self.id.is_empty()
            || self.id.len() > 256
            || self.public_key == [0; 32]
            || self.address.is_unspecified()
            || self.address.is_multicast()
        {
            return Err(Error::Invalid("peer"));
        }
        if self
            .udp_ports
            .iter()
            .chain(&self.tcp_ports)
            .chain(&self.tls_ports)
            .any(|p| *p == 0)
        {
            return Err(Error::Invalid("zero port"));
        }
        if self.udp_ports.len() + self.tcp_ports.len() + self.tls_ports.len() > 128 {
            return Err(Error::Invalid("too many ports"));
        }
        Ok(())
    }
    pub fn endpoints(&self, protocol: Protocol) -> impl Iterator<Item = SocketAddr> + '_ {
        let ports = match protocol {
            Protocol::WireguardUdp => &self.udp_ports,
            Protocol::WireguardTcp => &self.tcp_ports,
            Protocol::Stealth => &self.tls_ports,
        };
        ports.iter().map(|p| SocketAddr::new(self.address, *p))
    }
}
#[derive(Debug)]
pub struct WireguardConfig {
    pub private_key: PrivateKey,
    pub peer_key: [u8; 32],
    pub endpoint: SocketAddr,
    pub addresses: Vec<String>,
    pub allowed_ips: Vec<String>,
    pub dns: Vec<String>,
    pub mtu: u16,
    pub keepalive: Option<u16>,
}
impl WireguardConfig {
    /// Strictly reads one peer; never silently ignores routing/security directives.
    pub fn parse(text: &str) -> Result<Self> {
        if text.len() > 65536 {
            return Err(Error::Invalid("configuration too large"));
        }
        let mut section = "";
        let mut private = None;
        let mut public = None;
        let mut endpoint = None;
        let mut addresses = Vec::new();
        let mut ips = Vec::new();
        let mut dns = Vec::new();
        let mut mtu = 1420;
        let mut keepalive = None;
        let mut peer_seen = false;
        let mut seen = std::collections::HashSet::new();
        for line in text.lines() {
            let line = line.split('#').next().unwrap_or("").trim();
            if line.is_empty() {
                continue;
            }
            if line.starts_with('[') {
                match line {
                    "[Interface]" if section.is_empty() => section = "interface",
                    "[Peer]" if !peer_seen => {
                        peer_seen = true;
                        section = "peer"
                    }
                    _ => return Err(Error::Invalid("duplicate or unsupported section")),
                }
                continue;
            }
            let (key, value) = line
                .split_once('=')
                .ok_or(Error::Invalid("configuration assignment"))?;
            let key = key.trim();
            let value = value.trim();
            if !seen.insert((section, key)) {
                return Err(Error::Invalid("duplicate setting"));
            }
            match (section, key) {
                ("interface", "PrivateKey") => private = Some(PrivateKey::decode(value)?),
                ("interface", "Address") => {
                    addresses = value.split(',').map(|s| s.trim().to_owned()).collect()
                }
                ("interface", "DNS") => {
                    dns = value.split(',').map(|s| s.trim().to_owned()).collect()
                }
                ("interface", "MTU") => {
                    mtu = value.parse().map_err(|_| Error::Invalid("MTU"))?;
                    if !(1280..=65503).contains(&mtu) {
                        return Err(Error::Invalid("MTU bounds"));
                    }
                }
                ("peer", "PublicKey") => {
                    let p = PrivateKey::decode(value)?;
                    public = Some(p.expose());
                }
                ("peer", "Endpoint") => {
                    endpoint =
                        Some(value.parse().map_err(|_| {
                            Error::Invalid("endpoint must be a resolved IP and port")
                        })?)
                }
                ("peer", "AllowedIPs") => {
                    ips = value.split(',').map(|s| s.trim().to_owned()).collect()
                }
                ("peer", "PersistentKeepalive") => {
                    let n: u16 = value.parse().map_err(|_| Error::Invalid("keepalive"))?;
                    keepalive = if n == 0 { None } else { Some(n) };
                }
                _ => return Err(Error::Invalid("unsupported directive; no settings applied")),
            }
        }
        if addresses.is_empty() || ips.is_empty() {
            return Err(Error::Invalid("missing addresses or allowed IPs"));
        }
        for cidr in addresses.iter().chain(&ips) {
            let (ip, bits) = cidr.split_once('/').ok_or(Error::Invalid("CIDR"))?;
            let ip: IpAddr = ip.parse().map_err(|_| Error::Invalid("IP"))?;
            let bits: u8 = bits.parse().map_err(|_| Error::Invalid("CIDR prefix"))?;
            if bits > if ip.is_ipv4() { 32 } else { 128 } {
                return Err(Error::Invalid("CIDR prefix"));
            }
        }
        let endpoint: SocketAddr = endpoint.ok_or(Error::Invalid("missing endpoint"))?;
        if endpoint.port() == 0 {
            return Err(Error::Invalid("zero endpoint port"));
        }
        Ok(Self {
            private_key: private.ok_or(Error::Invalid("missing private key"))?,
            peer_key: public.ok_or(Error::Invalid("missing peer key"))?,
            endpoint,
            addresses,
            allowed_ips: ips,
            dns,
            mtu,
            keepalive,
        })
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn secrets_redacted_and_unknown_hooks_fail_closed() {
        let key = STANDARD.encode([3; 32]);
        let cfg = format!(
            "[Interface]\nPrivateKey={key}\nAddress=10.2.0.2/32\n[Peer]\nPublicKey={key}\nEndpoint=127.0.0.1:51820\nAllowedIPs=0.0.0.0/0,::/0\n"
        );
        let parsed = WireguardConfig::parse(&cfg).unwrap();
        assert!(!format!("{parsed:?}").contains(&key));
        assert!(WireguardConfig::parse(&(cfg + "PostUp=unsafe command\n")).is_err());
    }
}
