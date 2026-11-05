ProtunNext Travel rerun — November 4, 2026 New York / November 5 UTC

Controlled stage started 2026-11-05T02:21:10.367859+00:00; final independent postflight completed 2026-11-05T03:22:27.432295+00:00. This is a new router rerun, separate from the earlier one-hour CI campaign.

The measurements support compact pools and in-place crypto; Linux mmsg with outer UDP offloads; and unbuffered Stealth for this Travel router and Linux x64. Keep the tested TUN aggregation and io_uring prototypes out of the production default. Do not make parallel crypto or PGO mandatory from these measurements. The router run found a real io_uring completion-overflow liveness bug and verified a minimal experimental correction. No production backend, main branch or PR was changed.

Travel is one physical two-core Cortex-A53 ARM64 router running Linux 6.12.112. io_uring was enabled and actually exercised. Native ARM64 kernel 6.18 remains unmeasured. Twelve repeated rounds on this router are not twelve independent devices. The main full-worker bulk comparisons use balanced AB/BA order; supplementary latency, oversized, memory, batch-size, TLS 1.2 and TUN cases use fixed order. Raw results retain outliers and failures.

The controlled worker uses the actual Connection API, TUN, WireGuard encryption and authenticated UDP/TCP/TLS peer. The application is IPv4 UDP request/reply; every response validates payload, sequence, length and duplication. This is not an HTTPS or NIC forwarding benchmark. Whole-process RSS includes the peer and generator, whereas worker CPU is measured for the client thread at 100 Hz. Short timings and a two-core shared system produce wide throughput spreads; CPU/byte trends are often more consistent. Percentages from separate changes are not additive.

Completed: 396 main/supplementary timed worker samples; 48 held-out PGO comparisons (24 pairs); 600 fresh Stealth starts; three concurrent 600-second protocol soaks; 72 PR-source bridge samples; 56 baseline and 56 profile-used regression test executions. These are repeated executions, not counts of unique tests. All completed successfully. Both TLS versions, UDP offloads and io_uring activation were recorded. The three simultaneous soaks each observed five handshake responses and zero invalid packets; their throughput is unsuitable for ranking because they compete for two cores. Component results include 80 UDP cases, 128 TLS rows, 39 crypto/pool rows, 36 shared-session parallel-crypto rows and eight TUN-boundary rows. Four pilot worker samples and six PGO training cases are retained separately.

The main Proton comparison contains 36 sequential tunnel sessions, 48 bulk downloads and separate cold requests. It uses the existing account/key unchanged, two distinct saved Plus endpoints (US-NY#601 and US-NY#505), interface-bound temporary routing and verified HTTPS ranges from one pinned cdn.kernel.org IPv4 address. Stealth uses 12 MB and 50 MB files, other protocols 50 MB; three rounds reverse server/configuration and file order. Successful downloads require exit 0, HTTP 206, exact length and Content-Range, and successful TLS verification. Partial transfers are failures, never throughput samples. The origin file was linux-6.12.112.tar.xz. This tests the NoLocalAgent transport path, not full account/Plus-control-plane parity.

The controlled near-MTU table uses 1,200 application bytes, window 32 and paired medians. Positive CPU/RSS percentages mean higher cost.

| Change, matched otherwise | Throughput | Worker CPU / byte | Whole-test peak RSS |
|---|---:|---:|---:|
| mmsg alone | +7.79% | +3.78% | +16.48% |
| Compact versus full pool, copy crypto | -3.72% | +1.03% | -48.23% |
| In-place versus copy crypto, compact pool | +9.32% | -10.71% | +30.50% |
| Outer GSO/GRO versus mmsg alone | -1.26% | -6.76% | +15.95% |
| io_uring128 versus mmsg, both with offloads (original) | -0.31% | +1.60% | +83.36% |
| Unbuffered versus buffered Stealth, TLS 1.3 | +3.83% | -7.34% | +2.28% |
| Unbuffered versus buffered Stealth, TLS 1.2 | +9.47% | -13.05% | +1.09% |
| Outbound software TUN aggregation screening | -5.82% | +22.49% | -8.04% |

Compact-pool initial reservation fell 95.31%; that is reserved capacity, not a 95% runtime RSS reduction. Compact pools reduce measured RSS substantially, but in-place crypto raises whole-fixture peak RSS versus copy crypto at a fixed compact pool. Keep its CPU reduction with an explicit memory budget. TCP in-place CPU/byte fell 10.65% while throughput median changed -4.35% with wide variation; do not advertise a universal throughput gain. The batch-limit screening gives no convincing case for changing the present limit 16. mmsg reduces syscall counts, but that alone does not guarantee a throughput gain.

The TUN comparison is screening only: raw samples run 1.5 seconds versus one second for aggregate samples, and generator sends change. The prototype is outbound software segmentation, not a complete bidirectional multiqueue pipeline. Its higher UDP worker CPU and lack of a clear worker benefit support keeping it disabled.

The original live results below show successful-transfer medians in Mbps, with successes/attempts. Do not rank a 2/3 or 0/3 backend as reliable. The kernel failures were four-second connection timeouts with no HTTP response, whereas io_uring failures were 25-second partial-download stalls. They are distinct observations.

| Protocol / build | US-NY#601, 50 MB | US-NY#505, 50 MB |
|---|---:|---:|
| Kernel WireGuard UDP | 108.96 (2/3) | 130.43 (2/3) |
| Optimized mmsg WireGuard UDP | 116.16 (3/3) | 125.20 (3/3) |
| Optimized WireGuard TCP | 185.69 (3/3) | 172.52 (3/3) |
| Buffered Stealth | 106.13 (3/3) | 110.33 (3/3) |
| Unbuffered Stealth | 154.21 (3/3) | 139.31 (3/3) |
| Original io_uring UDP + offloads | — (0/3) | 95.59 (2/3) |

All 24 Stealth bulk downloads completed. At 12 MB, buffered/unbuffered medians were 78.31/110.77 Mbps on #601 and 80.07/102.92 Mbps on #505. Paired unbuffered improvements were +41.46%/+20.00% at 12 MB and +32.96%/+25.41% at 50 MB (#601/#505); client-process CPU/byte fell approximately 23–25%. These results favor unbuffered on this router despite the smaller controlled-worker advantage. The previous router run had mixed 12 MB results; this rerun does not pool different-day measurements as matched pairs.

Direct 12 MB controls changed from 100.37 to 201.17 Mbps during the main WAN series. That variation, three rounds per cell, cold/session effects and shared Internet paths prevent treating WAN medians as controlled hardware speed limits. Both original kernel connection failures remain recorded. No evidence here proves a cryptographic provider caused a kernel/userspace gap.

The extra original-io_uring diagnostic separates offloads from receiver selection:

| Original UDP backend | Successful 50 MB transfers, both servers |
|---|---:|
| batch-offload | 4/4 |
| uring-no-offload | 0/4 |
| uring-offload | 0/4 |

The router kernel fdinfo captured an empty visible CQ, no active receive poll, and `CqOverflowList: user_data=1, res=-105, flags=0`. The terminal ENOBUFS CQE lacked F_MORE and was trapped in overflow. After draining the visible CQ, the receiver still believed its request remained active and never flushed the terminal event or rearmed. This is a real liveness defect in the expanded experimental 128-buffer receiver. It is not evidence that io_uring itself is disabled or that every earlier download failure had this cause.

A burst regression explicitly requires real CQ overflow. Original source fails after 128 completions, enobufs=0, submissions=1. The minimal experimental correction checks CQ_OVERFLOW after draining the visible queue and invokes the crate submit path, which uses nonblocking GETEVENTS to flush pending completions. The corrected test records 353 completions, two ENOBUFS events and three submissions, then validates a new packet. All three corrected io_uring tests passed, including IPv4/IPv6 maximum payload, GRO short tail and cancellation. The CQ size and buffer count were unchanged to isolate the fix.

A separate two-round corrected-WAN series uses the same endpoints/origin, six seconds of settling, and records kernel handshakes before bulk. It is a later cohort, not a matched fix-speed comparison against the earlier series:

| UDP backend, corrected experiment | US-NY#601, 50 MB | US-NY#505, 50 MB |
|---|---:|---:|
| Kernel control | 68.24 (1/2) | 178.59 (1/2) |
| mmsg + offloads control | 131.97 (2/2) | 117.97 (2/2) |
| Corrected io_uring without offloads | 73.82 (2/2) | 109.64 (2/2) |
| Corrected io_uring with offloads | 151.66 (2/2) | 154.54 (2/2) |

The correction is experimental and was not deployed or merged. Even when downloads finish, original controlled io_uring performance is essentially tied on Travel with 83% greater whole-test peak RSS; the earlier x64 LTS tests measured 13.55–17.30% lower throughput. Eight corrected WAN downloads are not a long-duration or all-platform acceptance test. Keep mmsg as the Linux default; preserve the bug regression if io_uring development continues.

The last kernel-only check allowed quiet time and retried a small readiness request before the bulk file. Both 50 MB downloads completed: #601 56.94 Mbps and #505 204.90 Mbps. #505 needed a second readiness attempt. This demonstrates that the connection failures were recoverable in this run; their underlying cause is not established. Do not combine these later one-shot numbers with the original balanced-series medians or claim a provider defect. Handshake/readiness logs and every failed attempt remain in evidence/kernel-readiness.json.

Shared-session crypto primitive throughput at 1,280 bytes improved 74.99% with two workers and 85.98% with four; four threads oversubscribe this two-core router. Nonce/replay/tag checks passed, but no production bounded parallel packet queue, rekey/cancellation/order pipeline was benchmarked. This supports a prototype, not enabling production parallel crypto.

Travel-native PGO used instrumented training at 256/1,000 bytes and held-out 64/1,200-byte tests with four paired rounds per protocol/size. Matching Rust LLVM tools merged the profiles. At 1,200 bytes, UDP/TCP/Stealth throughput medians were -2.50%/+6.79%/-3.56%; client-worker CPU/byte changed -0.57%/-1.14%/-8.40%. At 64 bytes, throughput changed -7.55%/-1.86%/+10.90%. All 56 profile-used regression executions passed. The profile also optimizes the peer/generator, and the control plane was not representatively trained. PGO is not justified as a universal mandatory build.

The source bridge tests frozen PR #2 f53b963 with only a benchmark entry point added. Against frozen experiment 83f0252 with equivalent selected options, production median throughput differences were UDP -0.83%, TCP -1.41%, Stealth -2.90%, across 12 pairs each. Wide individual differences are retained. All 72 samples passed. This bridge supports relevance of the fixture to the PR hot path; it does not establish feature-parity acceptance.

General recommendation: retain compact buffers/in-place crypto with measured memory limits and the syscall-batching implementation; select offloads and TLS using complete-worker evidence. Keep both TLS adapters until supported-platform worker acceptance permits simplification. Reject the tested software TUN aggregation and original io_uring receiver as production defaults. Prototype parallel crypto only with bounded queues, session/key-epoch correctness, latency, rekey and cancellation tests. Make PGO an optional, architecture/workload-trained build rather than a release requirement.

| Target | Recommendation supported by current evidence | Remaining acceptance boundary |
|---|---|---|
| Linux x64, 6.12 / 6.18 LTS | mmsg + outer GSO/GRO; unbuffered Stealth candidate; compact/in-place | Prior actual-LTS worker results favor unbuffered by 48–51%; shipping integration, sustained NIC/forwarding workloads and complete feature regressions still required |
| Travel ARM64, 6.12.112 | mmsg + offloads; unbuffered Stealth favored for this device; keep batch limit 16 | WAN sample size is small; monitor runtime RSS and sparse-traffic latency |
| Other Linux ARM64, including 6.18 LTS | mmsg baseline; measure TLS per device/workload | Previous Neoverse-N2 worker lost 3.56% with unbuffered; Travel does not prove all ARM64 benefits. No native ARM64 6.18 acceptance yet |
| macOS 26 / 27, ARM64 | Buffered TLS remains the candidate | Prior M1 VM framing tests lost about 20–30% with unbuffered; complete Network Extension worker test remains |
| iOS / iPadOS 26 / 27, ARM64 | Keep buffered candidate pending device data | macOS components are not iOS/iPadOS device or Network Extension acceptance |
| Windows 11 25H2 / 26H2, x64 | Keep buffered pending representative worker comparison; use Windows packet APIs | Previous 26H2 framing comparisons were noisy/inconclusive; 25H2 runtime and full worker remain unmeasured |
| Android 16 / 17, 16 KB target | Keep current adapter until native application measurements; use protected/bound sockets | No Android native app performance acceptance; ARM64 Linux results are not VpnService results |

These are performance decisions for tested candidates, not a claim that every platform, account operation, Plus feature, network transition or deployment scenario has passed. Do not erase fallback/cancellation/MTU correctness merely because older OS versions are dropped. Kernel TLS remains outside the requested implementation scope.

All router stages have completed their guarded cleanup. Saved configuration, routes/rules/default path, packages, socket limits, interfaces, boot/netifd identity and Direct mode matched their snapshots. Only our own temporary stage files/interfaces/rules were removed. No installed software was removed, no key was rotated, and Home was not accessed. Independent final checks are in postflight.json. Local evidence/build artifacts remain on the laptop.

Official contracts used to check the diagnosis and methodology: [liburing CQ overflow](https://github.com/axboe/liburing/blob/master/man/io_uring_cq_has_overflow.3), [liburing event flushing](https://github.com/axboe/liburing/blob/master/man/io_uring_get_events.3), [multishot recvmsg and F_MORE](https://github.com/axboe/liburing/blob/master/man/io_uring_prep_recvmsg.3), [Linux 6.12 fdinfo implementation](https://github.com/torvalds/linux/blob/v6.12/io_uring/fdinfo.c), [Linux segmentation offloads](https://www.kernel.org/doc/html/v6.12/networking/segmentation-offloads.html), [Rust PGO procedure](https://doc.rust-lang.org/rustc/profile-guided-optimization.html). These documents establish API behavior; measured performance comes from the attached raw results, not the documentation.

Evidence: analysis.json and PAIRED-RESULTS.md contain all controlled per-round comparisons; FINAL-ANALYSIS.json summarizes WAN, regressions, bridge and cleanup; evidence/*.json retains raw stdout, counters, failures, source/binary fingerprints and timing. `uring-before.rs`, `uring-after.rs`, `overflow-regression.rs` and the patch retain the exact experimental receiver change. Previous CI evidence remains in [decision-hour-20261104.md](decision-hour-20261104.md) and is not silently replaced by this device rerun.

Repository copy: all evidence/ and reproduction/ paths above refer to files inside [the raw Travel archive](travel-decision-20261105.zip). The [summary JSON](travel-decision-20261105.json) is also available separately. Historical recommendations describe the tested revision; the [current compile-time selection](../tls-selection-20261105.md) supersedes the old global TLS policy.
