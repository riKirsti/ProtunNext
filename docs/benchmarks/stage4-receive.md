# Stage 4: bounded parallel receive/decryption

**Selection:** preserve the PR #8 serial production receiver. The bounded
batch/barrier prototype and its placement follow-up pass correctness checks but
have not justified their scheduling/CPU/memory costs. All 248 normal-release timed downloads are valid; the 24 separate diagnostic
downloads also verify. No production speedup is claimed. The complete tested
implementation is retained as an [explicit archive-only experiment](../../experiments/receive/README.md),
not enabled in normal builds. This rejects this implementation, not multicore
receive in general or unmeasured architectures.
Baseline: merged PR #8, `a44d520c337611f59d4c7186971919d57de36df8`.

## Candidate and safety boundary

The following describes the archived prototype, not the retained production path.

One helper authenticates an owned ciphertext prefix while the connection owner
authenticates the suffix. Only already available consecutive data messages are
batched: at most 32 originals, with batches below eight staying serial. The
existing packet pool owns every buffer; metadata vectors are reused. Channels
hold at most one outstanding batch. No payload copying or new crypto algorithm
is introduced. Linux can use the helper when its allowed affinity has two CPUs;
other production platforms retain serial delivery pending measurements.

Immutable receiving keys are leased through Arc; mutable tunnel state never
leaves the owner. Authenticated results own their buffers until ordered owner
commit. Commit rechecks key-instance identity and replay state before updating
timers, current session, IP validation/statistics or delivery. A failed tag cannot
consume a replay counter; concurrent duplicates are committed at most once.
Control messages end a group. Capacity reservations keep all results inside the
same owner processing budget, before timers, interface replacement and shutdown.
Both channel directions disconnect before helper join; abandoned packets retain
the existing erasing Drop. Session API tests cover deferred commit and stale-key
rejection; pipeline tests cover ordering, tamper/replay, real handshake barriers,
pool pressure and blocked-completion shutdown.

The extension remains within the existing declared vendored in-place patch
surface. It reuses the pinned ring AEAD; its [immutable key API](https://docs.rs/ring/0.17.14/ring/aead/struct.LessSafeKey.html)
supports shared immutable access. No session lock or unsafe sharing of the
single-owner protocol object is added.

## Fair comparison

Both baseline and candidate use negotiated vnet (`tun_mode=vnet`) and the same
two allowed client CPUs. GitHub four-CPU fixtures pin the fixed baseline peer
and its HTTP server to CPU 0, load to CPU 1, and clients to CPUs 2/3. Shared
auxiliary CPU cost is screened in aggregate; no independent-headroom assumption
or maximum-capacity inference is made from helper-limited tests. Actual client
affinity must match the recorded allocation. Candidate counters must prove the
helper and owner both authenticate batch slots. Same-binary controls and balanced
order, verified payloads, immutable identities and failure retention remain.

The workload tables below report absolute client CPU %, RSS MiB and verified
Mbps. The [physical Travel comparison](stage4-travel.md) covers all three
transports with cleanup and independent restoration checks. Separate
frame-pointer profiles do not supply speed estimates.

No candidate is selected until whole-download measurements justify its scheduling,
memory and maintenance cost. Multicore primitive throughput alone is insufficient.

## Initial screening and placement follow-up

Initial normalized production source `7da4431e14d70ba955f2fe8de615efb9f486160f`
(the following `5ba2066` commit preserves upstream CRLF only) passes all platform
checks and 24 focused verified downloads. It does not justify promotion: UDP
IPv4 single-flow throughput on 6.12 is 1906.41/1564.93 Mbps (-17.91%), client
CPU/byte +91.15%; at 200 Mbps, native x64 CPU/byte rises 43.73% and ARM64 24.89%.
These are separate cohorts, not pooled results. Both paced cells pass controls;
the unpaced cell is helper-limited. All exercise real parallel authentication.

The follow-up keeps the same batch bounds, ownership and crypto, but puts the
owner and helper on distinct CPUs within the caller's existing allowance.
Affinity changes affect only owned threads and end with their lifetimes; no
host/NIC setting changes. Startup failure retains serial delivery. Recorded
thread CPU IDs must cover exactly the two allowed client CPUs. This tests whether
thread migration/handoff placement contributes to the initial regression; it
does not assume that a scheduling change fixes it. Initial failures are retained
and the follow-up requires independent measurements and Travel validation.

## Final selection and coverage

The pinned follow-up is `543675241b948a6d3b8c656bb39feb7450b9470a`.
At fixed 200 Mbps on native x64, ten of twelve cells pass all screens; their
CPU/byte increases 26.56–48.81% and RSS increases 3.62–14.04%. TCP IPv4/one-flow
and Stealth IPv4/one-flow are inconclusive: baseline A/A CPU/byte drift is
+5.0770% and +5.0627%, narrowly above the existing 5% screen. Their timing
screens pass, and their original values remain below rather than being discarded.
On native ARM64 all twelve cells pass; CPU/byte increases 20.46–27.95%, with
mostly higher RSS (one cell is effectively unchanged at -0.07%). Fixed-rate
completion is not evidence of higher maximum download capacity.

Focused pinned 6.12 UDP throughput falls 39.85% while CPU/byte rises 107.47%.
Focused 6.18 UDP throughput rises 5.98%, but CPU/byte rises 42.13%; both unpaced
cohorts are auxiliary-limited, so neither establishes a client capacity ranking.
Separate VM allocations also prevent attributing differences to kernel versions.
The independent focused native follow-ups increase CPU/byte 53.99% on x64 and
30.55% on ARM64. Different cohorts are never pooled into an effect estimate.

The twelve balanced Travel downloads all verify. Observed throughput is lower
for the prototype on UDP, TCP and Stealth; the [Travel report](stage4-travel.md)
retains WAN controls, resource costs and limitations. The serial baseline tested
there is the final selected production code: final `src/` and `vendor/` match
merged PR #8 exactly. No runtime feature, helper thread, affinity setting,
cryptographic implementation, dependency, router change or release ships here.
The builder-only patch and stricter measurement controls are retained.

Coverage is deliberately explicit. The full IPv4 HTTP / IPv6 HTTPS × one/eight
flows × UDP/TCP/Stealth performance matrix is measured on native Linux x64 and
ARM64 at 200 Mbps (192 verified downloads; 22/24 cells qualified). LTS 6.12 and
6.18 performance tests in this stage cover focused UDP IPv4 HTTP/one-flow only;
cross-platform regression jobs cover additional functional paths. Windows,
Apple and Android are **build/contract checked, performance-unmeasured**.
No full LTS transport matrix, maximum-capacity win, total-machine energy result,
loaded-latency improvement, long-duration stability or universal multicore
conclusion is claimed. The 248 timed downloads include initial and pinned
screening cohorts; the 24 diagnostic downloads are excluded from these totals.

## All normal-release workload results

Each row is a geometric-mean baseline / tested prototype comparison from the
recorded balanced samples. CPU % means 100% of one logical CPU; client process
CPU time includes both owned threads. RSS is peak process resident memory,
excluding kernel/peer/load. CPU ns/B reports cost per verified payload byte.
Usable means control/timing/identity checks passed, not unlimited auxiliary
headroom. All unpaced rows below are helper-limited and cannot rank capacity.
The fixed-rate rows use 128 MiB payloads; unpaced rows use exactly 2 GiB after
warmup. Neither compiler-instrumented nor frame-pointer results enter these
speed estimates. Every individual sample, CPU layout, compiler, binary hash,
source/archive/runtime/effective hash and full precision value is retained in
the linked artifacts and [identity/analysis supplement](stage4-gh-evidence.md).

### Initial Linux 6.12 x64 unpaced

[Actions run 42755533441](https://github.com/riKirsti/ProtunNext/actions/runs/42755533441), [artifact 13735602041](https://github.com/riKirsti/ProtunNext/actions/runs/42755533441/artifacts/13735602041); 8 verified timed downloads.

| Workload | CPU core % baseline / prototype | Peak RSS MiB baseline / prototype | Mbps baseline / prototype | CPU ns/B baseline / prototype | CPU cost change | Quality |
|---|---:|---:|---:|---:|---:|---|
| udp-v4-http-1 | 68.66 / 107.73 | 4.02 / 4.25 | 1906.41 / 1564.93 | 2.88 / 5.51 | +91.15% | usable; helper-limited |

### Initial native x64 200 Mbps

[Actions run 42755530103](https://github.com/riKirsti/ProtunNext/actions/runs/42755530103), [artifact 13736025380](https://github.com/riKirsti/ProtunNext/actions/runs/42755530103/artifacts/13736025380); 8 verified timed downloads.

| Workload | CPU core % baseline / prototype | Peak RSS MiB baseline / prototype | Mbps baseline / prototype | CPU ns/B baseline / prototype | CPU cost change | Quality |
|---|---:|---:|---:|---:|---:|---|
| udp-v4-http-1 | 4.37 / 6.28 | 8.04 / 8.24 | 200.08 / 200.08 | 1.75 / 2.51 | +43.73% | usable |

### Initial native ARM64 200 Mbps

[Actions run 42755530103](https://github.com/riKirsti/ProtunNext/actions/runs/42755530103), [artifact 13736015406](https://github.com/riKirsti/ProtunNext/actions/runs/42755530103/artifacts/13736015406); 8 verified timed downloads.

| Workload | CPU core % baseline / prototype | Peak RSS MiB baseline / prototype | Mbps baseline / prototype | CPU ns/B baseline / prototype | CPU cost change | Quality |
|---|---:|---:|---:|---:|---:|---|
| udp-v4-http-1 | 5.76 / 7.20 | 7.10 / 7.59 | 200.08 / 200.08 | 2.30 / 2.88 | +24.89% | usable |

### Pinned Linux 6.12 x64 unpaced

[Actions run 43150064916](https://github.com/riKirsti/ProtunNext/actions/runs/43150064916), [artifact 13914365297](https://github.com/riKirsti/ProtunNext/actions/runs/43150064916/artifacts/13914365297); 8 verified timed downloads.

| Workload | CPU core % baseline / prototype | Peak RSS MiB baseline / prototype | Mbps baseline / prototype | CPU ns/B baseline / prototype | CPU cost change | Quality |
|---|---:|---:|---:|---:|---:|---|
| udp-v4-http-1 | 88.49 / 110.43 | 3.87 / 4.17 | 4020.70 / 2418.32 | 1.76 / 3.65 | +107.47% | usable; helper-limited |

### Pinned Linux 6.18 x64 unpaced

[Actions run 43150503013](https://github.com/riKirsti/ProtunNext/actions/runs/43150503013), [artifact 13913546959](https://github.com/riKirsti/ProtunNext/actions/runs/43150503013/artifacts/13913546959); 8 verified timed downloads.

| Workload | CPU core % baseline / prototype | Peak RSS MiB baseline / prototype | Mbps baseline / prototype | CPU ns/B baseline / prototype | CPU cost change | Quality |
|---|---:|---:|---:|---:|---:|---|
| udp-v4-http-1 | 69.86 / 105.23 | 4.05 / 4.21 | 3737.04 / 3960.68 | 1.50 / 2.13 | +42.13% | usable; helper-limited |

### Pinned native x64 focused 200 Mbps

[Actions run 43150062345](https://github.com/riKirsti/ProtunNext/actions/runs/43150062345), [artifact 13913601617](https://github.com/riKirsti/ProtunNext/actions/runs/43150062345/artifacts/13913601617); 8 verified timed downloads.

| Workload | CPU core % baseline / prototype | Peak RSS MiB baseline / prototype | Mbps baseline / prototype | CPU ns/B baseline / prototype | CPU cost change | Quality |
|---|---:|---:|---:|---:|---:|---|
| udp-v4-http-1 | 6.54 / 10.08 | 8.02 / 7.94 | 200.07 / 200.07 | 2.62 / 4.03 | +53.99% | usable |

### Pinned native ARM64 focused 200 Mbps

[Actions run 43150062345](https://github.com/riKirsti/ProtunNext/actions/runs/43150062345), [artifact 13913237997](https://github.com/riKirsti/ProtunNext/actions/runs/43150062345/artifacts/13913237997); 8 verified timed downloads.

| Workload | CPU core % baseline / prototype | Peak RSS MiB baseline / prototype | Mbps baseline / prototype | CPU ns/B baseline / prototype | CPU cost change | Quality |
|---|---:|---:|---:|---:|---:|---|
| udp-v4-http-1 | 5.81 / 7.59 | 7.54 / 7.93 | 200.08 / 200.07 | 2.32 / 3.03 | +30.55% | usable |

### Pinned native x64 full 200 Mbps

[Actions run 43150498346](https://github.com/riKirsti/ProtunNext/actions/runs/43150498346), [artifact 13914580863](https://github.com/riKirsti/ProtunNext/actions/runs/43150498346/artifacts/13914580863); 96 verified timed downloads.

| Workload | CPU core % baseline / prototype | Peak RSS MiB baseline / prototype | Mbps baseline / prototype | CPU ns/B baseline / prototype | CPU cost change | Quality |
|---|---:|---:|---:|---:|---:|---|
| udp-v4-http-1 | 3.93 / 5.20 | 7.47 / 8.43 | 200.08 / 200.08 | 1.57 / 2.08 | +32.50% | usable |
| udp-v4-http-8 | 4.18 / 5.79 | 8.29 / 8.84 | 200.70 / 200.70 | 1.67 / 2.31 | +38.42% | usable |
| udp-v6-https-1 | 3.64 / 5.30 | 7.48 / 8.29 | 200.08 / 200.08 | 1.46 / 2.12 | +45.64% | usable |
| udp-v6-https-8 | 4.49 / 5.99 | 8.01 / 9.13 | 200.69 / 200.69 | 1.79 / 2.39 | +33.53% | usable |
| tcp-v4-http-1 | 3.85 / 5.03 | 7.13 / 7.09 | 200.08 / 200.08 | 1.54 / 2.01 | +30.61% | inconclusive-control/timing |
| tcp-v4-http-8 | 4.40 / 5.56 | 7.64 / 8.09 | 200.70 / 200.71 | 1.75 / 2.22 | +26.56% | usable |
| tcp-v6-https-1 | 3.87 / 5.76 | 7.12 / 7.48 | 200.08 / 200.07 | 1.55 / 2.30 | +48.81% | usable |
| tcp-v6-https-8 | 4.68 / 5.98 | 7.89 / 8.17 | 200.69 / 200.69 | 1.86 / 2.38 | +27.84% | usable |
| stealth-v4-http-1 | 4.20 / 5.60 | 5.10 / 5.37 | 200.08 / 200.08 | 1.68 / 2.24 | +33.10% | inconclusive-control/timing |
| stealth-v4-http-8 | 4.54 / 5.84 | 6.44 / 6.72 | 200.70 / 200.70 | 1.81 / 2.33 | +28.63% | usable |
| stealth-v6-https-1 | 4.18 / 5.65 | 5.05 / 5.34 | 200.08 / 200.08 | 1.67 / 2.26 | +35.30% | usable |
| stealth-v6-https-8 | 4.65 / 5.96 | 5.28 / 5.56 | 200.69 / 200.69 | 1.85 / 2.38 | +28.29% | usable |

### Pinned native ARM64 full 200 Mbps

[Actions run 43150498346](https://github.com/riKirsti/ProtunNext/actions/runs/43150498346), [artifact 13914247403](https://github.com/riKirsti/ProtunNext/actions/runs/43150498346/artifacts/13914247403); 96 verified timed downloads.

| Workload | CPU core % baseline / prototype | Peak RSS MiB baseline / prototype | Mbps baseline / prototype | CPU ns/B baseline / prototype | CPU cost change | Quality |
|---|---:|---:|---:|---:|---:|---|
| udp-v4-http-1 | 5.86 / 7.45 | 7.09 / 7.56 | 200.08 / 200.08 | 2.34 / 2.98 | +27.16% | usable |
| udp-v4-http-8 | 5.79 / 7.35 | 6.84 / 8.09 | 200.69 / 200.69 | 2.31 / 2.93 | +27.12% | usable |
| udp-v6-https-1 | 6.55 / 7.91 | 7.37 / 7.37 | 200.07 / 200.07 | 2.62 / 3.16 | +20.65% | usable |
| udp-v6-https-8 | 6.32 / 7.76 | 7.42 / 8.02 | 200.68 / 200.67 | 2.52 / 3.09 | +22.67% | usable |
| tcp-v4-http-1 | 5.47 / 7.00 | 5.81 / 6.54 | 200.08 / 200.08 | 2.19 / 2.80 | +27.95% | usable |
| tcp-v4-http-8 | 5.51 / 6.96 | 6.89 / 7.66 | 200.69 / 200.69 | 2.19 / 2.77 | +26.43% | usable |
| tcp-v6-https-1 | 6.10 / 7.47 | 5.84 / 6.47 | 200.07 / 200.07 | 2.44 / 2.99 | +22.48% | usable |
| tcp-v6-https-8 | 5.85 / 7.26 | 6.45 / 7.26 | 200.68 / 200.67 | 2.33 / 2.89 | +24.18% | usable |
| stealth-v4-http-1 | 6.51 / 8.08 | 4.29 / 4.39 | 200.07 / 200.07 | 2.60 / 3.23 | +24.12% | usable |
| stealth-v4-http-8 | 6.70 / 8.07 | 5.79 / 5.91 | 200.68 / 200.69 | 2.67 / 3.22 | +20.47% | usable |
| stealth-v6-https-1 | 7.10 / 8.55 | 4.16 / 4.39 | 200.07 / 200.07 | 2.84 / 3.42 | +20.46% | usable |
| stealth-v6-https-8 | 7.02 / 8.64 | 4.59 / 4.71 | 200.66 / 200.67 | 2.80 / 3.45 | +23.19% | usable |

## Separate profiles and interpretation

[Profile run 43150500762](https://github.com/riKirsti/ProtunNext/actions/runs/43150500762)
retains native [x64 artifact 13914161634](https://github.com/riKirsti/ProtunNext/actions/runs/43150500762/artifacts/13914161634)
and [ARM64 artifact 13913996566](https://github.com/riKirsti/ProtunNext/actions/runs/43150500762/artifacts/13913996566).
All 24 diagnostic 1 GiB downloads verify: baseline/prototype × three transports
× client-process/all-four-CPU captures × two architectures. Actual candidate
parallel work is recorded in every candidate capture. Normal release timing
above remains separate. Fixed auxiliaries are included in the system captures.

Baseline process profiles still show substantial AEAD work: the leading ring
open samples are approximately 44/47% for x64 UDP/TCP and 46/49% for ARM64;
Stealth spreads cost across open, AES and TLS. Candidate profiles put
authentication samples in both owner and helper, confirming distribution of
work rather than removal of cost. Sample shares, different run durations and
virtual CPU placement do not establish that IPC, cache misses or migration alone
caused the regression. Two allowed logical CPUs in a VM do not guarantee two
isolated physical cores. The measured whole-process CPU cost is the deciding
evidence for this design.

## Correctness, reproduction and completion

The pinned prototype passes all nine [platform regression jobs](https://github.com/riKirsti/ProtunNext/actions/runs/43150068065),
[release erasure checks](https://github.com/riKirsti/ProtunNext/actions/runs/43150068015)
and native analyzer/build checks. The archived patch retains the added
deferred-commit, stale-key, replay/tamper, ordering/control-barrier, pool-pressure
and completion-channel shutdown tests. The final ordinary source remains the
serial production code. [Final production/tooling verification](https://github.com/riKirsti/ProtunNext/pull/9/checks),
[native checks](https://github.com/riKirsti/ProtunNext/actions/runs/43152689847),
and [release erasure](https://github.com/riKirsti/ProtunNext/actions/runs/43152689833)
track the final non-document change. [Archive-only reconstruction checks](https://github.com/riKirsti/ProtunNext/actions/runs/43152487543)
pass on native x64/ARM64. Local reconstruction reproduces both variants' tested
runtime and effective source fingerprints exactly; canonical path ordering
keeps Windows/Linux identities comparable. Local Python checks pass (16 download
and ten audit tests). The analyzer rejects missing helper work, CPU-placement
mismatch, incorrect TUN mode and aggregate auxiliary saturation.

Stage 4 closes by rejecting this bounded batch/barrier implementation and its
placement follow-up. It does not exhaust asynchronous or coarser multicore
designs. Review the Stage 5 shortlist against these profiles, costs and coverage
gaps after this stage is merged; no next-stage implementation is included.
