# Architecture and evidence

## Decisions

Rust is selected for ownership/lifetime enforcement across packet buffers and
for existing cryptographic/TLS implementations. It is not intrinsically faster
than well-written C/C++; a target benchmark must evaluate this implementation.

The packet path uses an owned allocation transferred directly between I/O,
crypto and output queues. It has no `pvpnclient`, PFFF, per-packet action channel
or app/control callback dispatch. `PacketPool` reserves memory once;
queues reject excess ownership. Established sessions retain the original crypto
scratch copy required by the reused cipher interface. This is not zero-copy I/O.

Single-peer `AllowedIPs` is enforced as destination routing on outbound IP
and source ACL on authenticated inbound IP, following WireGuard's documented
cryptokey-routing rules. CIDRs are parsed once; default v4/v6 routes have constant
checks and no per-packet allocation. Malformed IP lengths are rejected. This is
not a complete replacement for upstream virtual-router/NAT/firewall behavior.
Validated inbound packets are written to TUN before a terminal TCP/TLS read error
is reported, so an immediately following FIN cannot discard the final reply.

Control-plane protocol compatibility and platform policy remain separate from
the worker. Account APIs, certificates, applied server settings and policy
failures must be verified before Local Agent/Plus parity is marked complete.
No app setting is reported as applied merely because it was requested.

Linux UDP kernel/offload batching is a future adapter work item. The present
worker uses ordinary nonblocking calls with bounded work budgets and explicit
rescheduling; it does not pretend to implement GSO, GRO, recvmmsg or sendmmsg.
Kernel WireGuard management remains a separate future backend, not a userspace
UDP worker relabeled as kernel WireGuard.

## Reused dependency boundaries

* **proton-boringtun 4.9.0:** cryptographic WireGuard handshake/session/replay and
  key lifetime machinery only. Reimplementing these algorithms from scratch
  would introduce security/interoperability risk without a measured benefit.
  Earlier tests found its ring-backed session much faster than the whole VPN
  path; no guarantee is made that this dependency has no performance issues.
  Its published API requires `proton-os-interface` clock/lock types. The adapter
  uses OS entropy, synchronized wall time and explicit monotonic time. Linux
  obtains CLOCK_BOOTTIME to account for suspend.
* **rustls 0.23.46 + ring:** TLS cryptography/protocol only. No replacement AEAD
  or certificate verifier is fabricated. The ordinary helper verifies standard
  certificates. The separate Stealth entry point follows Proton's published
  outer policy (no CA/name identity check, valid handshake signature required),
  and always routes payload through inner peer-key-authenticated WireGuard.
  It is never selected as fallback for ordinary TLS or Local Agent.
* **Muon 3.0.0 / RCRL 2.0.0 / LocalAgent 0.13.0:** published session/fork,
  certificate refresher and mTLS/RESP3 implementations. Their account runtime
  executes on a separate thread. The embedding host supplies secure persistence;
  server subscriptions supply applied settings and refusals. Source review and
  offline account fixtures do not prove live service interoperability.
* **proton-vpn-netstack 0.8.1:** private API/DNS/LocalAgent sockets and the local
  HTTP endpoint. Ordinary customer ports bypass it. The small reserved port
  range uses a bounded flow map to prevent control/customer reply collisions.
  IPv4 private-response fragments retain the first fragment's route for ten
  seconds in a bounded table. Dynamic Windows subnet policy remains unfinished.
  Non-TCP/UDP traffic passes unchanged, as in the pinned upstream firewall;
  quoted-ICMP and IPv6 extension-header NAT are not upstream features.
* **Ed25519 dalek / PKCS#8:** identity creation, PEM compatibility and the
  published Ed25519-to-X25519 conversion; cryptography is not handwritten.
* **Mio/libc/socket2:** Unix readiness and thin OS calls. No packet representation is
  copied into an additional framework action queue.
* **serde/JSON/base64:** configuration/status/control boundaries, not required
  within the healthy established packet loop.
* **zeroize/getrandom:** secret clearing and operating-system entropy.

Dependency usefulness does not prove absence of bugs. Versions are locked and
changes need source review, compatibility tests and measured target evidence.

## Worker lifetime and resource limits

The Unix managed connection owns a duplicated, validated TUN fd and one packet
thread. Start generates a WireGuard initiation immediately. Compatibility mode
also requires authenticated decrypted data before TunnelEstablished. Its ICMP
probe and 500-unanswered-packet recovery trigger use the upstream retry policy;
probe replies are not delivered to customer applications. The low-level worker
keeps its explicitly documented handshake-only state contract. Disconnect joins the worker; repeated joins return its
final counters. Latest-state snapshots coalesce with a sequence number, avoiding
an unbounded callback backlog. Statistics requests have generation numbers so
an earlier state update cannot satisfy a later sampling request. After session
establishment, observation takes no per-packet lock or action-channel message;
periodic sampling is at most once/second unless explicitly requested.

The general-IP worker currently reserves 132 packet buffers of 65,535 bytes plus
headroom (about 8.3 MiB before TLS/framing/control state). Each output direction
is bounded to 64 packets, including stream-owned packet backlog. This is an
explicit memory tradeoff for accepting full-sized inbound IP, not a claim of
optimal router memory/cache behavior. MTU-sized pools, batched syscalls and
kernel offload require target measurements before changing these bounds.
TLS write-direction buffers are bounded to 65,536 bytes; vectored framing output
is forwarded to rustls's vectored writer, and ciphertext remains scheduled for
write even when all application packets have been accepted.

## Stealth policy and SNI

Proton's checksum-verified `pvpnclient 4.4.1/src/pvpnclient/wireguard/tls.rs`
sets `TlsClientConfig { insecure: true, .. }`. The corresponding
`proton-pfff-module 0.8.10/src/tls/config.rs` accepts the certificate identity but
uses rustls provider verification for TLS 1.2/1.3 handshake signatures. The new
`stealth` module implements this policy separately from standard TLS. Its TLS
constructor is crate-private; public execution always includes inner WireGuard.
There is no direct TLS-plaintext-to-TUN path. Without the configured WG private
peer identity an outer endpoint cannot authenticate an IP packet to TUN.

The Random SNI strategy uses the published alphabet, label-length range and TLD
family with OS entropy. Top uses v3.0.5's domain data, retained with Proton's
copyright/license attribution in `stealth_top_sni.rs`. The mapping is a stable
non-security hash of the address, not an exact reproduction of Rust's unspecified
upstream `DefaultHasher` mapping. This changes the selected cover name for some
peers, not the framing or authenticated VPN identity. No domain is contacted to
select SNI. The outer peer identity and traffic metadata are not authenticated
by this policy; inner WireGuard provides VPN data authentication/confidentiality.
Live Proton server acceptance and censorship behavior remain unverified.

Apple utun adds/removes its four-byte network-order address-family header with
vectored I/O, without shifting the payload. Linux/Android use TUN_NO_PI. Apple
uses CLOCK_MONOTONIC_RAW, which advances during sleep, and platform-specific
uplink binding. Windows now has a native Wintun C boundary, IP Helper management,
physical socket protection, event notifications and shared Mio/IOCP transport.
See [the Windows checkpoint](windows-runtime.md) for evidence and remaining gaps;
no real Windows adapter has been activated by the checks.

UniFFI 0.31.0 exports the Unix compatibility object, records and callbacks.
CI calls it through generated Python bindings, typechecks generated Swift, and
builds an Android API25 library plus Kotlin bindings and the independent
VpnService/manager AAR. The Windows object API is also exported and its generated
Python initialization is exercised without constructing an adapter. Full
source/API parity remains incomplete independently of deferred live acceptance.

## Official reference material checked

* Protun v3.0.5 public APIs and platform modules:
  https://github.com/ProtonVPN/protun/tree/a498c38ae6ffb0e68e7190b2831ef985d73fa6da
* Proton's published cryptographic crate and registry checksums:
  https://rust-registry.proton.me/index/pr/ot/proton-boringtun
* WireGuard protocol and cryptography:
  https://www.wireguard.com/protocol/
* TunSafe's published TCP wire format, normal/data frames and prediction rules:
  https://tunsafe.com/user-guide/tcp
* Linux TUN and multiqueue example:
  https://docs.kernel.org/networking/tuntap.html
* Mio readiness/draining requirements:
  https://docs.rs/mio/1.2.3/mio/struct.Poll.html#draining-readiness
* Rust vectored-write and short-write contract:
  https://doc.rust-lang.org/std/io/trait.Write.html
* NAT-PMP protocol:
  https://www.rfc-editor.org/rfc/rfc6886.html
* Proton's paid port-forwarding instructions and NAT incompatibility:
  https://protonvpn.com/support/port-forwarding-manual-setup
* Proton's feature descriptions (service/app scope, not evidence of library API):
  https://protonvpn.com/features

The parity ledger uses source references for each upstream requirement. Protocol
tests do not prove that an entitled live Proton server accepts every feature.
