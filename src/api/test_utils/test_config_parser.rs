//! Bounded reader for the pinned sample's mode/peer/fork INI format.
use crate::api::types::*;
use base64::{Engine, engine::general_purpose::STANDARD};
use std::{collections::BTreeMap, io::Read};
use zeroize::{Zeroize, Zeroizing};

pub struct ParsedConfig {
    pub initial_connection_config: InitialConnectionConfig,
    #[cfg(target_os = "linux")]
    pub tun_interface_name: Option<String>,
}
pub struct ParsedForkConfig {
    pub username: String,
    pub password: String,
    pub app_version: String,
    pub child: String,
    pub muon_env: MuonEnv,
}
impl Drop for ParsedForkConfig {
    fn drop(&mut self) {
        self.password.zeroize();
        self.username.zeroize();
    }
}
pub fn parse_ini_config(path: String) -> Result<(ParsedConfig, Option<ParsedForkConfig>), String> {
    let read = || -> std::io::Result<Zeroizing<String>> {
        let mut options = std::fs::OpenOptions::new();
        options.read(true);
        #[cfg(target_os = "linux")]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.custom_flags(libc::O_NOFOLLOW | libc::O_CLOEXEC);
        }
        let file = options.open(path)?;
        if !file.metadata()?.is_file() {
            return Err(std::io::ErrorKind::InvalidInput.into());
        }
        let mut text = Zeroizing::new(String::new());
        file.take(65537).read_to_string(&mut text)?;
        Ok(text)
    };
    parse(&read().map_err(|_| "cannot read development configuration".to_owned())?)
}
type Fields = BTreeMap<String, Zeroizing<String>>;
fn get<'a>(fields: &'a Fields, key: &str) -> Option<&'a str> {
    fields.get(key).map(|value| value.as_str())
}
fn required<'a>(fields: &'a Fields, key: &str) -> Result<&'a str, String> {
    get(fields, key).ok_or_else(|| format!("missing field {key}"))
}
fn ports(fields: &Fields, key: &str) -> Result<Vec<u16>, String> {
    get(fields, key).map_or(Ok(vec![]), |value| {
        value
            .split(',')
            .map(|value| {
                value
                    .trim()
                    .parse::<u16>()
                    .ok()
                    .filter(|port| *port != 0)
                    .ok_or_else(|| "invalid port".to_owned())
            })
            .collect()
    })
}
fn key(value: &str) -> Result<[u8; 32], String> {
    let bytes = Zeroizing::new(
        STANDARD
            .decode(value.trim())
            .map_err(|_| "invalid key encoding")?,
    );
    bytes
        .as_slice()
        .try_into()
        .map_err(|_| "key must contain 32 bytes".to_owned())
}
fn parse(text: &str) -> Result<(ParsedConfig, Option<ParsedForkConfig>), String> {
    if text.len() > 65536 {
        return Err("configuration exceeds 64 KiB".into());
    }
    let mut sections = BTreeMap::<String, Fields>::new();
    let mut section = String::new();
    for line in text.lines().map(str::trim) {
        if line.is_empty() || line.starts_with(['#', ';']) {
            continue;
        }
        if let Some(name) = line
            .strip_prefix('[')
            .and_then(|line| line.strip_suffix(']'))
        {
            section = name.trim().to_ascii_lowercase();
            continue;
        }
        let (name, value) = line.split_once('=').ok_or("expected INI key=value")?;
        if section.is_empty() {
            return Err("missing INI section".into());
        }
        let fields = sections.entry(section.clone()).or_default();
        if fields
            .insert(
                name.trim().to_ascii_lowercase(),
                Zeroizing::new(value.trim().to_owned()),
            )
            .is_some()
        {
            return Err("duplicate INI field".into());
        }
    }
    let mode = sections.get("mode").ok_or("missing mode section")?;
    let mut peers = Vec::new();
    for (name, fields) in &sections {
        let Some(id) = name.strip_prefix("peer.") else {
            continue;
        };
        peers.push(PeerInfo {
            peer_id: id.to_owned(),
            server_ip: IpAddress(
                required(fields, "server_ip")?
                    .parse()
                    .map_err(|_| "invalid server IP")?,
            ),
            server_public_key: WgPeerPublicKey(key(required(fields, "server_public_key")?)?),
            udp_ports: ports(fields, "udp_ports")?,
            tcp_ports: ports(fields, "tcp_ports")?,
            tls_ports: ports(fields, "tls_ports")?,
            priority: get(fields, "priority")
                .unwrap_or("0")
                .parse()
                .map_err(|_| "invalid priority")?,
            exit_label: get(fields, "exit_label").map(str::to_owned),
            server_name: get(fields, "server_name").map(str::to_owned),
        });
    }
    let connection_mode = match required(mode, "with_local_agent")? {
        "true" => {
            let environment = get(mode, "env").unwrap_or("prod").to_ascii_lowercase();
            ConnectionMode::LocalAgent {
                user_agent: required(mode, "user_agent")?.into(),
                app_version: required(mode, "app_version")?.into(),
                settings: Default::default(),
                certificate_duration: None,
                muon_env: match environment.as_str() {
                    "prod" => MuonEnv::Prod,
                    "atlas" => MuonEnv::Atlas { scientist: None },
                    name => MuonEnv::Atlas {
                        scientist: Some(name.into()),
                    },
                },
            }
        }
        "false" => ConnectionMode::NoLocalAgent {
            wg_private_key: get(mode, "client_private_key")
                .map(key)
                .transpose()?
                .map(WgClientPrivateKey),
        },
        _ => return Err("with_local_agent must be true or false".into()),
    };
    let fork = match (&connection_mode, sections.get("fork")) {
        (
            ConnectionMode::LocalAgent {
                app_version,
                muon_env,
                ..
            },
            Some(fields),
        ) => Some(ParsedForkConfig {
            username: required(fields, "username")?.into(),
            password: required(fields, "password")?.into(),
            app_version: get(fields, "app_version").unwrap_or(app_version).into(),
            child: get(fields, "child")
                .or_else(|| get(mode, "child"))
                .unwrap_or(app_version)
                .into(),
            muon_env: muon_env.clone(),
        }),
        _ => None,
    };
    Ok((
        ParsedConfig {
            initial_connection_config: InitialConnectionConfig {
                peers,
                connection_mode,
                network_available: true,
                sni_strategy: SniStrategy::Random,
                pcap_file: get(mode, "pcap_file").map(|path| PcapFileInfo {
                    file: PcapFile::Path {
                        path: path.into(),
                        mode: FileWriteMode::Overwrite,
                    },
                    max_bytes: None,
                }),
            },
            #[cfg(target_os = "linux")]
            tun_interface_name: get(mode, "tun_interface_name").map(str::to_owned),
        },
        fork,
    ))
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sample_modes_keys_peers_fork_defaults_and_malformed_inputs() {
        let text = format!(
            "[mode]\nwith_local_agent=true\nuser_agent=test\napp_version=windows-vpn@4.1.0\nchild=android-vpn@5.0.0\nenv=atlas\n[peer.first]\nserver_ip=192.0.2.1\nserver_public_key={}\nudp_ports=443, 51820\n[fork]\nusername=public-test-user\npassword=public-test-password\n",
            STANDARD.encode([9; 32])
        );
        let (config, fork) = parse(&text).unwrap();
        assert_eq!(
            config.initial_connection_config.peers[0].udp_ports,
            [443, 51820]
        );
        let fork = fork.unwrap();
        assert_eq!(fork.child, "android-vpn@5.0.0");
        assert!(matches!(fork.muon_env, MuonEnv::Atlas { scientist: None }));
        assert!(parse("[mode]\nwith_local_agent=invalid").is_err());
        assert!(parse("[mode]\nwith_local_agent=false\nclient_private_key=bad").is_err());
        assert!(parse(&"x".repeat(65537)).is_err());
        assert!(matches!(
            parse("[mode]\nwith_local_agent=false")
                .unwrap()
                .0
                .initial_connection_config
                .connection_mode,
            ConnectionMode::NoLocalAgent {
                wg_private_key: None
            }
        ));
    }
}
