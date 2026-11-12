# Stage 5: alternative acceleration

Baseline: merged PR #9, `4c7cc53a0932fa31579729b603d80a9b9e9ecabf`.
The transition review is recorded in [the agreed roadmap](../performance-stages.md#stage-5-review-november-11).
**Selection:** retain the existing ring production implementation. No runtime,
production dependency or default-provider change is selected. The complete
224-download normal-release evaluation, 24 separate diagnostic downloads,
primitive screens and physical Travel tests are retained below. The TLS-suite
prototype remains an explicit archive-only experiment. No production speedup or
release is claimed. Other deferred architectures have not been performance-rejected.

## Shortlist and first comparison

| Candidate | Evidence and fit | Next decision |
|---|---|---|
| Packet AEAD implementation | Stage 4 native UDP/TCP profiles attribute roughly 44–49% of client samples to AEAD opening. A serial implementation comparison covers all outer transports without adding a thread or altering wire/session semantics. | Existing/current RustCrypto are slower at packet sizes; AWS-LC packet opening is nearly tied. No replacement selected from preselection. |
| TLS crypto provider | Stealth adds AES/TLS work; packet-crypto results cannot establish TLS-provider benefits. | Tested independently; full Stealth matrix does not reproduce a CPU benefit, RSS rises, and implicit-provider compatibility remains unresolved. Keep archive-only. |
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

The initial [three-backend screen](https://github.com/riKirsti/ProtunNext/actions/runs/43157240325)
at source `b3f305de0fdadb985756a035c38cfbdfa7f6703b` retains native
[x64](https://github.com/riKirsti/ProtunNext/actions/runs/43157240325/artifacts/13916560969)
and [ARM64](https://github.com/riKirsti/ProtunNext/actions/runs/43157240325/artifacts/13916695898)
observations. All known-answer and directional interoperability tests pass.
At 1280/1420 bytes, AWS-LC opening differs from ring by -0.22/+0.33% on x64
and -0.30/-0.34% on ARM64. This is effectively a tie in this synthetic screen,
not evidence of a full-path win. Existing RustCrypto 0.10.1 opening is slower
by 241.99/275.72% on x64 and 138.63/129.88% on ARM64. The empty-message result
does not justify a bulk-download backend selection. Each architecture retains
216 observations, six repeats per backend/size/operation, without pooling hosts.

A separate follow-up includes current [RustCrypto 0.11.0](https://docs.rs/chacha20poly1305/0.11.0/chacha20poly1305/)
with zeroization enabled. All four backends re-run in the same balanced cohort;
the earlier observations remain tied to their original source. No forced CPU
backend, AVX-512 configuration flag or architecture-specific compiler tuning is
introduced here. A provider-only TLS experiment remains separate from packet AEAD.

## TLS-suite experiment and selection

The [hash-checked prototype](../../experiments/crypto/TLS_RECORDS.md) substitutes
AWS-LC implementations for the same ordered rustls cipher-suite identities.
Suite objects also carry hash/HKDF/PRF implementations; this is a suite-provider
comparison, not an isolated AES instruction benchmark. Key exchange, signing,
signature verification and randomness remain ring. Roots, server-name checks,
TLS versions, ALPN and Stealth's coupled WireGuard identity policy are unchanged.
WireGuard packet AEAD remains ring. Both variants use vnet and one client CPU;
the peer, load and HTTP(S) server are fixed baseline binaries on separate CPUs.

The first native x64 Stealth IPv4/one-flow cohort observes CPU/byte -3.88%,
but the separate full native matrix does not reproduce that reduction. All 24
full-matrix cells pass A/A and timing screens; all 192 downloads verify. The
four Stealth cells range from -0.01% to +1.43% CPU/byte on x64, and +0.53% to
+2.45% on ARM64. Their RSS rises 27.23–35.96% on x64 and 14.45–27.79% on ARM64.
UDP/TCP controls, which have no intended packet-path change, show small mixed
CPU effects. Keep all focused results rather than pooling allocations or
selecting only the first favorable sample.

Focused LTS Stealth IPv4/one-flow CPU/byte falls 1.74% on 6.12 and 3.16% on
6.18, with RSS rising 40.79% and 38.59%; observed throughput changes -0.12%
and -0.35%. Both cells are auxiliary-limited and cannot establish client
maximum capacity. Different allocations also prevent attributing differences
between these cohorts to the kernel. Fixed-rate native completion at 200 Mbps
is not a speedup. Absolute resource values and all workloads are below.

The [Travel comparison](stage5-travel.md) verifies all twelve balanced downloads;
one of twelve baseline A/A warmups fails. Stealth has higher observed CPU/byte
and RSS, while large UDP/TCP speed swings expose WAN variability. No clean WAN
speed ranking is claimed. Cleanup and all twelve independent restoration checks
pass. Its measured baseline is the latest selected production source.

There is also a separate compatibility blocker to adopting this prototype:
enabling both built-in rustls providers makes implicit builder selection
ambiguous until an application chooses a process default. Proton Local Agent
0.13.0's ConfigBuilder uses the implicit API, and the public transport accepts
externally built TLS connections. This follows from the pinned source and
[rustls's provider contract](https://docs.rs/rustls/latest/rustls/crypto/struct.CryptoProvider.html);
NoLocalAgent download success does not validate that broader integration.
The library does not silently install a process-global provider here.

No fixed CPU/RAM ceiling or minimum gain threshold decides this result. The full
matrix's mixed/non-improving CPU cost, higher Stealth RSS, lack of established
capacity improvement and unresolved compatibility do not justify production
adoption. Keep existing ring. This rejects these tested replacements/provider
design, not all crypto, native WireGuard, I/O, TLS or bypass architectures.
The deferred shortlist remains explicitly unbenchmarked.

## Coverage, correctness and reproduction

- 224/224 normal-release timed downloads verify: 32 focused observations and
  192 full-matrix observations, kept as separate cohorts. All 24 full-matrix
  cells pass control/timing screens. Full IPv4 HTTP/IPv6 HTTPS × one/eight flows
  × UDP/TCP/Stealth performance coverage is native Linux x64/ARM64 at 200 Mbps.
- LTS 6.12/6.18 performance coverage here is focused Stealth IPv4 HTTP/one-flow.
  UDP/TCP LTS performance is unmeasured in this stage. Windows/Apple/Android
  are build/contract checked, download-performance-unmeasured. Windows local
  primitive validation is not a platform download result.
- Known-answer and all directional primitive interoperability tests pass on
  native x64/ARM64 and locally on Windows. Initial/current primitive screens
  retain 432/576 observations respectively (1008 total, not downloads). The
  current RustCrypto screen enables key zeroization. No reduced-round cipher,
  forced backend, new packet algorithm, helper thread, PGO or native target flag.
- [All nine platform jobs for tested code](https://github.com/riKirsti/ProtunNext/actions/runs/43158753091),
  [release erasure](https://github.com/riKirsti/ProtunNext/actions/runs/43158753118)
  and [native checks](https://github.com/riKirsti/ProtunNext/actions/runs/43158753241)
  pass. These validate retained production; isolated TLS candidate checks validate
  the explicit-provider NoLocalAgent scope, not Local Agent/default-provider parity.
  Local Python checks pass (16 download and ten audit tests).
- Final `src/`, `vendor/`, `Cargo.toml` and `Cargo.lock` match merged PR #9.
  Production's dependency graph and implicit-provider behavior remain unchanged.
  The TLS patch is builder-only, digest checked before application solely to
  the candidate archive; manifests identify the patch and its limited scope.
  Reconstruction matches the tested runtime and effective source fingerprints
  of both variants, with canonical source ordering across Windows/Linux.
- Exact full-precision data, controls, source/archive/runtime/effective/binary
  identities, compiler/hardware information and raw primitive observations are
  in the [GitHub evidence supplement](stage5-gh-evidence.md), with links to all
  individual sample artifacts. [Travel evidence](stage5-travel-evidence.md)
  retains the driver, all sessions, failures and independent restoration checks.

## Separate diagnostic profiles

[Run 43158753819](https://github.com/riKirsti/ProtunNext/actions/runs/43158753819)
retains native [x64 artifact 13917457501](https://github.com/riKirsti/ProtunNext/actions/runs/43158753819/artifacts/13917457501)
and [ARM64 artifact 13917233327](https://github.com/riKirsti/ProtunNext/actions/runs/43158753819/artifacts/13917233327).
All 24 separate 1 GiB diagnostic downloads verify: baseline/prototype × three
transports × client-process/all-four-test-CPU captures × two architectures.
Frame-pointer diagnostics never supply the normal-release speed/resource
estimates. Fixed auxiliary work remains visible in the system captures.

The ARM64 candidate Stealth capture includes
`aws_lc_0_47_0_aes_gcm_dec_kernel`, while WireGuard opening stays in
`ring_core_0_17_14__chacha20_poly1305_open`. The x64 candidate also uses a
different AES kernel. This confirms the intended suite-provider implementation
is exercised; sampled shares do not prove a causal speed gain or explain the
entire RSS change. The whole-download comparisons decide selection.


## Current four-backend primitive screen

These are synthetic wall ns/operation, including input restoration, not download CPU/byte or Mbps. Six repeated samples per cell, geometric means; no forced CPU backend or native compiler target.

| Platform | Payload B | Operation | ring ns/op | RustCrypto 0.10.1 ns/op | RustCrypto 0.11.0 ns/op | AWS-LC 1.19.0 ns/op |
|---|---:|---|---:|---:|---:|---:|
| x64 | 0 | open | 168.99 | 1226.66 | 1184.28 | 159.88 |
| x64 | 0 | seal | 160.37 | 1212.61 | 1179.41 | 164.90 |
| x64 | 64 | open | 189.50 | 1794.13 | 1729.69 | 184.20 |
| x64 | 64 | seal | 182.96 | 1794.31 | 1756.28 | 191.98 |
| x64 | 256 | open | 274.57 | 1850.42 | 1798.40 | 266.57 |
| x64 | 256 | seal | 269.59 | 1844.04 | 1803.53 | 276.38 |
| x64 | 1280 | open | 758.69 | 2565.34 | 2513.40 | 754.36 |
| x64 | 1280 | seal | 740.09 | 2562.23 | 2518.47 | 747.83 |
| x64 | 1420 | open | 777.39 | 2917.09 | 2863.57 | 770.51 |
| x64 | 1420 | seal | 845.44 | 2911.73 | 2868.58 | 853.54 |
| x64 | 16384 | open | 7155.28 | 13151.60 | 13152.22 | 7169.77 |
| x64 | 16384 | seal | 7492.27 | 13328.70 | 13196.09 | 7338.10 |
| arm | 0 | open | 235.33 | 191.43 | 274.28 | 229.67 |
| arm | 0 | seal | 225.50 | 178.40 | 270.03 | 230.78 |
| arm | 64 | open | 254.78 | 331.78 | 496.69 | 250.52 |
| arm | 64 | seal | 246.09 | 326.08 | 501.59 | 253.32 |
| arm | 256 | open | 482.44 | 771.72 | 1168.13 | 476.97 |
| arm | 256 | seal | 379.95 | 764.40 | 1171.36 | 385.01 |
| arm | 1280 | open | 1310.67 | 3124.83 | 2912.50 | 1306.76 |
| arm | 1280 | seal | 1420.01 | 3117.03 | 2900.41 | 1426.49 |
| arm | 1420 | open | 1543.36 | 3541.65 | 3578.96 | 1539.43 |
| arm | 1420 | seal | 1484.21 | 3529.57 | 3561.66 | 1491.16 |
| arm | 16384 | open | 14504.40 | 37830.97 | 27894.69 | 14510.32 |
| arm | 16384 | seal | 14450.81 | 37805.77 | 27903.27 | 14446.18 |

## Normal-release download observations

Values compare frozen merged PR #9 / isolated AWS-LC TLS-suite prototype. Both use negotiated vnet and one allowed client CPU, with fixed baseline peer/load/server binaries on separate CPUs. CPU 100% means one core; RSS is peak client process memory, including setup/warmup, excluding kernel/auxiliaries. CPU ns/B uses verified payload bytes. TLS suite implementations change; WireGuard AEAD and UDP/TCP packet code remain ring. Fixed-rate results do not rank maximum capacity; unpaced LTS rows are auxiliary-limited.

### Focused Linux 6.12 x64 · unpaced

[Actions run](https://github.com/riKirsti/ProtunNext/actions/runs/43158753819) / [artifact 13917541273](https://github.com/riKirsti/ProtunNext/actions/runs/43158753819/artifacts/13917541273), 8 verified downloads.

| Workload | CPU core % baseline / prototype | RSS MiB baseline / prototype | Mbps baseline / prototype | CPU ns/B baseline / prototype | CPU cost change | Quality |
|---|---:|---:|---:|---:|---:|---|
| stealth-v4-http-1 | 95.67 / 93.89 | 2.95 / 4.15 | 4031.25 / 4026.41 | 1.90 / 1.87 | -1.74% | usable; helper-limited |

### Focused Linux 6.18 x64 · unpaced

[Actions run](https://github.com/riKirsti/ProtunNext/actions/runs/43158753819) / [artifact 13917212415](https://github.com/riKirsti/ProtunNext/actions/runs/43158753819/artifacts/13917212415), 8 verified downloads.

| Workload | CPU core % baseline / prototype | RSS MiB baseline / prototype | Mbps baseline / prototype | CPU ns/B baseline / prototype | CPU cost change | Quality |
|---|---:|---:|---:|---:|---:|---|
| stealth-v4-http-1 | 77.94 / 75.21 | 2.93 / 4.06 | 3641.81 / 3629.11 | 1.71 / 1.66 | -3.16% | usable; helper-limited |

### Focused native x64 · 200 Mbps

[Actions run](https://github.com/riKirsti/ProtunNext/actions/runs/43158751468) / [artifact 13916848276](https://github.com/riKirsti/ProtunNext/actions/runs/43158751468/artifacts/13916848276), 8 verified downloads.

| Workload | CPU core % baseline / prototype | RSS MiB baseline / prototype | Mbps baseline / prototype | CPU ns/B baseline / prototype | CPU cost change | Quality |
|---|---:|---:|---:|---:|---:|---|
| stealth-v4-http-1 | 4.31 / 4.14 | 5.02 / 6.91 | 200.08 / 200.08 | 1.72 / 1.66 | -3.88% | usable |

### Focused native ARM64 · 200 Mbps

[Actions run](https://github.com/riKirsti/ProtunNext/actions/runs/43158751468) / [artifact 13917242501](https://github.com/riKirsti/ProtunNext/actions/runs/43158751468/artifacts/13917242501), 8 verified downloads.

| Workload | CPU core % baseline / prototype | RSS MiB baseline / prototype | Mbps baseline / prototype | CPU ns/B baseline / prototype | CPU cost change | Quality |
|---|---:|---:|---:|---:|---:|---|
| stealth-v4-http-1 | 6.83 / 6.87 | 4.16 / 5.10 | 200.08 / 200.08 | 2.73 / 2.75 | +0.62% | usable |

### Full native x64 · 200 Mbps

[Actions run](https://github.com/riKirsti/ProtunNext/actions/runs/43159560703) / [artifact 13918016515](https://github.com/riKirsti/ProtunNext/actions/runs/43159560703/artifacts/13918016515), 96 verified downloads.

| Workload | CPU core % baseline / prototype | RSS MiB baseline / prototype | Mbps baseline / prototype | CPU ns/B baseline / prototype | CPU cost change | Quality |
|---|---:|---:|---:|---:|---:|---|
| udp-v4-http-1 | 4.95 / 4.88 | 7.96 / 7.90 | 200.04 / 200.08 | 1.98 / 1.95 | -1.40% | usable |
| udp-v4-http-8 | 5.44 / 5.44 | 8.38 / 8.98 | 200.71 / 200.71 | 2.17 / 2.17 | -0.17% | usable |
| udp-v6-https-1 | 5.16 / 5.08 | 8.29 / 8.82 | 200.08 / 200.08 | 2.06 / 2.03 | -1.51% | usable |
| udp-v6-https-8 | 5.90 / 5.90 | 8.59 / 9.04 | 200.71 / 200.70 | 2.35 / 2.35 | +0.12% | usable |
| tcp-v4-http-1 | 4.96 / 4.95 | 7.84 / 7.79 | 200.08 / 200.08 | 1.98 / 1.98 | -0.21% | usable |
| tcp-v4-http-8 | 5.24 / 5.41 | 8.15 / 8.41 | 200.71 / 200.71 | 2.09 / 2.16 | +3.35% | usable |
| tcp-v6-https-1 | 5.10 / 5.11 | 7.42 / 7.83 | 200.08 / 200.07 | 2.04 / 2.04 | +0.27% | usable |
| tcp-v6-https-8 | 5.47 / 5.53 | 7.56 / 8.18 | 200.71 / 200.71 | 2.18 / 2.20 | +1.07% | usable |
| stealth-v4-http-1 | 5.13 / 5.15 | 5.13 / 6.95 | 200.08 / 200.08 | 2.05 / 2.06 | +0.32% | usable |
| stealth-v4-http-8 | 5.72 / 5.72 | 6.51 / 8.28 | 200.71 / 200.71 | 2.28 / 2.28 | -0.01% | usable |
| stealth-v6-https-1 | 5.51 / 5.56 | 5.16 / 6.94 | 200.07 / 200.08 | 2.20 / 2.22 | +0.79% | usable |
| stealth-v6-https-8 | 5.93 / 6.01 | 5.14 / 6.98 | 200.71 / 200.71 | 2.36 / 2.40 | +1.43% | usable |

### Full native arm · 200 Mbps

[Actions run](https://github.com/riKirsti/ProtunNext/actions/runs/43159560703) / [artifact 13918390720](https://github.com/riKirsti/ProtunNext/actions/runs/43159560703/artifacts/13918390720), 96 verified downloads.

| Workload | CPU core % baseline / prototype | RSS MiB baseline / prototype | Mbps baseline / prototype | CPU ns/B baseline / prototype | CPU cost change | Quality |
|---|---:|---:|---:|---:|---:|---|
| udp-v4-http-1 | 5.83 / 5.73 | 7.85 / 7.43 | 200.08 / 200.08 | 2.33 / 2.29 | -1.70% | usable |
| udp-v4-http-8 | 5.55 / 5.56 | 7.66 / 7.97 | 200.72 / 200.72 | 2.21 / 2.21 | +0.05% | usable |
| udp-v6-https-1 | 6.50 / 6.39 | 7.14 / 7.09 | 200.08 / 200.08 | 2.60 / 2.56 | -1.65% | usable |
| udp-v6-https-8 | 5.86 / 5.81 | 7.91 / 8.04 | 200.72 / 200.72 | 2.33 / 2.32 | -0.79% | usable |
| tcp-v4-http-1 | 5.45 / 5.39 | 6.35 / 6.09 | 200.08 / 200.08 | 2.18 / 2.15 | -1.21% | usable |
| tcp-v4-http-8 | 5.39 / 5.37 | 6.96 / 7.34 | 200.72 / 200.72 | 2.15 / 2.14 | -0.44% | usable |
| tcp-v6-https-1 | 5.84 / 6.01 | 6.25 / 6.23 | 200.08 / 200.08 | 2.34 / 2.40 | +2.87% | usable |
| tcp-v6-https-8 | 5.56 / 5.60 | 6.74 / 7.41 | 200.72 / 200.72 | 2.22 / 2.23 | +0.70% | usable |
| stealth-v4-http-1 | 6.59 / 6.72 | 4.22 / 5.03 | 200.08 / 200.08 | 2.64 / 2.69 | +1.98% | usable |
| stealth-v4-http-8 | 6.50 / 6.53 | 5.49 / 6.28 | 200.71 / 200.71 | 2.59 / 2.60 | +0.54% | usable |
| stealth-v6-https-1 | 7.15 / 7.19 | 4.34 / 4.97 | 200.08 / 200.07 | 2.86 / 2.88 | +0.53% | usable |
| stealth-v6-https-8 | 6.67 / 6.84 | 4.28 / 5.47 | 200.71 / 200.71 | 2.66 / 2.72 | +2.45% | usable |

## Stage boundary

Stage 5 concludes with ring retained and the examined alternatives documented.
Native WireGuard, advanced I/O, CPU/NIC coordination and bypass remain deferred
for explicit coverage/privilege/lifecycle design and relevant measurements;
no exhaustive-backend or globally optimal performance claim is made. After
this stage merges, review Stage 6's client-only PGO, architecture-specific build
and platform-tuning plan against these results before implementation. No Stage 6
code or combination of unmeasured effects is included in this PR.
