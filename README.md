# ProtunNext

Independent Rust successor project targeting all functionality exposed by Protun v3.0.5.

**Development snapshot — not a complete replacement, not ready for deployment.**
`cargo run -- verify-parity` deliberately fails until the entire compatibility
matrix has implementation and verification evidence. A working protocol codec
or a green unit test does not mean a complete VPN client or paid-feature support.

## Working implementation in this snapshot

* WireGuard cryptographic-session adapter with real handshakes, packet round trips,
  replay protection, invalid-tag rejection and monotonic timer validation.
* Preallocated owned packet pool and bounded queues. Partial writes retain the
  original allocation with an offset; no tail allocation or copy.
* Independently implemented TunSafe normal/predicted framing with bounded circular
  receive storage, fragmented input and reset/error handling.
* Framed stream I/O with vectored output, bounded backpressure and real TCP
  loopback/WireGuard handshake tests.
* Shared Linux UDP/framed TCP/TLS worker attaching only to an existing caller-owned TUN descriptor and
  binding the outer socket to an explicitly supplied uplink interface before connect.
  TLS uses bounded rustls buffering and keeps flushing pending ciphertext.
* Strict single-peer WireGuard configuration reader; unknown directives fail
  rather than being silently ignored. No routing or DNS settings are applied.
* PCAP writer, NAT-PMP wire codec and candidate-selection/settings validation.

The optional `local-agent` feature provides a typed adapter to Proton's published mTLS control client, including settings/readback and subscriptions. It requires caller-supplied certificates and a protected stream. These components are not a complete Local Agent/account or platform runtime.
See [the complete parity ledger](docs/parity.json),
[architecture and dependency decisions](docs/architecture.md), and
[verification evidence](docs/verification.md).

## Build and test

```sh
cargo test --locked --all-targets
cargo build --locked --release
cargo run --locked -- status
cargo run --locked -- check-config /path/to/private-config.conf
cargo run --locked -- verify-parity # expected to fail during development
```

The private Proton registry is configured in `.cargo/config.toml`. No Proton
account credentials are required for the host tests. The Linux TUN integration
test is opt-in and requires a disposable network namespace; CI runs it separately.
Default tests never change host routes, interfaces, DNS, sysctls or packages.

The experimental Linux worker is invoked as:

```sh
protun-next run-udp private-config.conf EXISTING_TUN_FD UPLINK_INTERFACE
protun-next run-tcp private-config.conf EXISTING_TUN_FD UPLINK_INTERFACE
protun-next run-tls private-config.conf EXISTING_TUN_FD UPLINK_INTERFACE SERVER_NAME
```

The TLS command uses standard certificate verification; Proton Stealth certificate-policy
and SNI compatibility still require implementation and live verification. TCP/TLS
MTU is limited to 16,351 bytes by normal TunSafe framing.

The host must create/configure the TUN, addresses, routes and DNS. This command
duplicates its fd and makes the shared file description nonblocking. It does not
implement v3.0.5's complete connection lifecycle or Local Agent mode. Do not run it
as an unattended production client. No automatic software installation/removal,
router management or account login is implemented by this command.

## Performance evidence

The allocation integration test measures the real established-session
encrypt/frame/decode/decrypt path: after warm-up, 1,024 1280-byte packets must
produce **zero allocator calls** on the observed test thread. The result excludes
initialization, handshakes/rekey, errors, TLS, kernel I/O and account work.

`cargo bench --bench packet_path` times the actual framing implementation on the
machine running it. It is **not a VPN throughput benchmark**. There is currently
no measured router speedup, kernel WireGuard comparison, or claim of production
performance superiority.

## Scope and paid features

The compatibility target includes peer updates/failover, connectivity events,
TUN replacement, socket protection, stats, capture control, logging, secure cache,
API forks/cookies, certificate issuance/renewal, Local Agent state/settings,
NetShield, port forwarding, split TCP, NAT/circumvention options, policy refusals,
jails/restrictions and platform bindings—not just the three tunnel transports.

Paid capabilities remain subject to Proton's server policy and the account's
entitlements. A local setting, stub, codec or a successful connection cannot
grant or demonstrate a subscription capability. Secure Core/server choice and
other app/service features must be distinguished from capabilities actually
implemented inside the v3.0.5 library. Missing work is recorded explicitly.

GPL-3.0-or-later. No affiliation with or endorsement by Proton AG is implied.
