# Agreed performance roadmap

The objective is maximum worthwhile sustained download throughput for WireGuard
UDP, WireGuard TCP and Stealth, preserving existing functions and security.
Additional CPU and RAM may be justified by measured gains. There are no fixed
25% CPU or 8/32 MiB rejection limits, and higher budgets are not usage targets.
Research is open-ended; earlier prototypes do not exclude different designs.

## Stage boundaries

| Stage | Scope | Status |
|---|---|---|
| 1 | Real inner TCP/HTTPS baseline and complete-path profiling | Initial foundation merged in PR #5; PR #6 retains batch-screening evidence and regression checks |
| 2 | Simple receive-path improvements: framing/copies, scheduling, batching, bounded storage | Merged in PR #7 |
| 3 | Genuine receive-side TUN coalescing and offload metadata | Evaluated in PR #8; ready for review |
| 4 | Multicore receive architectures and asynchronous decryption | Review before starting |
| 5 | Alternative acceleration: crypto implementations, native WireGuard, advanced I/O/TLS, CPU/NIC coordination and kernel bypass | Review the evidence-backed shortlist before starting |
| 6 | Client-only PGO, architecture-specific builds and platform tuning | Review before starting |
| 7 | Combine measured winners and validate real workloads/platforms | Review before starting |

One stage per PR. Before each transition, review the goal, frozen baseline,
evidence, proposed changes, exclusions, validation and completion criteria with
the user. Preserve independent comparisons inside the stage; do not combine
unmeasured effects or carry speculative backends into production.

## Stage 2 review, November 9

Baseline: merged PR #6, `02fac0940cdd508a10821048bf1b5a0fe7eed93c`.

Source review identifies an extra framing-ring copy for complete TCP frames.
Linux/Android Stealth already borrows TLS plaintext, but subsequently copies
complete WireGuard frames through that ring. The candidate extracts complete
frames directly into owned packet buffers, preserving the ring for fragments.
Borrowed TLS spans use a bounded prefetch queue and the existing packet pool;
queue/pool pressure falls back to buffered decoding. Keep prediction, malformed
frame rejection, ordered delivery, ACK/control responsiveness and erasure intact.

Keep UDP batches at 16 and the current processing budgets, queue limits and pool
count unless retained pressure measurements justify a separate comparison.
No TUN coalescing, crypto concurrency, new backend or PGO belongs in Stage 2.

Use the existing baseline/candidate archive builder and download workflows.
Validate fragments, compressed headers, partial EOF, output/pool pressure,
prefetch ownership and TLS integration. Compare actual downloads on supported
Linux LTS and native x64/ARM64, with fixed baseline auxiliaries, A/A controls,
balanced order and explicit headroom/measurement-quality screens. UDP is an
unchanged-path control. Platform builds do not establish platform speedups.

The stage PR must report measured speed, CPU, memory, reliability and known
latency/device gaps by target, and a clear accept/reject/inconclusive decision
for each candidate. Finish with evidence-backed recommendations and the next
stage review; do not equate a passing test count with a performance improvement.
Controlled measurements support selection; physical-device and Proton WAN
validation remain explicit evidence gaps until actually performed.

## Stage 2 result

[PR #7](https://github.com/riKirsti/ProtunNext/pull/7) retains direct complete-frame
extraction and compact borrowed-TLS packet ownership. All 504 downloads in the
completed retained cohorts are valid. The clearest result is lower Stealth peak
client RSS, reproduced on both x64 LTS kernels and native x64/ARM64. Matched-load
ARM64 CPU reductions are observed; x64 CPU changes are small/mixed. Maximum
download-speed gains remain unproven. Worker bounds and UDP batch 16 remain.
See the [complete evaluation](benchmarks/stage2-receive.md) for source identities,
controls, original limitations and platform-specific recommendations.

## Stage 3 review, November 9

Baseline: merged PR #7, `f26fd3d741d125037797ec9953de1d50ebd93c8a`.
The chat review covers receive-side TUN work across UDP, TCP and Stealth, while
preserving the existing raw descriptor contract. Earlier profiles identify TUN
delivery and kernel receive processing as worthwhile candidates; they do not
establish that coalescing improves maximum throughput.

Implement conservative adjacent inner-TCP aggregation only for explicitly
negotiated Linux vnet descriptors. Validate original checksums, segmentation,
metadata, packet order, backpressure and exact ownership before comparing real
downloads. Raw TUN and other platforms retain ordinary delivery. Keep queue,
worker and crypto budgets fixed; multicore decryption, alternative backends and
PGO belong to later stages.

Use fixed PR #7 peer/load binaries, A/A controls and balanced comparisons. Check
both IPv4 HTTP and IPv6 HTTPS, one/eight flows and all three outer transports.
Retain failures, source identities, client CPU/byte, RSS, throughput, coalescing
counters and helper/headroom screens. Supported LTS kernels and native x64/ARM64
must establish compatibility independently of cross-platform build checks.
Reduced TUN calls alone do not establish a speedup; report measured benefits,
regressions and inconclusive cells before selecting a candidate in one stage PR.

## Stage 3 result

[PR #8](https://github.com/riKirsti/ProtunNext/pull/8) retains explicitly negotiated
Linux receive-side coalescing with original-checksum validation, bounded groups,
immutable queue ownership and erased copied-header scratch. All 576 final-code
timed downloads are valid; all 72 workload cells pass control/timing screens.
At 200 Mbps, client CPU/byte falls 40.26-50.08% on native x64 and 26.20-38.92%
on native ARM64 across all three transports. Peak client RSS falls in every cell.
All unpaced cells remain helper-limited; maximum client throughput is unproven.
All nine platform jobs and both native release-erasure checks pass.

The [complete Stage 3 evaluation](benchmarks/stage3-tun.md) retains all workload
results, preliminary/superseded launches, source identities, profiles and gaps.
Existing raw callers remain compatible; other platforms keep ordinary delivery.
Stage 4 has not started. After merge, freeze the merged baseline and review the
crypto/receive evidence, ordered bounded parallelism, session/replay ownership,
backpressure, validation and completion criteria before implementation.
