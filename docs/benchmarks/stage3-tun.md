# Stage 3: negotiated receive-side TUN coalescing

**Decision:** retain explicitly negotiated Linux receive-side coalescing as a client CPU/memory efficiency improvement. Existing raw callers keep ordinary delivery. Maximum client throughput remains unproven; this is not a universal platform speedup or an optimality claim.

Frozen baseline: merged PR #7, `f26fd3d741d125037797ec9953de1d50ebd93c8a`.

## Descriptor contract

Existing raw `IFF_TUN | IFF_NO_PI` descriptors keep their existing ABI and packet
delivery. An integrating Linux caller can create a **new, exclusively owned**
`IFF_TUN | IFF_NO_PI | IFF_VNET_HDR` descriptor, call
`linux::prepare_receive_coalescing(fd)`, then attach it normally. Discard the new
descriptor if preparation fails: its setup ioctls are not transactional. The
helper rejects raw descriptors before changing settings. It sets ten-byte,
little-endian virtio headers and disables kernel-to-userspace offloads, so
outbound packets remain ordinary fully checksummed IP. Attachment checks the
negotiated header ABI; reads fail closed on unexpected outbound GSO/checksum
metadata. Android, Apple and Windows retain ordinary packet delivery.

The implementation combines only consecutive checksum-valid TCP data with
matching flow, ACK, window, options and IP attributes; sequence numbers must be
contiguous. IPv4 requires a plain header and DF; IPv6 requires TCP without
extension headers. ECN, control packets, fragments, flow/option changes, invalid
checksums and other unsupported cases retain ordinary delivery. A PSH or short
segment ends a group. A group contains at most 32 original packets and 65,535
IP bytes. No reordering, delayed flush, extra packet storage or per-group heap
allocation is introduced.

One `writev` submits metadata, the updated header and immutable queued payloads.
Copied IP/TCP header scratch is erased after every aggregate write attempt.
Only an atomic successful write acknowledges originals. Backpressure preserves
all originals; unsupported aggregate submission disables aggregation and retries
ordinary delivery. New counters report TUN write attempts, accepted originals,
coalesced originals, maximum group and offload fallbacks. These diagnose behavior,
not performance by themselves.

## Controlled comparison

The existing download runners accept `tun_mode=compare`. Baseline and A/A clients
use raw TUN; the candidate uses explicitly negotiated vnet. Peer and load binaries
remain fixed baseline builds with raw peer TUN. The same fixture overlay is
injected into both source archives; no candidate API is linked into the baseline.
Recorded client descriptor flags must match the planned mode. Candidate samples
must actually coalesce, stay within the group bound and have zero offload fallback.
Candidate smoke coverage precedes timed samples. Normal release timing remains
separate from frame-pointer profiling.

The setup follows the Linux [TUN ioctl implementation](https://github.com/torvalds/linux/blob/v6.12/drivers/net/tun.c),
[virtio metadata conversion](https://github.com/torvalds/linux/blob/v6.12/include/linux/virtio_net.h)
and [virtio header ABI](https://github.com/torvalds/linux/blob/v6.12/include/uapi/linux/virtio_net.h).
Real supported-kernel downloads below establish end-to-end checksum and
segmentation correctness for the tested workloads; metadata tests alone do not.


## Final selection and measurements

Tested final production source: `5dba6015353d20127c772ce80cd90063e6d5c6b0`, including header-scratch erasure. All **576/576** planned timed downloads are valid: 384 unpaced transfers of exactly 2 GiB and 192 matched-load transfers of exactly 128 MiB. All 72 workload cells pass their A/A and timing screens. Each cell includes four baseline A/A controls and a balanced four-sample baseline/candidate comparison. This is one allocation per target/cohort, not a confidence interval or a pooled speedup experiment.

Client CPU cost falls in every final cell. At 200 Mbps, native x64 reductions are **40.26–50.08%**, and ARM64 reductions **26.20–38.92%** across UDP, TCP and Stealth, IPv4 HTTP / IPv6 HTTPS and one/eight inner flows. Peak client RSS also falls in every final cell. The matched-load cells have auxiliary headroom, but their imposed rate cannot establish maximum capacity. All 48 unpaced cells are helper-limited, so their observed speed changes are descriptive rather than maximum-client-throughput evidence.

CPU core % uses 100% = one fully occupied core and can exceed 100% for multiple threads. CPU is process CPU per verified downloaded byte, including synchronous kernel work on client threads and excluding deferred kernel work. RSS is lifetime client high-water resident memory including setup/warmup; it excludes kernel and auxiliary memory. Positive changes below mean higher cost. All measurements use normal release builds; profiles below are separate.

| Target / workload | Client CPU cost reduction across 12 cells | Peak RSS reduction across 12 cells | Quality and interpretation |
|---|---:|---:|---|
| Linux 6.12.115 x64, KVM | 19.45–32.31% | 9.37–53.97% | 12/12 usable; helper-limited, no capacity ranking |
| Linux 6.18.60 x64, KVM | 14.84–24.85% | 14.19–50.65% | 12/12 usable; helper-limited, no capacity ranking |
| Native x64, unpaced | 26.48–34.43% | 12.25–27.69% | 12/12 usable; helper-limited, no capacity ranking |
| Native ARM64, unpaced | 19.56–34.62% | 25.14–41.85% | 12/12 usable; helper-limited, no capacity ranking |
| Native x64, 200 Mbps | 40.26–50.08% | 14.72–36.52% | 12/12 usable; matched load, no capacity ranking |
| Native ARM64, 200 Mbps | 26.20–38.92% | 18.51–44.37% | 12/12 usable; matched load, no capacity ranking |

Every final candidate sample exercises coalescing with zero offload fallback, zero TUN backpressure and zero pool-pressure events. At least 99.54% of accepted original packets participate in groups in each unpaced candidate sample; at least 98.18% at matched load. Maximum observed group is 32. Counters include setup/warmup and do not independently establish a performance improvement.

### Recommendation by target

| Target | Decision | Evidence limit |
|---|---|---|
| Linux 6.12 / 6.18 x64 | Retain negotiated coalescing; raw callers remain compatible | KVM loopback measurements; physical router/NIC/WAN behavior unverified |
| Native Linux ARM64 / x64 | Retain the explicit capability based on matched-load CPU/RSS improvements | Recorded host kernel is 7.0.0-1017-azure; the separate Travel 6.12 test below has noisy WAN controls and higher RSS; ARM64 6.18 devices remain unmeasured |
| Android / Apple / Windows | Keep ordinary packet delivery | Cross-platform regressions/builds pass; no coalescing or download-speed claim on these platforms |

Keep UDP batch 16, existing worker budgets/queue bounds, crypto selection and TLS adapters. No delayed aggregation, cross-flow reordering, new backend, PGO, persistent router changes or release is selected in this stage. The bounded Travel test below adds physical forwarding and Proton WAN observations; loaded latency, controlled NIC/interrupt behavior, kernel memory, thermal behavior, mobile devices and repeatable multi-device/WAN acceptance remain explicit gaps. Lower client CPU does not establish lower total-machine energy.

### Physical Travel supplement

The [latest-code Travel test](stage3-travel.md) reports absolute CPU %, memory and
download speed for all three transports, separately from GitHub Actions. All
twelve balanced baseline/candidate comparison downloads complete and the six
candidate sessions coalesce real WAN traffic. Of 24 planned measured sessions,
23 succeed; one baseline UDP A/A warmup fails. All WAN control cohorts are noisy
or incomplete, preventing a clean speed ranking. Client CPU/byte is descriptively
lower, but peak client RSS is higher on Travel, so the GitHub memory result is
not generalized to the physical router. Independent postflight confirms restored
Direct mode and unchanged saved configuration, account/key, packages and normal
routing, with all owned temporary artifacts and watchdog removed.

## Profiles and remaining bottlenecks

Separate frame-pointer diagnostics retain baseline/raw and candidate/vnet builds, client-process captures and all-four-test-CPU captures on native x64 and ARM64. All 24 diagnostic 1 GiB downloads verify their payloads; the twelve candidate diagnostic runs actually coalesce. Raw perf data, symbolized reports, fixture logs, source/binary identities and compiler flags are retained in the linked native unpaced artifacts. They are excluded from the 576 normal-release timing samples.

In the client-process candidate profiles, WireGuard ChaCha20-Poly1305 opening is the largest flat sampled symbol: about 33–34% for x64 UDP/TCP and 19% for x64 Stealth, and 38–45% across ARM64 transports. x64 Stealth AES-GCM opening is another 12%; ARM64 Stealth AES-GCM is about 14%. Coalescing checksum validation accounts for roughly 4–5.5%. The baseline profiles instead retain prominent TUN/syscall/TCP receive costs alongside crypto. These are sampled shares from instrumented runs, not calibrated CPU/byte or normal-release capacity estimates.

The all-CPU captures include idle, helpers, softirq/interrupt context and deferred work; they are retained to expose work outside the client process. Differing durations and background/helper work prevent turning those percentage reports into a total-system CPU or energy claim. The next-stage inference is that ordered multicore receive/decryption deserves investigation after reviewing the merged baseline and these profiles; it is not implemented here.

## Validation and source identity

[All nine platform jobs pass](https://github.com/riKirsti/ProtunNext/actions/runs/42728574783) on the final production source, including actual 6.12/6.18 kernels, real TUN, ARM64-musl, Windows, Apple SDK and Android checks. [Release erasure checks pass on x64 and ARM64](https://github.com/riKirsti/ProtunNext/actions/runs/42728574967). The [native checks](https://github.com/riKirsti/ProtunNext/actions/runs/42728570056) pass 55 injected Rust library tests per architecture and 25 Python analyzer tests. Coverage includes valid-checksum boundary changes, malformed/checksum-invalid packets, IPv4/IPv6 metadata, sequence wrap, short tails/PSH, size/group bounds, immutable originals, real EAGAIN ownership, one-time aggregate rejection/fallback, raw descriptor preservation and negotiated roundtrips. Local Windows all-target/all-feature tests and Linux ARM64 compile/lint checks also pass.

The archive builder injects the same fixture and diagnostic overlays into both variants. Fixed peer/load binaries use the PR #7 baseline. Each cohort retains archive/effective/runtime fingerprints, binary SHA-256 hashes, actual kernel/toolchain, plan, all samples, controls and analysis in its raw evidence link. For the final cohorts, effective source fingerprints are:

```text
baseline:  fb4f32f8ae59fae3dd383aef79db556e92e2c600e113eb7123e02189c999c5e6
candidate: 84bde952a66099689d07b271a9740e39d1f8a10601854ea9b7be97e1637afc99
```

## Complete final workload results

### Linux 6.12.115 x64, KVM

[Raw evidence](https://github.com/riKirsti/ProtunNext/actions/runs/42728567350/artifacts/13723629998). Kernel `6.12.115`; candidate `5dba6015353d20127c772ce80cd90063e6d5c6b0`.

Planned/observed samples: 96/96; invalid-data errors: 0.

| Workload | Mbps baseline / candidate | CPU ns/B baseline / candidate | CPU core % baseline / candidate | CPU change | RSS MiB baseline / candidate | RSS change | Screen |
|---|---:|---:|---:|---:|---:|---:|---|
| udp-v4-http-1 | 1951.08 / 1970.03 | 4.050 / 2.941 | 98.77 / 72.43 | -27.37% | 6.40 / 4.03 | -37.02% | usable; auxiliary-limited |
| udp-v4-http-8 | 1721.40 / 1819.25 | 4.642 / 3.451 | 99.88 / 78.47 | -25.67% | 6.63 / 4.15 | -37.35% | usable; auxiliary-limited |
| udp-v6-https-1 | 1862.24 / 1850.72 | 4.289 / 2.964 | 99.85 / 68.57 | -30.90% | 5.27 / 4.39 | -16.58% | usable; auxiliary-limited |
| udp-v6-https-8 | 1617.58 / 1696.54 | 4.940 / 3.386 | 99.89 / 71.81 | -31.45% | 6.28 / 4.21 | -32.91% | usable; auxiliary-limited |
| tcp-v4-http-1 | 2090.27 / 2057.20 | 3.825 / 2.931 | 99.94 / 75.36 | -23.38% | 4.58 / 2.69 | -41.18% | usable; auxiliary-limited |
| tcp-v4-http-8 | 1834.31 / 1888.34 | 4.356 / 3.421 | 99.88 / 80.76 | -21.46% | 6.01 / 2.90 | -51.68% | usable; auxiliary-limited |
| tcp-v6-https-1 | 1833.21 / 1938.90 | 4.360 / 2.952 | 99.91 / 71.53 | -32.31% | 6.01 / 2.77 | -53.97% | usable; auxiliary-limited |
| tcp-v6-https-8 | 1677.41 / 1781.65 | 4.768 / 3.388 | 99.97 / 75.46 | -28.93% | 6.03 / 3.16 | -47.70% | usable; auxiliary-limited |
| stealth-v4-http-1 | 1852.02 / 1808.63 | 4.312 / 3.373 | 99.82 / 76.25 | -21.78% | 3.20 / 2.90 | -9.37% | usable; auxiliary-limited |
| stealth-v4-http-8 | 1643.97 / 1672.79 | 4.866 / 3.919 | 99.99 / 81.95 | -19.45% | 3.73 / 2.93 | -21.48% | usable; auxiliary-limited |
| stealth-v6-https-1 | 1611.81 / 1605.23 | 4.847 / 3.413 | 97.66 / 68.49 | -29.58% | 3.26 / 2.95 | -9.47% | usable; auxiliary-limited |
| stealth-v6-https-8 | 1582.55 / 1613.45 | 5.045 / 3.776 | 99.79 / 76.15 | -25.15% | 3.54 / 2.92 | -17.59% | usable; auxiliary-limited |

Candidate counters: at least 99.55% of accepted original packets participate in coalesced groups per sample; maximum group 32; offload fallbacks 0. Counters include setup/warmup, not just the timed body.

### Linux 6.18.60 x64, KVM

[Raw evidence](https://github.com/riKirsti/ProtunNext/actions/runs/42728567350/artifacts/13723778588). Kernel `6.18.60`; candidate `5dba6015353d20127c772ce80cd90063e6d5c6b0`.

Planned/observed samples: 96/96; invalid-data errors: 0.

| Workload | Mbps baseline / candidate | CPU ns/B baseline / candidate | CPU core % baseline / candidate | CPU change | RSS MiB baseline / candidate | RSS change | Screen |
|---|---:|---:|---:|---:|---:|---:|---|
| udp-v4-http-1 | 3246.52 / 3569.68 | 2.441 / 1.861 | 99.06 / 83.04 | -23.76% | 6.35 / 4.17 | -34.29% | usable; auxiliary-limited |
| udp-v4-http-8 | 3022.35 / 3265.88 | 2.644 / 2.043 | 99.88 / 83.39 | -22.74% | 6.27 / 4.15 | -33.74% | usable; auxiliary-limited |
| udp-v6-https-1 | 3149.76 / 3413.37 | 2.529 / 1.903 | 99.56 / 81.19 | -24.75% | 4.92 / 4.05 | -17.63% | usable; auxiliary-limited |
| udp-v6-https-8 | 2874.37 / 3111.14 | 2.781 / 2.096 | 99.90 / 81.50 | -24.63% | 6.39 / 3.97 | -37.88% | usable; auxiliary-limited |
| tcp-v4-http-1 | 3387.10 / 3675.44 | 2.362 / 1.859 | 99.99 / 85.43 | -21.27% | 5.73 / 2.83 | -50.65% | usable; auxiliary-limited |
| tcp-v4-http-8 | 3190.70 / 3344.35 | 2.507 / 2.075 | 99.99 / 86.76 | -17.22% | 5.99 / 2.99 | -50.13% | usable; auxiliary-limited |
| tcp-v6-https-1 | 3116.44 / 3498.53 | 2.567 / 1.929 | 99.99 / 84.35 | -24.85% | 5.16 / 2.76 | -46.44% | usable; auxiliary-limited |
| tcp-v6-https-8 | 2913.23 / 3153.25 | 2.746 / 2.134 | 99.99 / 84.09 | -22.30% | 5.48 / 2.89 | -47.23% | usable; auxiliary-limited |
| stealth-v4-http-1 | 2974.09 / 3159.04 | 2.690 / 2.180 | 99.99 / 86.09 | -18.95% | 3.43 / 2.94 | -14.19% | usable; auxiliary-limited |
| stealth-v4-http-8 | 2813.33 / 2888.24 | 2.843 / 2.422 | 99.99 / 87.42 | -14.84% | 3.62 / 2.90 | -19.94% | usable; auxiliary-limited |
| stealth-v6-https-1 | 2707.54 / 2869.89 | 2.954 / 2.275 | 99.99 / 81.60 | -23.01% | 3.52 / 2.95 | -16.23% | usable; auxiliary-limited |
| stealth-v6-https-8 | 2658.32 / 2773.13 | 3.009 / 2.469 | 99.98 / 85.58 | -17.95% | 3.65 / 2.95 | -19.32% | usable; auxiliary-limited |

Candidate counters: at least 99.54% of accepted original packets participate in coalesced groups per sample; maximum group 32; offload fallbacks 0. Counters include setup/warmup, not just the timed body.

### Native x64, unpaced

[Raw evidence](https://github.com/riKirsti/ProtunNext/actions/runs/42728567350/artifacts/13725126490). Kernel `7.0.0-1017-azure`; candidate `5dba6015353d20127c772ce80cd90063e6d5c6b0`.

Planned/observed samples: 96/96; invalid-data errors: 0.

| Workload | Mbps baseline / candidate | CPU ns/B baseline / candidate | CPU core % baseline / candidate | CPU change | RSS MiB baseline / candidate | RSS change | Screen |
|---|---:|---:|---:|---:|---:|---:|---|
| udp-v4-http-1 | 2146.12 / 2622.45 | 3.308 / 2.169 | 88.73 / 71.09 | -34.43% | 9.86 / 8.57 | -13.13% | usable; auxiliary-limited |
| udp-v4-http-8 | 1996.05 / 2384.43 | 3.550 / 2.472 | 88.57 / 73.68 | -30.36% | 11.40 / 8.60 | -24.58% | usable; auxiliary-limited |
| udp-v6-https-1 | 2147.95 / 2405.14 | 3.505 / 2.354 | 94.10 / 70.78 | -32.82% | 9.44 / 8.28 | -12.25% | usable; auxiliary-limited |
| udp-v6-https-8 | 1968.37 / 2169.33 | 3.799 / 2.646 | 93.46 / 71.75 | -30.34% | 10.73 / 8.50 | -20.74% | usable; auxiliary-limited |
| tcp-v4-http-1 | 2225.44 / 2692.32 | 3.262 / 2.160 | 90.75 / 72.69 | -33.79% | 9.58 / 7.48 | -21.93% | usable; auxiliary-limited |
| tcp-v4-http-8 | 2064.11 / 2475.86 | 3.493 / 2.453 | 90.11 / 75.90 | -29.77% | 11.03 / 8.05 | -27.06% | usable; auxiliary-limited |
| tcp-v6-https-1 | 2244.11 / 2474.16 | 3.454 / 2.369 | 96.89 / 73.27 | -31.41% | 9.08 / 7.89 | -13.11% | usable; auxiliary-limited |
| tcp-v6-https-8 | 2020.29 / 2232.89 | 3.755 / 2.647 | 94.83 / 73.88 | -29.51% | 10.33 / 7.47 | -27.69% | usable; auxiliary-limited |
| stealth-v4-http-1 | 2071.35 / 2506.63 | 3.552 / 2.408 | 91.96 / 75.45 | -32.20% | 6.47 / 4.88 | -24.58% | usable; auxiliary-limited |
| stealth-v4-http-8 | 1936.68 / 2256.21 | 3.830 / 2.725 | 92.72 / 76.85 | -28.85% | 8.00 / 6.37 | -20.43% | usable; auxiliary-limited |
| stealth-v6-https-1 | 2102.89 / 2270.88 | 3.705 / 2.629 | 97.40 / 74.64 | -29.04% | 6.72 / 5.13 | -23.63% | usable; auxiliary-limited |
| stealth-v6-https-8 | 1902.83 / 2060.70 | 4.018 / 2.954 | 95.58 / 76.09 | -26.48% | 7.19 / 5.23 | -27.30% | usable; auxiliary-limited |

Candidate counters: at least 99.56% of accepted original packets participate in coalesced groups per sample; maximum group 32; offload fallbacks 0. Counters include setup/warmup, not just the timed body.

### Native ARM64, unpaced

[Raw evidence](https://github.com/riKirsti/ProtunNext/actions/runs/42728567350/artifacts/13724223190). Kernel `7.0.0-1017-azure`; candidate `5dba6015353d20127c772ce80cd90063e6d5c6b0`.

Planned/observed samples: 96/96; invalid-data errors: 0.

| Workload | Mbps baseline / candidate | CPU ns/B baseline / candidate | CPU core % baseline / candidate | CPU change | RSS MiB baseline / candidate | RSS change | Screen |
|---|---:|---:|---:|---:|---:|---:|---|
| udp-v4-http-1 | 2410.20 / 3061.98 | 3.319 / 2.190 | 100.00 / 83.82 | -34.02% | 9.78 / 7.16 | -26.87% | usable; auxiliary-limited |
| udp-v4-http-8 | 2401.43 / 2891.69 | 3.330 / 2.376 | 99.97 / 85.89 | -28.65% | 10.59 / 7.50 | -29.20% | usable; auxiliary-limited |
| udp-v6-https-1 | 2609.10 / 2990.63 | 3.066 / 2.302 | 99.99 / 86.05 | -24.92% | 9.85 / 7.37 | -25.14% | usable; auxiliary-limited |
| udp-v6-https-8 | 2561.05 / 2854.64 | 3.124 / 2.505 | 100.00 / 89.38 | -19.81% | 10.85 / 7.85 | -27.66% | usable; auxiliary-limited |
| tcp-v4-http-1 | 2480.84 / 3160.24 | 3.225 / 2.108 | 100.00 / 83.28 | -34.62% | 10.01 / 7.35 | -26.60% | usable; auxiliary-limited |
| tcp-v4-http-8 | 2476.61 / 2990.50 | 3.230 / 2.298 | 99.99 / 85.92 | -28.84% | 10.35 / 6.94 | -32.90% | usable; auxiliary-limited |
| tcp-v6-https-1 | 2632.26 / 3096.45 | 3.039 / 2.210 | 100.00 / 85.54 | -27.28% | 10.35 / 6.75 | -34.75% | usable; auxiliary-limited |
| tcp-v6-https-8 | 2611.56 / 2933.33 | 3.063 / 2.380 | 100.00 / 87.25 | -22.32% | 10.09 / 7.02 | -30.46% | usable; auxiliary-limited |
| stealth-v4-http-1 | 2170.95 / 2613.11 | 3.685 / 2.554 | 100.00 / 83.43 | -30.68% | 7.80 / 4.54 | -41.85% | usable; auxiliary-limited |
| stealth-v4-http-8 | 2153.45 / 2497.78 | 3.715 / 2.747 | 99.99 / 85.78 | -26.04% | 8.15 / 5.65 | -30.69% | usable; auxiliary-limited |
| stealth-v6-https-1 | 2277.06 / 2531.03 | 3.513 / 2.663 | 100.00 / 84.24 | -24.21% | 7.22 / 4.28 | -40.66% | usable; auxiliary-limited |
| stealth-v6-https-8 | 2237.76 / 2450.28 | 3.575 / 2.876 | 100.00 / 88.07 | -19.56% | 7.22 / 4.54 | -37.21% | usable; auxiliary-limited |

Candidate counters: at least 99.55% of accepted original packets participate in coalesced groups per sample; maximum group 32; offload fallbacks 0. Counters include setup/warmup, not just the timed body.

### Native x64, 200 Mbps

[Raw evidence](https://github.com/riKirsti/ProtunNext/actions/runs/42728570056/artifacts/13724820970). Kernel `7.0.0-1017-azure`; candidate `5dba6015353d20127c772ce80cd90063e6d5c6b0`.

Planned/observed samples: 96/96; invalid-data errors: 0.

| Workload | Mbps baseline / candidate | CPU ns/B baseline / candidate | CPU core % baseline / candidate | CPU change | RSS MiB baseline / candidate | RSS change | Screen |
|---|---:|---:|---:|---:|---:|---:|---|
| udp-v4-http-1 | 200.08 / 200.08 | 2.868 / 1.471 | 7.17 / 3.68 | -48.71% | 10.66 / 7.72 | -27.55% | usable; matched load |
| udp-v4-http-8 | 200.71 / 200.73 | 2.885 / 1.721 | 7.24 / 4.32 | -40.33% | 11.62 / 8.11 | -30.20% | usable; matched load |
| udp-v6-https-1 | 200.08 / 200.08 | 2.932 / 1.531 | 7.33 / 3.83 | -47.76% | 9.79 / 7.87 | -19.62% | usable; matched load |
| udp-v6-https-8 | 200.71 / 200.73 | 3.100 / 1.789 | 7.78 / 4.49 | -42.31% | 10.28 / 8.40 | -18.30% | usable; matched load |
| tcp-v4-http-1 | 200.08 / 200.08 | 2.823 / 1.543 | 7.06 / 3.86 | -45.35% | 8.87 / 7.57 | -14.72% | usable; matched load |
| tcp-v4-http-8 | 200.71 / 200.73 | 2.852 / 1.623 | 7.16 / 4.07 | -43.08% | 11.13 / 7.78 | -30.06% | usable; matched load |
| tcp-v6-https-1 | 200.08 / 200.08 | 3.150 / 1.572 | 7.88 / 3.93 | -50.08% | 8.79 / 7.18 | -18.27% | usable; matched load |
| tcp-v6-https-8 | 200.71 / 200.73 | 3.013 / 1.715 | 7.56 / 4.30 | -43.09% | 9.51 / 7.79 | -18.07% | usable; matched load |
| stealth-v4-http-1 | 200.08 / 200.08 | 2.937 / 1.690 | 7.35 / 4.23 | -42.46% | 8.08 / 5.13 | -36.52% | usable; matched load |
| stealth-v4-http-8 | 200.71 / 200.72 | 2.960 / 1.768 | 7.43 / 4.44 | -40.26% | 8.42 / 6.39 | -24.11% | usable; matched load |
| stealth-v6-https-1 | 200.08 / 200.08 | 3.191 / 1.704 | 7.98 / 4.26 | -46.59% | 6.51 / 5.26 | -19.28% | usable; matched load |
| stealth-v6-https-8 | 200.70 / 200.72 | 3.317 / 1.937 | 8.32 / 4.86 | -41.61% | 6.78 / 5.19 | -23.55% | usable; matched load |

Candidate counters: at least 98.19% of accepted original packets participate in coalesced groups per sample; maximum group 32; offload fallbacks 0. Counters include setup/warmup, not just the timed body.

### Native ARM64, 200 Mbps

[Raw evidence](https://github.com/riKirsti/ProtunNext/actions/runs/42728570056/artifacts/13724486677). Kernel `7.0.0-1017-azure`; candidate `5dba6015353d20127c772ce80cd90063e6d5c6b0`.

Planned/observed samples: 96/96; invalid-data errors: 0.

| Workload | Mbps baseline / candidate | CPU ns/B baseline / candidate | CPU core % baseline / candidate | CPU change | RSS MiB baseline / candidate | RSS change | Screen |
|---|---:|---:|---:|---:|---:|---:|---|
| udp-v4-http-1 | 200.08 / 200.08 | 3.638 / 2.318 | 9.10 / 5.80 | -36.30% | 10.10 / 6.97 | -30.95% | usable; matched load |
| udp-v4-http-8 | 200.70 / 200.72 | 3.389 / 2.178 | 8.50 / 5.46 | -35.75% | 10.26 / 7.79 | -24.12% | usable; matched load |
| udp-v6-https-1 | 200.08 / 200.08 | 3.615 / 2.521 | 9.04 / 6.31 | -30.26% | 8.79 / 7.16 | -18.51% | usable; matched load |
| udp-v6-https-8 | 200.70 / 200.72 | 3.240 / 2.273 | 8.13 / 5.70 | -29.85% | 9.82 / 7.84 | -20.18% | usable; matched load |
| tcp-v4-http-1 | 200.08 / 200.08 | 3.559 / 2.174 | 8.90 / 5.44 | -38.92% | 9.94 / 6.59 | -33.67% | usable; matched load |
| tcp-v4-http-8 | 200.71 / 200.72 | 3.214 / 2.123 | 8.06 / 5.33 | -33.94% | 10.40 / 7.45 | -28.34% | usable; matched load |
| tcp-v6-https-1 | 200.08 / 200.08 | 3.460 / 2.362 | 8.65 / 5.91 | -31.71% | 7.98 / 6.41 | -19.73% | usable; matched load |
| tcp-v6-https-8 | 200.70 / 200.72 | 3.163 / 2.196 | 7.93 / 5.51 | -30.57% | 9.44 / 6.90 | -26.88% | usable; matched load |
| stealth-v4-http-1 | 200.07 / 200.08 | 4.010 / 2.633 | 10.03 / 6.58 | -34.36% | 7.59 / 4.22 | -44.37% | usable; matched load |
| stealth-v4-http-8 | 200.70 / 200.72 | 3.651 / 2.548 | 9.16 / 6.39 | -30.22% | 7.87 / 5.53 | -29.72% | usable; matched load |
| stealth-v6-https-1 | 200.07 / 200.08 | 3.962 / 2.852 | 9.91 / 7.13 | -28.02% | 5.41 / 4.21 | -22.07% | usable; matched load |
| stealth-v6-https-8 | 200.70 / 200.71 | 3.616 / 2.669 | 9.07 / 6.70 | -26.20% | 6.43 / 4.47 | -30.43% | usable; matched load |

Candidate counters: at least 98.19% of accepted original packets participate in coalesced groups per sample; maximum group 32; offload fallbacks 0. Counters include setup/warmup, not just the timed body.


## Preliminary and superseded evidence

The 24 focused timed downloads below use initial source `01b400c54de98bc9b5091e8263084653d4d57681`, before final copied-header erasure. All are valid; they are independent preliminary cohorts and are not pooled with the final 576. The [superseded unpaced launch](https://github.com/riKirsti/ProtunNext/actions/runs/42727833667) and [superseded matched-load launch](https://github.com/riKirsti/ProtunNext/actions/runs/42727836106) on `70b0127daac93d98e4a9d5af8f6e6b2336825b0d` were canceled when the erasure correction was added; they are excluded from completed-cohort totals.

### Preliminary Linux 6.12 x64

[Raw evidence](https://github.com/riKirsti/ProtunNext/actions/runs/42727186261/artifacts/13723182968). Kernel `6.12.115`; candidate `01b400c54de98bc9b5091e8263084653d4d57681`.

Planned/observed samples: 8/8; invalid-data errors: 0.

| Workload | Mbps baseline / candidate | CPU ns/B baseline / candidate | CPU change | RSS MiB baseline / candidate | RSS change | Screen |
|---|---:|---:|---:|---:|---:|---|
| udp-v4-http-1 | 2319.22 / 2471.91 | 3.376 / 2.297 | -31.97% | 6.36 / 3.98 | -37.43% | usable; auxiliary-limited |

Candidate counters: at least 99.87% of accepted original packets participate in coalesced groups per sample; maximum group 32; offload fallbacks 0. Counters include setup/warmup, not just the timed body.

### Preliminary native x64, 200 Mbps

[Raw evidence](https://github.com/riKirsti/ProtunNext/actions/runs/42727183205/artifacts/13722892713). Kernel `7.0.0-1017-azure`; candidate `01b400c54de98bc9b5091e8263084653d4d57681`.

Planned/observed samples: 8/8; invalid-data errors: 0.

| Workload | Mbps baseline / candidate | CPU ns/B baseline / candidate | CPU change | RSS MiB baseline / candidate | RSS change | Screen |
|---|---:|---:|---:|---:|---:|---|
| udp-v4-http-1 | 200.07 / 200.08 | 4.141 / 2.543 | -38.60% | 9.95 / 7.93 | -20.29% | usable; matched load |

Candidate counters: at least 98.19% of accepted original packets participate in coalesced groups per sample; maximum group 32; offload fallbacks 0. Counters include setup/warmup, not just the timed body.

### Preliminary native ARM64, 200 Mbps

[Raw evidence](https://github.com/riKirsti/ProtunNext/actions/runs/42727183205/artifacts/13722537959). Kernel `7.0.0-1017-azure`; candidate `01b400c54de98bc9b5091e8263084653d4d57681`.

Planned/observed samples: 8/8; invalid-data errors: 0.

| Workload | Mbps baseline / candidate | CPU ns/B baseline / candidate | CPU change | RSS MiB baseline / candidate | RSS change | Screen |
|---|---:|---:|---:|---:|---:|---|
| udp-v4-http-1 | 200.08 / 200.08 | 3.611 / 2.228 | -38.30% | 9.84 / 6.78 | -31.03% | usable; matched load |

Candidate counters: at least 98.19% of accepted original packets participate in coalesced groups per sample; maximum group 32; offload fallbacks 0. Counters include setup/warmup, not just the timed body.


## Stage boundary

Stage 3 is complete in PR #8 and ready for review. After merge, freeze that merged commit and review Stage 4 with the user before starting: current receive/crypto profiles, ordered bounded parallelism, replay/session/rekey ownership, backpressure and shutdown, fixed-auxiliary comparisons and completion criteria. Alternative backends/crypto implementations remain Stage 5; PGO/build tuning remains Stage 6; combined real-workload/platform acceptance remains Stage 7.
