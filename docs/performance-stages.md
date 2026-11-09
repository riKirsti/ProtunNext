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
| 2 | Simple receive-path improvements: framing/copies, scheduling, batching, bounded storage | Active; one PR against merged PR #6 |
| 3 | Genuine receive-side TUN coalescing and offload metadata | Review before starting |
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
