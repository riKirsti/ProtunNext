# ProtunNext

Independent Rust successor project targeting all functionality exposed by Protun v3.0.5.

**v3.0.5 function implementations are covered; live acceptance remains pending.**
The [source comparison and function mapping](docs/parity-scope.md) covers 39
functional requirements plus a separate performance-evidence item.
`verify-code-parity` checks implementation coverage. `verify-parity` deliberately
fails until deferred service/device acceptance is complete. This is a development
build; offline tests do not establish live Proton Plus support or release readiness.

The [supported platform contract](docs/supported-platforms.md) is Windows 11
25H2/26H2 native x64; Linux 6.12/6.18 LTS x64/ARM64; Android 16/17 API 36/37;
and macOS/iOS/iPadOS 26/27 ARM64. Production backend choices are internal and
fixed. [Measured TLS selection](docs/tls-selection-20261103.md) retains the small
transfer regression as well as the repeatable sustained-download benefit.

## Working implementation in this snapshot

* WireGuard cryptographic-session adapter with real handshakes, packet round trips,
  replay protection, invalid-tag rejection and monotonic timer validation.
* Preallocated owned packet pool and bounded queues. Partial writes retain the
  original allocation with an offset; no tail allocation or copy.
* Independently implemented TunSafe normal/predicted framing with bounded circular
  receive storage, fragmented input and reset/error handling.
* Framed stream I/O with vectored output, bounded backpressure and real TCP
  loopback/WireGuard handshake tests.
* Shared Unix/Windows UDP/framed TCP/TLS worker. Unix attaches only to a caller-owned TUN descriptor and
  binding the outer socket to an explicitly supplied uplink interface before connect.
  TLS selects buffered rustls on Windows/Apple and unbuffered rustls on Linux/Android at compile time; both keep flushing pending ciphertext.
* Managed worker start/disconnect/join, bounded latest-state observation
  and real WireGuard byte/loss/handshake/RTT statistics; no user callbacks or
  control messages run per packet.
* Allocation-free single-peer `AllowedIPs` enforcement for outbound destinations
  and authenticated inbound sources, including IPv4/IPv6 family checks.
* Strict single-peer WireGuard configuration reader; unknown directives fail
  rather than being silently ignored. This low-level reader applies no OS settings.
* PCAP writer, NAT-PMP wire codec and candidate-selection/settings validation.

The compatibility API now has a separate supervisor for peer updates,
network recovery, replaceable/optional TUN, socket-protection callbacks, statistics
and persistent capture controls. The optional `local-agent` feature adds an
account thread using Proton's published Muon, certificate refresher and managed
mTLS Local Agent. API/DNS/control sockets use a private in-tunnel TCP/IP stack;
ordinary customer packets bypass it. The connection retains private sockets and
NAT ownership through outer reconnects, and services local HTTP while offline or
waiting for TCP connect. Virtual-network policy includes selected Windows subnets,
tracked reply ownership and fragmented private responses. The Android service builds an AAR; Windows loopback transports and
native initialization run without creating a VPN adapter.
Account issuance, renewal and paid-server integration have not been tested with
real credentials. Public API declarations alone do not count as implementation.
See [the complete parity ledger](docs/parity.json),
[architecture and dependency decisions](docs/architecture.md), and
[verification evidence](docs/verification.md).

## Build and test

```sh
cargo test --locked --all-targets
cargo build --locked --release --lib --bin protun-next-cli
cargo run --locked -- status
cargo run --locked -- check-config /path/to/private-config.conf
cargo run --locked -- verify-parity # expected to fail during development
cargo run --locked -- verify-code-parity # function implementation coverage
```

The private Proton registry is configured in `.cargo/config.toml`. No Proton
account credentials are required for the host tests. The Linux TUN integration
test is opt-in and requires a disposable network namespace; CI runs it separately.
Default tests never change host routes, interfaces, DNS, sysctls or packages.

The experimental Linux worker is invoked as:

```sh
protun-next-cli run-udp private-config.conf EXISTING_TUN_FD UPLINK_INTERFACE
protun-next-cli run-tcp private-config.conf EXISTING_TUN_FD UPLINK_INTERFACE
protun-next-cli run-tls private-config.conf EXISTING_TUN_FD UPLINK_INTERFACE SERVER_NAME
protun-next-cli run-stealth private-config.conf EXISTING_TUN_FD UPLINK_INTERFACE [random|top|DNS_SNI]
```

The TLS command uses standard certificate verification. The separate Stealth
command uses Proton's published outer policy and Random/Top/explicit SNI;
inner WireGuard authenticates packets before TUN delivery. Temporary Travel
tests cover the saved-config NoLocalAgent path; broader account-enabled live
acceptance remains incomplete. TCP/TLS
MTU is limited to 16,351 bytes by normal TunSafe framing.

The host must create/configure the TUN, addresses, routes and DNS. This command
duplicates its fd and makes the shared file description nonblocking. It does not
implement v3.0.5's complete connection lifecycle or Local Agent mode. Do not run it
as an unattended production client. No automatic software installation/removal,
router management or account login is implemented by this command.

## Performance evidence

The [final performance policy](docs/performance-policy.md) consolidates all eight
stages: retain the serial ring-based worker, compact bounded buffers, direct
framing/TLS receive and explicitly negotiated Linux TUN coalescing. Portable
release builds remain the product default. PGO/CPU-native builds remain optional,
target-qualified experiments. The campaign closes after its final validation PR;
recorded gaps do not schedule further work automatically.

The allocation integration test measures the real established-session
encrypt/frame/decode/decrypt path: after warm-up, 1,024 1280-byte packets must
produce **zero allocator calls** on the observed test thread. The result excludes
initialization, handshakes/rekey, errors, TLS, kernel I/O and account work.

`cargo bench --bench packet_path` times the actual framing implementation on the
machine running it. It is **not a VPN throughput benchmark**. The
[real-download reports](docs/performance-policy.md#review-of-every-stage) retain
native CPU/memory/throughput and temporary Travel observations, including noisy
controls and rejected candidates. These establish specific efficiency findings,
not universal maximum speed or production superiority. Account-enabled acceptance
and several platform/device measurements remain incomplete.

## Scope and paid features

The compatibility target includes peer updates/failover, connectivity events,
TUN replacement, socket protection, stats, capture control, logging, secure cache,
API forks/cookies, certificate issuance/renewal, Local Agent state/settings,
NetShield, port forwarding, split TCP, NAT/circumvention options, policy refusals,
jails/restrictions and platform bindingsâ€”not just the three tunnel transports.

Paid capabilities remain subject to Proton's server policy and the account's
entitlements. A local setting, stub, codec or a successful connection cannot
grant or demonstrate a subscription capability. Secure Core/server choice and
other app/service features must be distinguished from capabilities actually
implemented inside the v3.0.5 library. Deferred acceptance work is recorded explicitly.

GPL-3.0-or-later. No affiliation with or endorsement by Proton AG is implied.
