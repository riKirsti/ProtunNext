# Performance-test audit and rerun — 2026-11-05

This report replaces performance conclusions from the earlier worker-next
campaign wherever the old peer, timing or shutdown behavior could affect them.
Production still changes only packet erasure: bulk zero fill followed by
`zeroize::optimization_barrier`, with the same erased region and lifetime.
Experimental GRO ownership, parallel crypto and kTLS are not shipping backends.

## Confirmed test defects and corrections

1. **The TLS peer did not implement vectored writes.** Pinned Rustls 0.23.46
   implements them for `Stream`, but its `StreamOwned` wrapper inherits Rust's
   first-nonempty-buffer default. The previous test peer therefore generated
   different TLS record batching than its framing code requested. The corrected
   wrapper explicitly forwards to `Stream::write_vectored`. A loopback regression
   test demonstrates both behaviors against the actual pinned library. Matched
   legacy/corrected-peer blocks quantify sensitivity without changing the client.
   This was a test limitation, not proof of a defect in Proton's servers.
2. **Shutdown could manufacture a failure after successful traffic.** Closing
   the peer before the client created a race returning `UnexpectedEof`. The peer
   now acknowledges a drain boundary, keeps its socket open while the client
   stops, and accepts expected closure only after that acknowledgement. Early
   closure during traffic still fails. Failed development reruns are retained.
   This explains the synthetic fixture's shutdown failures; it does not establish
   the cause of earlier real Proton HTTPS failures.
3. **The old peer busy-polled.** It now waits for socket readiness. Fixed peer
   and generator CPU are recorded separately; high auxiliary utilization flags
   a capacity limitation. Client process CPU is not total system CPU or energy.
4. **Warmup and measurement boundaries were inadequate.** The synthetic workload
   now warms for one second before a separate start gate. Five-second measured
   intervals end before generator sorting/serialization. Rate parsing is outside
   the packet loop. Travel's final WAN samples use 100 MB files and actual traffic
   warmup, rather than the preliminary sub-second 12 MB downloads.
5. **The load and CPU policy could bias conclusions.** New cases include burst
   and sliding windows, one/eight flows, sparse packets, fixed offered rate,
   same-executable A/A controls, and ABBA/BAAB implementation comparisons. The
   parallel prototype additionally receives the full available CPU set in a
   sensitivity test. That set overlaps the fixed auxiliaries; it is a scheduling
   sensitivity, not an isolated many-core throughput measurement.
6. **Router download tests cannot evaluate parallel TX crypto alone.** The
   prototype parallelizes encryption only; downloads mostly use unchanged RX
   decryption. A separate fixed-rate TX-sensitive TUN test exercises outbound
   payloads. Peer and generator load is low and measured, not assumed negligible.

The previously discovered shared-Cargo-target error remains invalidated. This
audit uses distinct per-variant build directories, distinct executable hashes,
compiled source identities and a fixed independently identified peer/generator.
It also checks the actual kTLS handoff marker, negotiated TLS version/cipher,
requested peer-write policy, data integrity and GSO activity.

Primary contracts checked:
[Rustls 0.23.46 stream source](https://github.com/rustls/rustls/blob/v/0.23.46/rustls/src/stream.rs),
[Rust Write default](https://doc.rust-lang.org/std/io/trait.Write.html#method.write_vectored),
[Linux 6.18 TLS documentation](https://docs.kernel.org/6.18/networking/tls.html),
[Rustls extracted-connection responsibilities](https://github.com/rustls/rustls/blob/v/0.23.46/rustls/src/conn/kernel.rs),
[zeroize barrier contract](https://github.com/RustCrypto/utils/blob/zeroize-v1.9.1/zeroize/src/barrier.rs).

## Test acceptance and limits

A/A uses the same executable on both sides. The predeclared engineering tolerance
is 5% for throughput and CPU per byte. This is a noise screen, not a significance
test. Incomplete/failed blocks and controls exceeding tolerance remain in the
evidence. No individual packets or repeated rounds are represented as independent
host allocations. CPU/byte counts one successfully echoed application byte, not
the sum of both traffic directions. Synthetic tests are IPv4 UDP echoes through
the real Connection/TUN/WireGuard worker, including TCP/Stealth outer transports;
they are not inner-TCP, NIC, mobile-device or Proton WAN performance ceilings.

Peer and generator utilization at or above 0.85 CPU core flags a possible
auxiliary bottleneck. Affinity does not isolate interrupts, asynchronous kernel
work or cloud-host scheduling. Fixed-rate cases evaluate cost at the achieved
load; they do not establish maximum throughput. Raw latency includes the load
generator and socket path and is not used to select production changes here.

The native x64 pilot exposes four logical CPUs on two SMT cores; ARM64 exposes
four single-threaded Neoverse-N2 cores. Distinct logical CPU affinities do not
guarantee distinct physical execution resources. These are explicit host-budget
comparisons, not an isolated many-core proof. Travel's low-rate check provides
separate evidence where auxiliary demand is below 0.073 CPU core.

## Travel rerun

Travel ran OpenWrt Linux **6.12.112**, in Direct mode, using its existing server
**US-NY#187** and account/key unchanged. The laptop generated verified HTTPS
range downloads from `cdn.kernel.org`; benchmark-only forwarding and source NAT
sent those connections through the owned temporary tunnel. Counter checks proved
the received body crossed that tunnel. Accept rules preceded the existing flow
offload rule. No global flow-offload or socket settings changed.

Windows selected Travel for each test socket with documented
[`IP_UNICAST_IF`](https://learn.microsoft.com/en-us/windows/win32/winsock/ipproto-ip-socket-options).
No Windows routes, firewall rules or administrator action were required. The
temporary administrator-route helper prepared earlier was never executed.

All **44 measured 100 MB downloads completed**, with TLS certificate validation,
HTTP 206, exact Content-Range/body length and tunnel-counter validation. This
window took **428.8 seconds**. Two initial kernel warmups timed out before the
third succeeded; those failures remain recorded. Their cause is not established.
The preliminary 12 MB window took 200.9 seconds and had warmup failures; its
sub-second downloads are retained as preliminary functional observations, not
used for final speed comparisons.

Geometric means of two observations on each side of a matched block:

| Transport / comparison | Baseline Mbps | Candidate Mbps | Candidate process CPU/byte |
|---|---:|---:|---:|
| UDP: main → erasure | 158.0 | 185.6 | -11.5% |
| TCP: main → erasure | 180.9 | 195.3 | -6.2% |
| Stealth: main → erasure | 134.2 | 156.0 | -4.8% |
| UDP: erasure → GRO ownership | 179.7 | 159.6 | +0.8% |
| UDP: erasure → parallel TX | 171.0 | 182.2 | -1.0% |
| TCP: erasure → parallel TX | 172.6 | 194.4 | -0.4% |
| Stealth: erasure → parallel TX | 152.5 | 137.0 | -0.2% |
| UDP: erasure → kernel WireGuard | 185.0 | 359.7 | Not comparable as process CPU |

**These WAN numbers are observations, not exact general speedup claims.**
UDP/TCP same-binary throughput controls differed **8.15% / 7.24%**, exceeding the
5% screen; Stealth differed **-2.73%**. CPU controls differed -2.75%, -1.58% and
+1.42%, respectively. The kernel observations were 351.8 and 367.8 Mbps. Its
advantage is large in this window, but two observations do not establish a
universal ratio. Download results do not establish a parallel-TX advantage.

### Controlled TX-sensitive router check

Twelve matched samples plus twelve separate same-binary controls used the real
worker and TUN in a private network namespace. Public fixture keys were used;
there was no account/server involvement. Each sample ran five measured seconds
after one second of warmup, with 1,200-byte payloads in 32-packet bursts at
**1,000 packets/second**. Every sample validated the payload and achieved the
offered rate within 5%. The fixed peer and generator shared one router CPU; the
client could use both. This setup measures cost at a low fixed load, not capacity.

| Outer transport | Serial ns/byte | Parallel ns/byte | CPU increase | Same-binary CPU difference |
|---|---:|---:|---:|---:|
| WireGuard UDP | 33.75 | 40.08 | +18.74% | -2.89% |
| WireGuard TCP | 35.39 | 41.24 | +16.52% | -0.44% |
| Stealth | 50.64 | 63.36 | +25.13% | -0.74% |

Combined peer/generator utilization never exceeded **0.073 CPU core**. Mid-run
thread snapshots recorded the crypto workers and their CPU counters. The result
rejects this batch-barrier prototype as a CPU optimization at this load; it does
not reject every possible asynchronous crypto pipeline or a many-core design.

### kTLS on Travel

The bounded TLS 1.3 fixture again failed key replacement with **EBUSY, errno 16**.
This is a functional capability failure, not a throughput comparison. Installing
additional userspace libraries does not demonstrate kernel key-update support.
Keep userspace TLS on this router. The kernel's initial-key acceptance must not
be presented as complete long-lived TLS support.

### Restoration

Each router window had independent timeout cleanup. Final checks confirmed
Direct mode, configuration/account-file hashes, package inventory, default
route, policy rules, interfaces, boot identity, netifd identity and socket limits
unchanged. Owned temporary interfaces, forwarding/NAT rules, binaries and copied
configuration files were removed. No package was installed, upgraded or removed.

## Native and supported-LTS results

### Actual Linux 6.18.60

[Corrected actual-LTS run](https://github.com/riKirsti/ProtunNext/actions/runs/42065421979):
96 successful samples, including twelve A/A and twelve implementation blocks.
All twelve same-binary controls met the 5% throughput/CPU screen. No block reached
the auxiliary utilization flag. Four virtual CPUs were pinned to distinct host
logical CPUs through QMP before guest execution; this does not establish physical
core or interrupt isolation. The earlier failed preflight was a KVM permission
failure, retained separately, and produced no usable performance measurement.

The table averages three blocks per workload **within one host allocation**:

| Stealth workload | kTLS throughput vs unbuffered Rustls | kTLS client CPU/byte |
|---|---:|---:|
| 1,200 bytes, window 32, one flow, burst | -3.36% | +13.70% |
| 1,200 bytes, window 32, one flow, sliding | -8.23% | +11.69% |
| 1,200 bytes, window 32, eight flows, sliding | -9.10% | +13.78% |
| 64 bytes, window 1 | -6.09% | +15.38% |

All sessions negotiated TLS 1.3 / AES-256-GCM. Each kTLS sample emitted its actual
kernel-handoff marker. Compiled source identities matched the generated overlay
manifest, and every payload passed integrity checks. The fixture exported build
logs, source identities, kernel configuration and CPU pins; it did **not** export
binary or kernel-image digests. Source hashes must not be represented as binary
hashes. The separate bounded functional fixture, rather than these short speed
samples, establishes the previously tested traffic-key-update behavior on 6.18.

The stable rerun rejects this bounded adapter as an improvement for these worker
loads on this guest. It does not establish a universal kTLS loss. Its known
nonblocking control-write recovery, fallback and record-limit gaps also remain
production gates. The [pinned Rustls API contract](https://github.com/rustls/rustls/blob/v/0.23.46/rustls/src/conn/kernel.rs)
explicitly leaves record-limit policy and substantial control handling to the
caller after secret extraction.

### Native x64 / ARM64

[Full corrected run](https://github.com/riKirsti/ProtunNext/actions/runs/42064242899):
**1,884 successful samples: 1,848 timed and 36 smoke samples**, across three
concurrent runner allocations per architecture. All six jobs passed. Distinct
runner IDs confirm separate allocations; the hosted images reuse their baked-in
hostnames, so hostname uniqueness was not used as evidence. Physical host
independence is not established. Each allocation passed 35 unit tests, including
the actual Rustls vectored-write regression and fragmented TLS-header meter.

All 462 four-observation blocks were complete. **88/90 A/A controls passed** the
5% screen. The two exceptions were ARM64 64-byte/window-1 UDP CPU (+5.60%) and TCP
CPU (-8.72%) controls on separate allocations. Those sparse cells are retained
and flagged, not silently rerun until they pass or used to select a backend.

Native hosts ran **Linux 7.0.0-1017-azure**, not 6.12/6.18. The following table
averages ratios from three allocation blocks. Workload: 1,200-byte application
payloads, window 32, one flow, burst. All underlying controls in this table passed
and none reached the auxiliary utilization flag. Positive throughput is better;
negative CPU per successfully echoed byte is better.

| Architecture | Comparison / outer transport | Throughput | Client CPU/byte |
|---|---|---:|---:|
| x64 | Main → erasure / UDP | +7.46% | -6.95% |
| x64 | Main → erasure / TCP | +3.13% | -3.49% |
| x64 | Main → erasure / Stealth | +3.93% | -5.75% |
| ARM64 | Main → erasure / UDP | +9.66% | -17.28% |
| ARM64 | Main → erasure / TCP | +4.39% | -12.99% |
| ARM64 | Main → erasure / Stealth | +7.10% | -9.22% |
| x64 | Erasure → final-GRO ownership / UDP | -0.01% | -0.15% |
| ARM64 | Erasure → final-GRO ownership / UDP | -0.73% | +0.58% |
| x64 | Erasure → parallel TX / UDP | -27.25% | +35.80% |
| x64 | Erasure → parallel TX / TCP | -27.36% | +34.37% |
| x64 | Erasure → parallel TX / Stealth | -29.28% | +41.67% |
| ARM64 | Erasure → parallel TX / UDP | -10.30% | +34.35% |
| ARM64 | Erasure → parallel TX / TCP | -8.77% | +30.67% |
| ARM64 | Erasure → parallel TX / Stealth | -4.19% | +24.78% |
| x64 | Erasure → kTLS / Stealth | -8.33% | +15.00% |
| ARM64 | Erasure → kTLS / Stealth | -1.42% | +9.71% |

The full summary retains sliding-window, eight-flow, rate-limited and sparse
cells, per-allocation ranges, CPU-budget sensitivity and auxiliary flags. Some
continuous-load x64 cases hit the auxiliary flag; their throughput is not used
as a client capacity ranking. At 10,000 packets/second, achieved load stayed
constant and erasure reduced CPU/byte by 8.84% / 5.48% / 5.66% for x64 UDP/TCP/
Stealth and 17.20% / 11.38% / 7.42% for ARM64. These fixed-load checks support the
cost reduction independently of saturated throughput.

Giving the parallel client all four available logical CPUs did not reverse the
tested design's CPU regression: burst CPU/byte increased 33–39% on x64 and
21–25% on ARM64. The overlapping auxiliary assignments and SMT limit remain;
this is not evidence against every many-core or asynchronous pipeline.

### How much did the TLS test peer matter?

Changing **only the peer's write behavior**, with the same erasure client,
increased observed burst throughput **80.45% on x64 and 14.19% on ARM64**; client
CPU/byte fell 36.11% and 12.50%. The legacy x64 peer hit the auxiliary utilization
flag. These are **test-peer effects, not ProtunNext code gains**. The TLS record
meter shows the legacy peer emits the encrypted records in the <=2 KB bucket;
the intended vectored peer also emits larger records. The old traffic was valid
TLS, but did not implement the intended peer batching model. Both behaviors now
have explicit sensitivity coverage. Neither model establishes Proton's actual
server record distribution.

Two corrected pilot allocations per architecture also completed successfully;
they are retained separately and are not pooled into the full-run figures.

## Decision and platform suggestions

* **General:** keep bulk packet erasure and the existing production packet path.
  Its erasure contract, optimized stores and CPU reduction have independent
  source, functional and measurement support. Do not promote the tested
  final-GRO ownership handoff, batch-barrier parallel TX or kTLS adapter.
* **Linux 6.12 / Travel:** retain existing mmsg, outer UDP GSO/GRO, in-place crypto
  and unbuffered userspace TLS. The rejected GRO experiment is only the additional
  ownership handoff; existing GSO/GRO remains. Kernel WireGuard remains the UDP
  reference and was much faster in this WAN window. The difference does not by
  itself identify `ring` or a single source of overhead.
* **Linux 6.18:** retain the same userspace path. The corrected actual-LTS kTLS
  worker comparison is stable and unfavorable for this prototype. Future kTLS
  work needs a different measured design plus complete nonblocking/control/
  record-limit handling before production consideration.
* **Windows x64 and Apple ARM64:** preserve current native adapters and buffered
  Rustls. No Windows/Apple VPN performance gain is inferred from Linux results.
* **Android 16/17 ARM64:** preserve the current adapter and unbuffered Rustls.
  This campaign did not measure physical Android device performance.

The shipping runtime (`src`, Cargo files, vendor and platform code) is byte-for-
byte unchanged from the previously verified erasure candidate 849be97. The new
work corrects experiments and evidence. Cross-platform verification
[run 42065435064](https://github.com/riKirsti/ProtunNext/actions/runs/42065435064)
passed eight jobs; macOS 26 was canceled before test steps, so it does not supply
a new pass. [Run 42039911247](https://github.com/riKirsti/ProtunNext/actions/runs/42039911247)
passed all nine jobs at 811764c, whose shipping runtime is also identical to this
proposal. These statuses were checked directly. No release or production
deployment was made.

## Reproduction and evidence

The repository contains the corrected probe, archive construction, predeclared
matrix, common analysis and actual-LTS runner under `experiments/audit` and
`experiments/next`. The summary JSON retains all workload cells and allocation
ranges. The evidence ZIP includes original native/LTS artifacts, failures from
development, router measurements and the scripts used to analyze them. Public
router evidence omits saved-configuration fingerprints and raw process addresses;
the full private evidence remains in the local workspace. No credentials or VPN
private keys are included.

Files: [summary JSON](test-audit-20261105.json),
[evidence ZIP](test-audit-20261105-evidence.zip),
[ZIP SHA-256](test-audit-20261105-evidence.sha256).
