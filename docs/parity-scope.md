# v3.0.5 implementation coverage

The comparison target is **ProtonVPN/protun v3.0.5**, commit
`a498c38ae6ffb0e68e7190b2831ef985d73fa6da`. The source inventory records SHA256
and public symbols for 15 API files, including the optional development helpers.
The behavioral comparison also uses its published pvpnclient4.4.1,
proton-boringtun4.9.0, Muon3.0.0, RCRL2.0.0, LocalAgent0.13.0 and netstack0.8.1
sources. This is a function-coverage review; it is not a claim that every line of
every third-party dependency has been manually audited or is free of defects.

All **39 functional requirements** in `parity.json` now have executable
implementations. Its 40th item, matched target performance evidence, is a
measurement requirement. `verify-code-parity` checks that coverage ledger.
`verify-parity` remains a separate release gate and still fails because live
Proton/Plus and physical platform acceptance have been deferred. Neither a
manifest entry nor a passing offline fixture proves live service interoperability.

## Public functions and their execution paths

| v3.0.5 capability | ProtunNext implementation |
| --- | --- |
| Unix connect, optional/replaced TUN, ownership and interface errors | `api/supervisor.rs`, `connection.rs`, `engine.rs`, Unix/utun adapters |
| Update peers, atomic configuration/settings changes | Supervisor command path; equivalent endpoint updates retain the WG socket/session |
| Up/down/switch, failover, retry, disconnect and wait | Supervisor plus cancellable reactor; captures, private sockets and NAT mappings survive outer-worker replacement |
| WireGuard UDP, framed TCP and Stealth | Shared packet engine, narrow WireGuard session adapter, independent TunSafe framing and bounded rustls I/O |
| Socket protector and physical interface selection | Unix pre-SYN callback/binding; Android protect and Network binding; Windows native interface selection |
| Local HTTP connection-test service | Private netstack listener, available before VPN establishment and while offline or TCP connect is pending |
| Connection statistics and debug counters | Fresh worker samples, connection-relative continuous-clock timestamps and explicit snapshot logging |
| Capture start/stop, path/Unix fd, append/overwrite, limits/events | Capture controller and connection-owned sink, including nonseekable transferred descriptors |
| Logger and callback contracts | Native UniFFI exports; SDK authentication traces filtered from the host logger |
| Private key and persistent caches | OS-seeded identity, legacy PKCS8/Muon/certificate formats and host-owned persistence callbacks |
| Fork selectors/cookies and auth invalidation | Muon exchange, old-session removal, stale-result rejection and consumed-selector suppression |
| Certificate issuance/renewal, expiry, duration, invalidation and rekey | Official RCRL; unusable LocalAgent key/certificate regenerates identity and WG key |
| Prod/Atlas/custom API environment and pins | Validated Muon environment adapter |
| NetShield levels, split TCP, random NAT, port forwarding, soft jail and circumvention | Actual certificate features and/or LocalAgent settings; subscriptions return applied values and refusals |
| Exit labels, server name, jails, plan/2FA/invoice/session limits, restrictions | Peer/account control plus complete typed server-value/error mappings |
| LocalAgent traffic/block statistics and saved-data estimate | Six-field server-response accumulation and the source category estimate |
| Apple | utun prefix adapter, continuous clock, interface binding, shared runtime and Swift export |
| Windows initialize/connect/details/IPv6/DNS/delete routes | Native Wintun and IP Helper implementation; cleanup is limited to process-owned resources |
| Windows alternate address selection | Subnets2..255 propagated consistently to adapter, private stack, DNS, probes and policy |
| Android core/manager/VpnService | Real service, generated native API, per-user cache/cipher, network selection, interface/per-app policy, statistics/capture and lifecycle callbacks |
| Development INI parser | `api/test_utils/test_config_parser.rs`, including both modes, peers, environment, capture, interface and fork defaults |
| Development sync/async login and child-session fork | `api/test_utils/muon_test_auth.rs`, opt-in `test_utils` feature; official SDK SRP/fork flow with a host-network adapter |

Implementation paths in this table are relative to `src/` unless prefixed with
`android/`. See the requirement ledger for individual evidence and source links.
All settings are requests to Proton's service; the code never converts a
requested paid feature into a fabricated server acceptance.

## Network behavior checked against the published SDK

Ordinary customer ports bypass the private TCP/IP stack and NAT map. The small
reserved range uses ownership-aware translation; a new TCP flow requires SYN
without ACK. A tracked reply follows its recorded owner even when its destination
is outside the virtual subnet. Untracked packets use the source routing table's
destination rules. IPv4 fragments retain the first fragment's ownership for ten
seconds; the destination stack performs private reassembly. Flow expiry uses a
suspend-inclusive monotonic clock and the source fifteen-minute idle timeout.

Private API/DNS/LocalAgent traffic has no OS-resolver or host-network fallback.
The optional development login helper explicitly uses the host network, as the
old test helper did; it is not compiled into the default runtime. Merely enabling
the development feature does not log in or run an account test.

Source checks also prevent invented requirements: the pinned library implements
LocalAgent port-forwarding enablement, not NAT-PMP lease management. Its packet
parser/firewall does not implement related-ICMP or IPv6 extension-header NAT.
Those are not missing v3.0.5 functions. The extra NAT-PMP codec in this project
does not demonstrate a working lease service.

## Compatibility and verification boundaries

This fresh project requires host integration and regenerated bindings; Android's
new namespace and generated library are not a binary replacement for an old AAR.
The synchronous development login helper returns typed failures instead of
panicking; interactive 2FA was not supported by the old helper either. Malformed
auth/certificate cache records can recover through a new session/certificate
rather than failing initialization. Configuration parsing rejects malformed or
duplicate fields. Top-domain SNI selection is stable but intentionally does not
copy the old shuffle mapping. Retry scheduling uses this project's bounded
candidate policy rather than reproducing the old executor's timing.

Windows cleanup never enumerates/deletes unrelated adapters or routes. Required
platform APIs are implemented, but real Wintun/utun activation, DNS/route changes,
Android handover/always-on/revoke and packaged C# host acceptance remain device
verification. Android's provided additional ABI build commands remain unverified;
CI builds arm64-v8a. Default/native full builds and `--no-default-features
--features unix` retain the private HTTP/NAT services; a bare no-feature codec
build is not the full native client configuration.

Live Proton login, certificate trust/renewal, Plus feature acceptance and router
throughput remain deferred. There is **no measured performance superiority over
v3.0.5 or kernel WireGuard**. No router software, files, settings or routes were
accessed, removed or changed for this implementation work.

Primary comparisons:

- [Pinned API source](https://github.com/ProtonVPN/protun/tree/a498c38ae6ffb0e68e7190b2831ef985d73fa6da/src/api)
- [Pinned Android source](https://github.com/ProtonVPN/protun/tree/a498c38ae6ffb0e68e7190b2831ef985d73fa6da/vpn-core-android/vpn-core)
- Published SDK files: `pvpnclient/net/{router,firewall,conntrack,ip_fragment_handler,packet}.rs`, `supervisor/{main,localagent}.rs`; Muon `examples/auth-login.rs` and session/fork source.
