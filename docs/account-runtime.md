# Account and compatibility implementation checkpoint

Reference: Protun v3.0.5 commit a498c38ae6ffb0e68e7190b2831ef985d73fa6da,
and its published Muon 3.0.0, RCRL 2.0.0, LocalAgent 0.13.0 and
netstack 0.8.1 dependencies. These are source-based compatibility decisions,
not evidence of successful Proton service interoperability.

The Linux supervisor owns peer/port selection, connectivity recovery, optional
and replacement TUN descriptors, callbacks, requested statistics, captures and
the account worker. Metadata-only peer updates retain the socket and WG session.
An active capture sink survives peer/socket replacement; final disconnect closes
it. Captures contain inner IP plus synthetic UDP-wrapped WG frames, following
the upstream diagnostic convention rather than representing real TLS packets.

The account worker executes Muon session restoration and fork/cookie exchange,
RCRL certificate issuance/renewal and invalidation, and managed LocalAgent mTLS,
subscriptions and setting changes. Certificate refresh parameters include
NetShield, split TCP, random NAT and port forwarding. LocalAgent additionally
receives soft jail, circumvention routing and exit labels. Server readback,
restrictions, jails and policy refusals are separate from requested settings.
Key rejection causes persisted identity replacement and a new WG session.
Expired or invalidated certificates disconnect LocalAgent until replaced.
Fork selectors are not retried automatically or written to logs.
Replacing a fork clears Muon's in-memory authentication as well as the host
cache, suspends the old refresh request and invalidates the previous account's
certificate. A generation check rejects stale fork results. The offline API
fixture checks the actual certificate-request JSON, rejected-token cache
clearing, replacement fork GET, cookie delivery and duplicate suppression over
the private network using public test tokens.

Identity cache data uses RFC 8410 Ed25519 PKCS#8 v1 PEM: the legacy reader
assumes that envelope. The WG scalar derives from the same Ed25519 seed.
Certificate cache data preserves the legacy PEM plus Issue-Date/Refresh-Date
headers; session auth is the published Muon Auth JSON. The embedding host still
supplies secure persistence through the original cache callback contract.

Control TCP and DNS use a separate private netstack with privileged source-port
translation. Only registered control flows are consumed by it. No OS DNS or
outside-tunnel API connection fallback is installed. Network control is on an
account thread; the packet thread only exchanges bounded packet work with that
stack. Capture is disabled by an atomic fast check; enabled capture intentionally
performs file I/O and must not be used for performance comparisons.

Offline checks exercise two real private TCP/IP stacks with translated TCP and
UDP request/reply traffic, real WG handshakes without a TUN, cache compatibility,
settings readback, jail gating, statistics aggregation and capture file controls.
No test in this checkpoint uses a Proton account or a router.

The shared Unix worker now also has Apple's utun header adapter, a continuous
Apple clock, platform-specific uplink binding and Android TUN/socket-protection
support. UniFFI 0.31.0 exports the compatibility records, callbacks and Unix
connection object; Kotlin/Swift/Python binding generation is part of verification.
The local HTTP connection-test service is implemented, and the private control
flow map now protects customer traffic with colliding source ports. Ordinary
customer ports bypass that connection-tracking map.

Still required before full source/API parity can be asserted: complete upstream
virtual-router/NAT/firewall behavior (including dynamic Windows subnets), native
Windows transports and management, higher-level Android service integration,
and comprehensive offline account/protocol failure fixtures. NAT-PMP lease management is an extra
router capability, not a v3.0.5 parity requirement: upstream exposes LocalAgent
port-forwarding enablement without implementing that lease manager.

IPv4 private response fragments now retain the first fragment's ownership for
ten seconds (a bounded 4,096-entry table). Real private-stack tests split a
1,024-byte UDP response on its wire path and verify reassembly at the private
socket. Customer tracked flows use upstream's fifteen-minute idle timeout.
Queue pressure on private receive consumes/drops its packet instead of failing
the VPN worker or passing the packet into TUN. Ordinary customer ports still
avoid the map. Fragments arriving without their first fragment follow the
upstream untracked path; no out-of-order reassembly is added to the router.

The pinned pvpnclient `net/packet.rs::next_transport` recognizes only directly
encapsulated TCP/UDP. `firewall.rs::no_track_not_addressable` accepts other
customer/tunnel protocols unchanged. Thus ICMP quoted-flow translation and IPv6
extension-header NAT are not implemented upstream and are not additional
v3.0.5 parity requirements. ProtunNext also passes them unchanged, apart from
its internal connectivity-probe replies and virtual-address isolation.
