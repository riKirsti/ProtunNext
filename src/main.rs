use protun_next::{config::WireguardConfig, control};
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
            println!("{}", include_str!("../docs/parity.json"));
            Ok(())
        }
        Some("verify-parity") => control::release_gate(),
        Some("check-config") if args.len() == 3 => {
            let bytes = std::fs::read(&args[2])?;
            let text = std::str::from_utf8(&bytes)
                .map_err(|_| protun_next::Error::Invalid("UTF-8 configuration"))?;
            let config = WireguardConfig::parse(text)?;
            println!(
                "configuration valid; endpoint={}, mtu={}; no network changes applied",
                config.endpoint, config.mtu
            );
            Ok(())
        }
        Some("run-udp") if args.len() == 5 => {
            #[cfg(target_os = "linux")]
            {
                let config = WireguardConfig::parse(&std::fs::read_to_string(&args[2])?)?;
                let fd: i32 = args[3]
                    .parse()
                    .map_err(|_| protun_next::Error::Invalid("TUN descriptor"))?;
                let counters = protun_next::linux::run_udp(
                    fd,
                    &config,
                    &args[4],
                    &std::sync::atomic::AtomicBool::new(false),
                )?;
                println!(
                    "{}",
                    serde_json::to_string(&counters)
                        .map_err(|e| protun_next::Error::Protocol(e.to_string()))?
                );
                Ok(())
            }
            #[cfg(not(target_os = "linux"))]
            {
                Err(protun_next::Error::Unsupported("Linux UDP adapter"))
            }
        }
        _ => {
            eprintln!(
                "usage: protun-next status | verify-parity | check-config FILE | run-udp FILE TUN_FD UPLINK_DEVICE"
            );
            Err(protun_next::Error::Invalid("command"))
        }
    }
}
