use protun_next::{config, control};
fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}
fn run() -> protun_next::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("status") => {
            println!("{}", include_str!("requirements.json"));
            Ok(())
        }
        Some("verify-parity") => control::release_gate(),
        Some("verify-code-parity") => control::code_coverage_gate(),
        Some("check-config") if args.len() == 3 => {
            let config = config::load(std::path::Path::new(&args[2]))?;
            println!(
                "configuration valid; endpoint={}, mtu={}; no network changes applied",
                config.endpoint, config.mtu
            );
            Ok(())
        }
        Some(command @ ("run-udp" | "run-tcp" | "run-tls" | "run-stealth"))
            if (command == "run-stealth" && (args.len() == 5 || args.len() == 6))
                || args.len() == if command == "run-tls" { 6 } else { 5 } =>
        {
            #[cfg(target_os = "linux")]
            {
                let config = config::load(std::path::Path::new(&args[2]))?;
                let fd: i32 = args[3]
                    .parse()
                    .map_err(|_| protun_next::Error::Invalid("TUN descriptor"))?;
                let stop = std::sync::atomic::AtomicBool::new(false);
                use protun_next::transport::Transport;
                let transport = match command {
                    "run-udp" => Transport::Udp,
                    "run-tcp" => Transport::Tcp,
                    "run-stealth" => {
                        let sni = match args.get(5).map(String::as_str) {
                            None | Some("random") => protun_next::stealth::SniStrategy::Random,
                            Some("top") => protun_next::stealth::SniStrategy::Top,
                            Some(name) => {
                                protun_next::stealth::SniStrategy::Explicit(name.to_owned())
                            }
                        };
                        Transport::Stealth(sni)
                    }
                    _ => Transport::Tls(Box::new(protun_next::tls::client(&args[5])?)),
                };
                let counters =
                    protun_next::linux::run_transport_fd(fd, &config, &args[4], transport, &stop)?;
                println!(
                    "{}",
                    serde_json::to_string(&counters)
                        .map_err(|e| protun_next::Error::Protocol(e.to_string()))?
                );
                Ok(())
            }
            #[cfg(not(target_os = "linux"))]
            {
                let _ = command;
                Err(protun_next::Error::Unsupported("Linux TUN adapter"))
            }
        }
        _ => {
            eprintln!(
                "usage: protun-next-cli status | verify-parity | verify-code-parity | check-config FILE | run-udp FILE TUN_FD UPLINK_DEVICE | run-tcp FILE TUN_FD UPLINK_DEVICE | run-tls FILE TUN_FD UPLINK_DEVICE SERVER_NAME | run-stealth FILE TUN_FD UPLINK_DEVICE [random|top|DNS_SNI]"
            );
            Err(protun_next::Error::Invalid("command"))
        }
    }
}
