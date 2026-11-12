# Stage 5: alternative acceleration

Baseline: merged PR #9, `4c7cc53a0932fa31579729b603d80a9b9e9ecabf`.
The transition review is recorded in [the agreed roadmap](../performance-stages.md#stage-5-review-november-11).
This stage is in progress; no backend or production speedup is selected yet.

## Shortlist and first comparison

| Candidate | Evidence and fit | Next decision |
|---|---|---|
| Packet AEAD implementation | Stage 4 native UDP/TCP profiles attribute roughly 44–49% of client samples to AEAD opening. A serial implementation comparison covers all outer transports without adding a thread or altering wire/session semantics. | Screen existing ring/RustCrypto and current AWS-LC; integrate competitive candidates separately for real downloads. |
| TLS crypto provider | Stealth adds AES/TLS work; packet-crypto results cannot establish TLS-provider benefits. | Assess a separate provider-only comparison after packet AEAD screening; no combined-effect assumption. |
| Native WireGuard | Official embedding APIs vary by OS. Linux's interface configures kernel WireGuard; adopting it does not itself preserve the current arbitrary UDP/TCP/Stealth transport hooks or application-owned packet API. | Defer integration until packet/transport/control-plane coverage and privilege/lifecycle design are explicit. No performance rejection is inferred. |
| Advanced I/O | Current paths already batch UDP and coalesce negotiated TUN output. Another I/O backend must preserve packet ownership/erasure and cross-platform API behavior. | Revisit profiles for remaining syscall/queue costs after crypto comparison; no gain inferred from fewer calls. |
| CPU/NIC coordination and AF_XDP | AF_XDP requires an XDP program, queue-bound sockets and UMEM/ring ownership; zero-copy support depends on the driver. Current GH fixtures are virtualized and Travel's normal Direct path must be preserved. | Require a device-specific coverage/privilege/restoration design before implementation. Do not assume a generic or userspace-stack win. |

Sources: [AWS-LC AEAD compatibility API](https://docs.rs/aws-lc-rs/1.19.0/aws_lc_rs/aead/index.html),
[current AWS-LC release](https://github.com/aws/aws-lc-rs/releases/tag/v1.19.0),
[existing RustCrypto API/security notes](https://docs.rs/chacha20poly1305/0.10.1/chacha20poly1305/),
[official WireGuard embedding](https://www.wireguard.com/embedding/), and
[kernel AF_XDP documentation](https://docs.kernel.org/networking/af_xdp.html).
The shortlist's integration priorities are inferences from these API requirements
and ProtunNext's measured profiles, not benchmark conclusions about untested backends.

## Primitive preselection

The [isolated screening crate](../../experiments/crypto/README.md) changes no
production dependency, packet code or feature. Tests require the RFC known answer,
all directional cross-backend pairs and authentication failures. A pinned
single-CPU native x64/ARM64 screen records six balanced orders, representative
payload sizes, source/dependency/binary identities and raw observations. Wall
ns/op includes restoring synthetic input, but excludes setup/allocation. It
cannot establish full-session CPU cost, RSS, download speed or reliability.

Only whole-path comparisons can justify a production selection. Competitive
candidates require exact downloads, baseline-fixed auxiliaries, A/A/order and
headroom checks, independent platform evidence, absolute client CPU %/RSS/Mbps,
security/erasure regressions and temporary latest-selected-code Travel tests.
Build-only platforms remain explicitly performance-unmeasured. No primitive
result, helper-limited unpaced result or build pass alone establishes a speedup.
