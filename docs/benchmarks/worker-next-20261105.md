# Further worker performance results — 2026-11-05

The production proposal is **bulk packet erasure only**. The GRO ownership
handoff was removed after mixed architecture results. Parallel crypto and kTLS
remain isolated experiments outside the shipping build. There was no release,
router deployment, account access or router package/configuration change.

## Final production change

Baseline: `c4b2554398ffcbf35cdaa9c8797504d8df146bd9` (merged main).
Measured candidate: `849be97a207ec0762e1b3660fd61db4a77d2bca8`.

Packet recycling and destruction retain their existing erasure region and
lifetime. A bulk zero fill followed by `zeroize::optimization_barrier` replaces
byte-wise volatile erasure. The zeroize dependency floor is now 1.9.1, which
provides this public API. Key erasure elsewhere, packet ownership, queue sizes,
TLS selection, public APIs and platform bindings are unchanged.

This follows the library's documented writes-then-barrier pattern in its
[official source](https://github.com/RustCrypto/utils/blob/zeroize-v1.9.1/zeroize/src/barrier.rs).
Tests cover unaligned spans, adjacent guards and a shortened packet retaining a
previously longer live region. Optimized native x64/ARM64 assemblies retain bulk
zeroing calls. Runtime tests alone would not prove resistance to dead-store
elimination; source-contract and generated-code checks accompany them.

## Repeated worker results

[Final erasure-only run](https://github.com/riKirsti/ProtunNext/actions/runs/41976010102):
648 successful timed samples, no failed samples, three allocated hosts per
architecture, six balanced matched pairs per cell on each host. Each cell below
therefore has 18 pairs across three allocations. Figures are geometric means of
paired ratios, with equal weight per allocation. No narrow confidence interval
is inferred from correlated packets or repeated rounds.

Bulk application payload is 1,200 bytes, window 32. Positive throughput is better;
negative client CPU per byte is better.

| Architecture | Transport | One-flow throughput | One-flow CPU/byte | Eight-flow throughput | Eight-flow CPU/byte |
|---|---|---:|---:|---:|---:|
| x64 | WireGuard UDP | +4.86% | -7.86% | +4.12% | -7.14% |
| x64 | WireGuard TCP | +2.20% | -4.26% | +1.68% | -3.91% |
| x64 | Stealth | +0.45% | -1.94% | -0.20% | -1.81% |
| ARM64 | WireGuard UDP | +6.37% | -18.40% | +5.71% | -17.03% |
| ARM64 | WireGuard TCP | +3.02% | -13.76% | +2.67% | -12.34% |
| ARM64 | Stealth | +5.95% | -10.69% | +5.80% | -9.95% |

The x64 Stealth throughput result is effectively flat, including a small measured
eight-flow regression. Sparse 64-byte/window-1 results are mixed and much smaller;
all cells, allocation ranges and measured p99 values are in the
[summary JSON](worker-next-20261105.json). Bulk cases do not have a measured p99;
null values must not be converted into a latency claim.

Native performance hosts ran Linux **7.0.0-1017-azure**. These results must not be
labeled measured 6.12/6.18 performance. The separate supported-LTS functional
checks below establish behavior, not these exact performance percentages.

The real Connection/TUN/WireGuard worker carries synthetic IPv4 UDP echoes over
UDP, framed TCP or Stealth. Peer and generator are separate processes, always
from the fixed baseline binary. Two CPUs are available to the client, one to the
peer and one to the generator. Affinity does not isolate interrupts or all
asynchronous kernel work. The peer busy-polls; its CPU utilization cannot by
itself establish available capacity or a throughput ceiling.

CPU is measured for the client process only, including synchronous system time
and excluding peer/generator CPU and asynchronous work outside that process. The
denominator is one successfully echoed application byte, not the sum of traffic
in both directions. Warm-up authentication is excluded. RSS is client lifetime
peak, not pool reservation. These are **not inner TCP/HTTPS, NIC, Proton WAN,
mobile power or real-account acceptance tests**.

## Comparisons rejected or kept experimental

### Final-GRO-segment ownership

A prior three-variant campaign compared main, GRO-only and GRO-plus-erasure:
[run 41973962627](https://github.com/riKirsti/ProtunNext/actions/runs/41973962627).
All 972 timed samples passed executable identity and data validation. GRO and GSO
were exercised in the bulk UDP cells. GRO-only throughput changed about +1.8–2.0%
on x64 and -1.2–1.4% on ARM64. These are not universal architecture laws. The small
mixed result did not justify changing the common production path. The experiment
remains reproducible at its recorded revisions; final production GRO handling
is exactly main's existing implementation.

### Bounded parallel crypto

Two persistent TX crypto workers reserve nonces on the serial owner before
execution and deliver completions in input order. Immutable keys cross the
boundary; tunnel state, timers and statistics stay on the owner. Completion checks
key identity as well as the session index. Tests cover sparse/full batches,
ordinary peer interoperability, duplicates, tampering, expiry and shutdown with
owned work in flight. Detached RX tickets have regression coverage but are not
enabled in the measured worker.

The design takes account of WireGuard's
[serial/parallel queue discussion](https://www.wireguard.com/papers/wireguard-netdev22.pdf).
It does not assume primitive throughput predicts VPN throughput.

The first per-packet-dispatch/batch-barrier experiment regressed:
[41972734509](https://github.com/riKirsti/ProtunNext/actions/runs/41972734509).
A refinement dispatches reusable whole batches and keeps batches below eight
packets inline:
[41974064959](https://github.com/riKirsti/ProtunNext/actions/runs/41974064959).
Each has six pairs per cell on one allocation per architecture. The refinement
still reduced bulk UDP throughput by about 29% on x64 and 10–11% on ARM64, while
raising CPU/byte about 45% and 31–34%. It does not belong in production. This
rejects the tested barrier designs, not every possible asynchronous pipeline.

### Kernel TLS

The bounded TLS 1.3 functional probe passed data, RX/TX traffic-key updates,
session tickets and clean closure on actual Linux 6.18.60. Kernel counters showed
one RX and two TX rekeys with no rekey errors. Actual Linux 6.12.115 accepted the
initial keys but rejected replacement with EBUSY. That is a demonstrated
limitation, not successful long-lived kTLS acceptance. Native x64/ARM64 also passed
the bounded fixture. The negotiated cipher was AES-256-GCM. TLS 1.2 is outside
this prototype: a userspace_required declaration is not a TLS 1.2 kTLS test pass.

Evidence and contracts:
[functional run](https://github.com/riKirsti/ProtunNext/actions/runs/41971072655),
[Linux 6.18 documentation](https://docs.kernel.org/6.18/networking/tls.html),
[Linux socket-option source](https://github.com/torvalds/linux/blob/v6.18/net/tls/tls_main.c),
[pinned Rustls kernel API](https://github.com/rustls/rustls/blob/v/0.23.46/rustls/src/conn/kernel.rs).
Rustls delegates substantial control handling and cipher-limit policy to the
caller after extraction; initial handoff success is insufficient for production.

A separate outer-TLS component screen borrows decrypted records in both backends
and uses the same independent Rustls peer. The native hosts did not show a
consistent advantage. The actual 6.18 guest improved larger-message throughput
about 7–10% and reduced CPU/byte about 9–14%:
[component run](https://github.com/riKirsti/ProtunNext/actions/runs/41973614123).
Component results cannot establish complete Stealth performance.

The follow-up executes the real worker with kTLS vectored writes, preserving the
caller's socket binding, trust policy and SNI:
[worker run](https://github.com/riKirsti/ProtunNext/actions/runs/41975432921).
It has two pairs per cell on one allocation for each kernel/architecture case.

| Worker environment | Bulk Stealth throughput, one/eight flows | CPU/byte, one/eight flows | Interpretation |
|---|---:|---:|---|
| Native x64, Linux 7.0 | +6.16% / +7.45% | +7.98% / +7.23% | Throughput/CPU tradeoff; sparse throughput -11.17% |
| Native ARM64, Linux 7.0 | -3.27% / -2.86% | +16.04% / +15.17% | Tested candidate loses; sparse throughput -14.70% |
| Actual x64 6.18.60 KVM guest | -12.01% / -0.25% | +2.40% / +8.21% | Performance inconclusive: unchanged controls swung substantially |

The 6.18 guest's unchanged UDP control changed +20.90% in one bulk cell and its
unchanged TCP control changed -19.04% in another. Its functional pass and payload
checks do not make that a stable performance comparison. Do not attribute small
worker CPU differences there to kTLS or proclaim a universal kTLS loss.

The worker adapter has blocking setup and incomplete nonblocking control-write
recovery, fallback and long-lived record-limit handling. It is restricted to
bounded fixtures and cannot be selected by production code. Its diagnostics do
not supply comparable underlying syscall counters. Keep userspace TLS as the
production default. A production kTLS proposal needs repeated stable supported-
kernel worker gains and the remaining lifecycle/partial-I/O gates first.

## Verification and platform decisions

[All nine verification jobs passed](https://github.com/riKirsti/ProtunNext/actions/runs/41976014941)
for measured source 849be97: Windows, macOS 26, Xcode 27, Android native packaging,
Linux native/TUN, ARM64-musl, and actual 6.12.115/6.18.60 guests. Local shipping
Windows verification also passed 68 tests after rebuilding the vendor dependency
from the shipping source. SDK/build checks are not physical-device performance
or Proton Plus acceptance. Ordinary TLS and Stealth regression coverage remains.

* General: keep the small erasure improvement and the existing packet path.
  Do not add queues, workers or backend switches without a measured advantage.
* Linux x64/ARM64: preserve mmsg, outer UDP GSO/GRO and in-place crypto. Keep
  unbuffered Rustls. Linux 6.12 keeps userspace TLS; kTLS is still research on 6.18.
* Windows x64 and Apple ARM64: keep buffered Rustls and existing native adapters.
  No native VPN performance improvement is claimed from Linux measurements.
* Android 16/17 ARM64: keep its current unbuffered adapter and service contracts.
  Device performance remains unmeasured in this campaign.

The resulting profiles still contain significant kernel/networking and WireGuard
AEAD work. Next candidates should address those measured costs. An asynchronous
crypto design must avoid the tested handoff/barrier cost. Any new io_uring or PGO
comparison must isolate the client and preserve liveness/ownership; neither was
added to production here. Flow queues/multiqueue remain conditional research.
Tailscale's [article](https://tailscale.com/blog/making-tailscale-faster) informed
questions about memory ownership, queue depth and concurrency; it does not prove
that ProtunNext should duplicate its design or gains.

## Validity correction and durable evidence

Initial runs 41969099610, 41970017615 and 41971733173 shared one Cargo target
folder across archived packages. Cargo reused the baseline executable; their
variant hashes are identical. They cannot establish candidate speedups or losses.
Baseline profiles still describe baseline work. The first also failed artifact
upload because profiler files were root-owned. Superseded runs 41972470951,
41972471050 and 41972532343, plus later superseded push/verification attempts,
were canceled rather than treated as test passes.

Corrected builds isolate every target folder, hash the effective source after
transformations, embed the identity in each executable, and verify it at runtime.
The runner rejects equal executable hashes and wrong identities. Generated code
is checked separately. The invalid A/A samples remain explicitly marked in the
[evidence ZIP](worker-next-20261105-evidence.zip), alongside the final data,
experimental results, unit logs, hardware descriptions and flat profile reports.
[ZIP SHA256](worker-next-20261105-evidence.sha256) accompanies it. Original artifact
ZIP digests are retained in manifests. Full disassembly and raw perf.data remain
in the linked GitHub artifacts and verified local downloads.

Reproduce the aggregation after extracting the ZIP:

```sh
python scripts/analyze-worker-next.py erasure-final/x64-1 erasure-final/x64-2 erasure-final/x64-3 erasure-final/arm64-1 erasure-final/arm64-2 erasure-final/arm64-3 --output result.json
```

The script validates provenance and all sample outcomes before calculating
ratios. The public summary contains per-allocation data and aggregation details.
No recommendation relies on the invalid comparisons or treats an inconclusive
performance run as proof of a backend advantage.
