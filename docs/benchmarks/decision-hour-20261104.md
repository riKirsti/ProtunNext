ProtunNext: one-hour performance decision report

Campaign start: 2026-11-05 01:00:52 UTC (November 4, 20:00:52 New York). Hard deadline: 02:00:52 UTC. This report distinguishes completed experiments from release gates that remain unproved. Neither router was accessed or changed. Main remains `86a3f0b`; PR #2 remains `f53b963`, unmerged. All new code is on `experiments/decision-hour-20261104`.

The decision supported by these measurements is to retain compact pools and in-place crypto, retain Linux mmsg with outer UDP offloads, use unbuffered TLS as the Linux x64 candidate, and avoid imposing unbuffered TLS globally. Do not restore this io_uring or software TUN-segmentation prototype as the default. Parallel crypto warrants an actual packet-worker prototype; its production decision is still open. The PGO results do not justify making PGO mandatory.

**What actually ran**
The main fixture calls the real Connection API and packet worker, uses a real isolated TUN, performs authenticated WireGuard handshakes and payload encryption/decryption, and transports packets over UDP, framed TCP or TLS. A controlled peer decrypts, reverses an IPv4/UDP packet and returns it through WireGuard. Every response checks length, sequence, duplication and payload. Reported Mbps counts returned application payload once; this is a request/reply workload, not an Internet download or aggregate bidirectional line rate. The peer and generator run alongside the client; their CPU and whole-process memory are recorded. This is not a forwarding/NIC/HTTPS acceptance test.

The principal worker matrix has **1,188 successful timed samples and 5,400 successful fresh-start cycles**. Each decision-critical cell has four balanced AB/BA rounds on three fresh runner allocations: 12 matched pairs. x64 guests actually booted **Linux 6.12.115 and 6.18.60**, exposed four CPUs, and used TSC clocks; they refused to benchmark with fewer CPUs or a jiffy clock. ARM64 ran natively on four Neoverse-N2 CPUs under kernel 7.0.0-1017-azure. That ARM host has no /dev/kvm: it is not ARM64 LTS-kernel performance acceptance.

The component rerun contains 160 UDP cases, 256 TLS adapter/framing samples, 48 crypto round trips, 24 pool measurements, 16 TUN-boundary samples and 72 shared-session crypto samples on the two actual LTS kernels. Additional native x64/ARM64 jobs contain 216 shared-session crypto samples, 36 PGO-training samples and 288 held-out baseline/profile-used worker samples. All six native PGO jobs passed 56 regression checks per baseline and profile-used build: **672 successful test executions**. This count includes repeats, not 672 distinct tests.

Activation was checked, not assumed: the worker matrix recorded **6,125,063 io_uring completions, 14,751,247 GRO datagrams and 15,227,147 GSO datagrams**, with zero recorded kernel fallbacks and zero invalid packets. The received payload was validated in every timed sample. Cold starts succeeded 900 times per TLS adapter in each of the three environments. This does not establish that the earlier live Proton HTTPS failures are fixed.

**Complete worker results**
Values below are paired median throughput changes at 1,200 application bytes and window 32. Each column is paired within its own machines; results from different architectures are not compared as if hardware were identical. Percentages from separate changes must not be added.

| Change, otherwise matched | x64 Linux 6.12 | x64 Linux 6.18 | Native ARM64 kernel 7.0 |
|---|---:|---:|---:|
| mmsg alone, offloads off | -1.66% | +1.16% | -0.22% |
| In-place crypto, compact pool fixed | +7.75% | -0.09% | +6.61% |
| Outer UDP GSO/GRO, mmsg fixed | +2.59% | +10.61% | +7.96% |
| io_uring + offloads versus mmsg + same offloads | -17.30% | -13.55% | -4.56% |
| Unbuffered versus buffered Stealth, TLS 1.3 | +50.78% | +48.36% | -3.56% |
| Unbuffered versus buffered Stealth, TLS 1.2 | +48.42% | +45.58% | -3.17% |
| TUN outbound aggregate/segmentation prototype | -13.67% | -12.46% | -8.87% |

The io_uring result reverses the optimistic socket-only conclusion: its complete-worker throughput fell 17.30%, 13.55% and 4.56%, while client-worker CPU/byte rose 37.79%, 30.58% and 5.61%. Whole-fixture peak RSS rose roughly 82–89%. It was enabled and exercised on both supported LTS kernels. This is a rejection of the tested metadata-aware receive implementation/configuration, not a claim that io_uring cannot be fast. It covers UDP receive; sendmmsg still transmits. No io_uring TCP/TLS/TUN implementation was benchmarked.

Batching really reduced send calls per datagram from 1.0 to approximately 0.0625, but batching alone did not establish a complete-worker throughput win. Batch limits 1/8/16/32 showed no convincing reason to replace the existing limit 16; do not describe 16 as a proved universal optimum. Outer UDP offloads have better evidence: worker CPU/byte changes were -9.32%, -16.74% and -13.48%. The 6.12 throughput interval remains wide; the native ARM and 6.18 gains are clearer.

Compact-pool reservation remains 8,659,068 → 405,758 bytes, **95.31% less reserved capacity**. This is not a 95% RSS reduction or throughput gain. In the copy-crypto UDP ablation, measured whole-fixture peak RSS fell approximately 35.10%, 33.01% and 41.55%. In-place/compact memory ablations, oversized 2,048-byte application packets and all three transports are in the raw results. Whole-fixture RSS includes the controlled peer. There is no new claim of zero whole-worker allocation/copy/wipe traffic.

The TUN prototype actually reduced read syscalls and separately authenticated its segments; it did not improve this worker workload. It handles outbound software segmentation, not a complete bidirectional batching/multiqueue pipeline. Its raw comparison uses 1.5-second samples versus 1-second aggregate samples and changes generator sends from many to one. It is useful screening evidence, not a clean fixed-offered-load TUN-only percentage. Keep this prototype out of the default.

**TLS across platforms**
Linux x64 supports unbuffered TLS as the performance candidate: Stealth TLS 1.3 gained 50.78%/48.36%, and worker CPU/byte fell 35.67%/33.66%. Native Linux ARM64 instead lost 3.56% and used 6.68% more worker CPU/byte. Sparse 64-byte Stealth p99 changed about +0.60%, +1.82% and +0.57%; there was no corresponding sparse-latency benefit. Both TLS 1.2 and 1.3 were negotiated and recorded; the main AES suite was AES-256-GCM. These results do not cover a separate negotiated ChaCha TLS suite.

Native ARM64 Apple runners executed on **macOS 26.6.2** and **macOS 27.0.1**, three runner allocations per version. These are Apple M1 virtual machines executing ARM64 instructions, not iOS/iPadOS Network Extension tests. Their 576 generic adapter and 192 framing samples all completed. The real-framing download comparison gave:

| OS | TLS 1.3 / 1,280-byte frames | TLS 1.3 / 16,352-byte frames |
|---|---:|---:|
| macOS 26.6.2 | -25.77% | -29.91% |
| macOS 27.0.1 | -20.43% | -24.86% |

This blocks an unconditional unbuffered choice for Apple. It is adapter/framing evidence, not a full VPN worker throughput result.

The laptop reports Windows 11 26H2, build 26300.9818. Three repeated processes on this one laptop produced 96 large framing samples, 1 GiB per sample, yielding 12 pairs per cell. Unbuffered TLS 1.3 medians were -7.15% at 1,280 bytes and -4.25% at 16,352 bytes; intervals crossed zero and individual pairs varied considerably. The earlier -36.51% framing point estimate did not reproduce at the same magnitude. A 64 MiB pilot even changed the ranking. Treat Windows selection as inconclusive, not a fixed regression or a proved win. The additional generic adapter rerun is retained separately. The raw fixture's old scope string says Linux; Windows environment records and its GetThreadTimes implementation identify the actual execution platform.

Recommendation: keep buffered TLS as the simpler choice for Apple and as the leading Linux ARM64 candidate; retain unbuffered for Linux x64. Windows needs a steadier representative worker comparison. Android has no new native runtime performance measurement. Do not convert component measurements into a single worldwide default. No TLS production selection was changed by this campaign.

**Parallel crypto, PGO and the actual PR head**
Parallel crypto now uses one shared encryption/decryption Session pair, atomic nonce allocation, a real mutex-protected replay commit and bounded 128-packet ordering epochs. Separate correctness runs checked nonce uniqueness, payload, bad-tag non-commit, replay rejection and wrong-epoch key rejection. At 1,280 bytes, 2/4 crypto workers gained **76.49%/98.99% on native x64** and **77.07%/212.82% on native ARM64** over one worker. These are 18 pairs per cell across three runners. They are primitive gains, not VPN throughput gains: no production parallel queue, ordered TCP/TLS drain, rekey/cancellation pipeline or latency budget was implemented. Therefore production parallel crypto remains an open gate.

PGO followed the documented generate → representative train → matching llvm-profdata merge → profile-use sequence. Training used UDP/TCP/Stealth at 256/1,000 bytes; held-out tests used 64/1,200 bytes with balanced build order and separate native x64/ARM64 profiles. Near-MTU throughput medians were x64 UDP +6.18%, TCP +6.07%, Stealth -2.09%; ARM64 +0.40%, +2.49%, +1.43%. Most bulk x64 intervals crossed zero; CPU/byte savings were below the proposed 10% gate. PGO also optimizes the fixture's peer and generator, so its throughput effect cannot be attributed entirely to the client. Validation ran native kernel 7.0, not both LTS kernels. Do not make PGO a release requirement from these results. Binary/profile SHA-256 values and successful profile-used regressions are retained.

The production bridge archived PR #2 head **f53b963** and added only the benchmark entry point/controlled peer, leaving its production packet-path source intact. Against the frozen reference with equivalent selected options, all 144 bridge samples delivered correct data (12 pairs per transport/architecture). The PR median changes were:

| Native architecture | UDP | TCP | Stealth |
|---|---:|---:|---:|
| x86_64 | -3.72% | -2.69% | +1.18% |
| aarch64 | +0.28% | +1.92% | -0.07% |

The simplification itself has no large established speed gain. The x64 UDP interval was [-5.32%, -1.09%], so it should not be described as exact performance equivalence. This bridge is account-free; it does not retest all control-plane/Plus/platform functionality.

**Failed attempts and corrections**
Earlier failed attempts remain in the archive and are excluded from the principal statistics: missing BINFMT_SCRIPT/ELF-init support; QEMU staying alive after a successful guest halted; incomplete CPU discovery/coarse guest clocks; a controlled TLS peer treating expected shutdown EOF as fatal; a TUN assertion mistakenly using logical packet counts instead of read syscalls; and a release test build with RNG trait mismatches. Static ELF init, explicit success markers, ACPI/SMP/clock checks, correct shutdown handling, actual syscall instrumentation and fresh isolated unwind test builds allowed successful reruns. The precise compiler cause of the trait-mismatch build was not established; successful rebuilds are not a compiler-root-cause diagnosis.

The experimental recvmsg implementation was also corrected to put its 16-byte prefix and 24-byte control area inside existing packet headroom. This preserves full payload capacity and the normal packet-pool contract. Maximum IPv4/IPv6 packets, empty packets, GRO with a short final segment, buffer recycling and bounded idle cancellation passed. None of these experimental changes was deployed to a router or merged into production.

**Gates that remain unproved**
- Native ARM64 Linux 6.12/6.18 performance; ARM kernel 7.0 is explicitly supplementary.
- Full Windows 25H2/26H2, Android 16/17, macOS/iOS/iPadOS 26/27 packet-worker/device acceptance. The new Apple and Windows results cover TLS components, not all platform bindings.
- Actual parallel-crypto packet-worker integration and its rekey, ordered output, cancellation and p99 behavior.
- Shipping-binary PGO with fixed independent peer/generator, representative control-plane training and held-out validation on both LTS kernels.
- Physical NIC/forwarding, inner TCP/HTTPS traffic, loss/RTT scenarios, broad MTU/IPv6/flow/direction matrices and long soaks per finalist on every target kernel. The main worker workload is IPv4/UDP echo; IPv6 boundary correctness is separately tested.
- The previous live Proton failures and real Plus-account acceptance. Successful account-free cold starts do not explain or repair those failures.

Consequently this is a completed bounded experiment campaign, not completion of every release gate in the earlier audit. Its default-selection recommendations are limited to the tested code, hardware and workloads.

**Reproduction and sources**
[Worker matrix](https://github.com/riKirsti/ProtunNext/actions/runs/41949204026), [actual LTS components](https://github.com/riKirsti/ProtunNext/actions/runs/41949203991), [native PGO/correctness](https://github.com/riKirsti/ProtunNext/actions/runs/41949032738), [PR-head bridge](https://github.com/riKirsti/ProtunNext/actions/runs/41950384745), [Apple TLS](https://github.com/riKirsti/ProtunNext/actions/runs/41950699980), [ten-minute soak](https://github.com/riKirsti/ProtunNext/actions/runs/41950818166). The experiment branch contains the harness, build workflows, parser and this report generator. Frozen reference: `83f025231535a3ef0731123988e9dae8621cef4a`; principal worker/components experiment: `5d410819b71da3c700d64a8f0a67f24345cc53f2`; PGO: `19a1283cf58ce9a7995495cf148456690982b39a`; PR bridge: `805710fcd25528d5e9b9b0167f2a977561d7f38d`.

Primary mechanics were checked against [liburing recvmsg multishot](https://raw.githubusercontent.com/axboe/liburing/master/man/io_uring_prep_recvmsg.3), [Linux segmentation-offload documentation](https://docs.kernel.org/networking/segmentation-offloads.html), [Linux x86 Kconfig](https://raw.githubusercontent.com/torvalds/linux/v6.12/arch/x86/Kconfig), [rustc PGO documentation](https://doc.rust-lang.org/rustc/profile-guided-optimization.html), [WireGuard queueing/parallel design](https://www.wireguard.com/papers/wireguard-netdev22.pdf), the exact cached rustls 0.23.46 stream implementation, and the vendored Proton Session/nonce/replay source. Those references verify contracts and implementation choices; the performance numbers come from the attached executions.

`analysis.json` preserves parsed samples, exact per-artifact pairs, exploratory hierarchical bootstrap intervals, environments and failure lines. `PAIRED-RESULTS.md` gives the complete comparison table. Confidence intervals resample runner allocations and within-runner pairs; three runners are still a small hardware population. Windows repetitions share one laptop. CPU/byte was measured at each configuration’s achieved load, not at one fixed offered load. ARM matrix artifacts identify source through the GitHub commit but lack their own binary hash; LTS, PGO, Apple and bridge artifacts include hashes. ZIP manifests include artifact IDs and archive SHA-256 values. Do not pool old failed cohorts, add optimization percentages or interpret recv-call counters as all io_uring syscalls.

**Ten-minute soak outcomes**

| Environment | Transport | Delivered GiB | Handshake responses | Outcome |
|---|---|---:|---:|---|
| aarch64 / 7.0.0-1017-azure | stealth | 49.64 | 5 | 600.00s, exact payload checks passed |
| aarch64 / 7.0.0-1017-azure | tcp | 76.40 | 5 | 600.00s, exact payload checks passed |
| aarch64 / 7.0.0-1017-azure | udp | 73.79 | 5 | 600.00s, exact payload checks passed |
| x86_64 / 7.0.0-1017-azure | stealth | 48.59 | 5 | 600.00s, exact payload checks passed |
| x86_64 / 7.0.0-1017-azure | tcp | 69.03 | 5 | 600.00s, exact payload checks passed |
| x86_64 / 7.0.0-1017-azure | udp | 104.78 | 5 | 600.00s, exact payload checks passed |
