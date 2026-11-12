# Final performance selection

This is the final decision for the current campaign, after reviewing Stages 1–8.
The selected production implementation is merged PR #13,
`7c582a28a528701ef3b3874f476903ab10f205a7`. The closing PR consolidates and
validates that choice; it introduces no new packet-path architecture.

**Use the existing serial, ring-based, bounded production path and portable
release build. For a Linux integration that owns TUN creation, explicitly
prepare receive coalescing on a new compatible descriptor. Keep PGO and CPU
specialization optional and target-qualified. Do not distribute a benchmark
profile as a production profile or promote host-native flags globally.**

This selects the best justified configuration from this campaign. Absolute
maximum speed across all platforms has not been established. The campaign is
closed for now; recorded gaps are not an automatic next-stage work queue.
Reopening performance research requires a new user request.

## Production policy

| Area | Final selection | Reason |
|---|---|---|
| Ownership | Compact reusable pools, bounded queues, tracked erasure, in-place WireGuard data crypto | Preserve accepted security behavior and avoid unnecessary copies/storage |
| Framed receive | Direct complete-frame extraction and MTU-sized borrowed-TLS prefetch | Stage 2 retains memory reductions without maximum-size buffer promotion |
| Linux TUN | Explicit receive-only vnet coalescing on newly owned descriptors; raw descriptors remain raw | Stage 3 reduces client CPU/byte in every final measured cell; preserve ABI and fallback |
| Receiver | Serial connection/session owner | Tested batch/barrier and placement designs increase whole-process cost |
| UDP | Existing mmsg/GSO/GRO where supported, batch 16, no wait to fill | Batch 32 does not establish a repeatable gain |
| Crypto | Existing ring packet and TLS provider policy | Replacement primitives do not justify adoption; TLS provider trial raises RSS and leaves integration issues |
| TLS I/O | Existing unbuffered Linux/Android and buffered Windows/Apple selection | Preserve platform choices, bounded backpressure and accepted-byte ownership |
| Budgets | Existing worker budgets, pools and queue bounds | No sustained-pressure evidence justifies larger allocations or budgets |
| General build | Portable release, thin LTO, one codegen unit, existing panic policy and product features | Preserve supported target compatibility |
| Native tuning | Optional fresh matching PGO + CPU-native experiment on the measured host | Stage 7 shows efficiency gains; arbitrary CPUs, OS targets and feature sets remain unqualified |
| Travel | Portable build with negotiated Linux coalescing; device PGO remains an optional measured candidate | Cortex-A53 flags regress; PGO WAN evidence is mixed/noisy across stages |
| Test peer | Legacy helper | Stage 8 reader still saturates and regresses ARM64; helper speed is not client speed |

UDP, TCP and Stealth retain their security and account semantics. No automatic
transport switch, global TLS provider installation, production crypto helper,
mandatory affinity or NIC/sysctl change is selected. Benchmark environment
variables are not product settings.

## Review of every stage

| Stage | Evidence considered | Final decision |
|---|---|---|
| 1 / PR #5–6 | Real kernel TCP/HTTPS baseline, original/corrected peer failures and 192-download UDP batch experiment | Keep verification/regressions and batch 16; measurement infrastructure is not a speedup |
| 2 / PR #7 | 504 verified timed downloads; matched-load Stealth RSS reductions 25.95–34.38% x64 and 22.32–37.17% ARM64 | Keep direct extraction and MTU-sized prefetch; reject maximum-sized prefetch |
| 3 / PR #8 | 576 verified final downloads / 72 passing control/timing cells; matched-load CPU/byte reductions 40.26–50.08% x64 and 26.20–38.92% ARM64 | Keep negotiated Linux receive coalescing, raw ABI and unsupported-packet fallback |
| 4 / PR #9 | 248 verified timed downloads, placement follow-up and device tests; qualified full-matrix CPU/byte rises 26.56–48.81% x64 and 20.46–27.95% ARM64 | Keep serial; archive the tested parallel design |
| 5 / PR #10 | Primitive screens, 224 verified whole-path downloads, TLS RSS increases, provider compatibility and device tests | Keep ring/current adapters; archive suite-provider trial; untested backends remain untested |
| 6 / PR #11 | Independent CPU/PGO cohorts, optimized erasure controls, LTS limits, device regressions and failed/superseded checks | Keep optional build experiments; no global flag promotion |
| 7 / PR #12 | 1,728 verified payloads / 216 cells, 211 usable; combined efficiency gains, two delayed-TCP rate misses and helper-limited capacity; 32 physical sessions with retained noisy Stealth cohort | Retain qualified optional native combination/device PGO candidate; no maximum-speed winner |
| 8 / PR #13 | 96 verified helper-screen payloads, separate profiles, 24 sampled TCP cases, 24 physical sessions, ARM64 reader regression and three failed WAN controls | Keep legacy helper and unresolved TCP/capacity limits; no new accepted WAN gain |

These experiments have different source identities, hosts and workloads. Do not
multiply or add their percentages into a cumulative speedup. Matched-rate data
measures efficiency at that rate; helper-limited/noisy data cannot rank maximum
client throughput. All failures and identities remain in the original reports:
[baseline](benchmarks/download-baseline.md), [batch](benchmarks/udp-batch-experiment.md),
[Stage 2](benchmarks/stage2-receive.md), [Stage 3](benchmarks/stage3-tun.md),
[Stage 4](benchmarks/stage4-receive.md), [Stage 5](benchmarks/stage5-acceleration.md),
[Stage 6](benchmarks/stage6-builds.md), [Stage 7](benchmarks/stage7-acceptance.md),
[Stage 8](benchmarks/stage8-capacity.md).

## Build and integration guidance

The ordinary product build preserves its default Local Agent and bindings:

```sh
cargo build --locked --release --lib --bin protun-next-cli
```

Use the existing [platform build/SDK instructions](supported-platforms.md) for
cross-target packaging. Do not copy runner-native binaries to another CPU or
install an injected benchmark fixture. A measured NoLocalAgent configuration
does not authorize dropping account functionality from a product. Keep debug
counters and generation instrumentation out of normal-release speed reports.

On Linux, create a new, exclusively owned TUN with
`IFF_TUN | IFF_NO_PI | IFF_VNET_HDR`, call
`linux::prepare_receive_coalescing(fd)` **before attachment**, then attach it
through the existing API. Discard that new descriptor on preparation failure:
setup ioctls are not transactional. Never convert an existing/raw caller
descriptor. The negotiated ABI uses ten-byte little-endian headers and disables
kernel-to-userspace offloads. Unsupported groups retain ordinary delivery;
aggregate rejection disables coalescing and retries originals. Android, Apple
and Windows keep their descriptor contracts. See the
[full contract/security checks](benchmarks/stage3-tun.md#descriptor-contract).

PGO/CPU tuning remains an opt-in experiment, not the portable product default.
For a fixed deployment, qualify the exact source, compiler/LLVM tools, target,
CPU features and full product features; train fresh representative profiles,
hold out evaluation traffic, retain identities, run functional/erasure checks
and compare speed/CPU/memory against portable. Generation/use flags otherwise
must match; final binaries must omit generation counters. Do not reuse a
historical benchmark profile for a new revision, Local Agent build, target or
device. Rebuild/requalify after changes. The
[experiment tools](../experiments/build/README.md) reproduce evidence; they are
not a full-feature product training/deployment pipeline.

## Closing validation and boundaries

The [final validation report](benchmarks/final-performance.md) records fresh
portable baseline/current CPU %, peak process RSS MiB and verified Mbps for
native Linux x64/ARM64 and temporary Travel UDP/TCP/Stealth tests. Unchanged
runtime comparisons characterize/regression-check the selection; they establish
no new speedup. CPU 100% means one core; process metrics exclude auxiliary/kernel
memory and asynchronous kernel CPU. Keep per-workload controls/failures visible.

Windows, Apple and Android have build/API checks but no measured closing-stage
download performance. Account-enabled CLI/SDK capacity, physical NIC/RTT/loss/
thermal behavior, a peer-independent capacity ceiling and old delayed-TCP misses
remain unqualified. Stage 8 did not reproduce those misses or establish a causal
fix. These gaps prevent universal-best claims; they do not require another
performance PR now.

No release, permanent installation, saved setting, package or account/key change
is part of this decision. Broader live/paid-feature acceptance retains the
existing [parity ledger](parity-scope.md).
