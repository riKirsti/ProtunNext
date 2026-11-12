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
| 3 | Genuine receive-side TUN coalescing and offload metadata | Merged in PR #8 |
| 4 | Multicore receive architectures and asynchronous decryption | Evaluated in PR #9; reject tested batch/barrier designs, retain serial production receiver |
| 5 | Alternative acceleration: crypto implementations, native WireGuard, advanced I/O/TLS, CPU/NIC coordination and kernel bypass | Evaluated in PR #10; retain ring, archive TLS-suite experiment and record deferred architectures |
| 6 | Client-only PGO, architecture-specific builds and platform tuning | Review before starting |
| 7 | Combine measured winners and validate real workloads/platforms | Review before starting |

One stage per PR. Before each transition, review the goal, frozen baseline,
evidence, proposed changes, exclusions, validation and completion criteria with
the user. Preserve independent comparisons inside the stage; do not combine
unmeasured effects or carry speculative backends into production.

Starting with Stage 3, every stage PR includes absolute baseline/candidate CPU
usage (100% = one core), peak process memory MiB and verified download Mbps by
measured platform and by WireGuard UDP, WireGuard TCP and Stealth. Put readable
tables in the PR and retain full workload tables, immutable source/binary
identities, GitHub Actions evidence and measurement limits in the stage report.
Label build-only platforms as performance-unmeasured.

Each stage also temporarily tests its latest production code on the physical
Travel router for all three transports, comparing against the frozen baseline.
Use only owned temporary artifacts and narrowly scoped test routing, an automatic
cleanup watchdog, and independent before/after verification. Preserve Direct mode,
saved configuration, account/key and installed packages; report failures, noise,
CPU/memory definitions, WAN limitations and cleanup results explicitly.

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
The [latest-code physical Travel supplement](benchmarks/stage3-travel.md) records
CPU %, RSS MiB and verified Mbps for all three transports. All twelve balanced
comparison downloads succeed; one of twelve baseline A/A sessions fails at warmup.
WAN controls are noisy/incomplete, and Travel RSS increases despite lower observed
CPU/byte. Cleanup and independent restoration checks pass. These physical results
remain separate from the controlled GitHub cohorts and do not establish a clean
WAN speed ranking.
The Stage 4 transition review below freezes the merged baseline and revisits
crypto/receive evidence, ordered bounded parallelism, session/replay ownership,
backpressure, validation and completion criteria before implementation.

## Stage 4 review, November 9

Frozen baseline: merged PR #8, `a44d520c337611f59d4c7186971919d57de36df8`.
After coalescing, decryption accounts for roughly 19-45% of sampled client work.
Travel proves physical coalescing interoperability, but its noisy WAN controls
and increased RSS do not establish a repeatable speed or memory improvement.

Evaluate a bounded pipeline with one decryption helper plus the existing owner.
Workers authenticate owned packet buffers using the existing cryptographic
implementation. Only the connection owner commits replay/session/timer/statistics
updates and delivers plaintext in receive order. Control messages, rekey, interface
replacement and shutdown are barriers. Keep small batches serial; no packet
copying, unbounded queue, new algorithm/backend, mandatory PGO or release.

Use identical baseline/candidate TUN modes and CPU allowances. Native multicore
comparisons must account for shared helper CPUs and not conceal auxiliary limits.
Measure real verified downloads, client CPU %, CPU/byte, RSS and Mbps across all
transports; retain A/A/order controls, immutable identities, failures and gaps.
Exercise replay/tamper, invalid packets, counter/session changes, ordering,
backpressure, cancellation, ownership and cleanup. Include temporary latest-code
Travel comparisons with restoration checks. Keep only justified changes in one
stage PR, with accept/reject/inconclusive decisions before the next stage review.

## Stage 4 result

[PR #9](https://github.com/riKirsti/ProtunNext/pull/9) preserves the tested bounded
parallel receiver as an explicit archive-only patch and keeps PR #8 production
source. All 248 normal-release timed downloads verify. Fixed-rate native x64
and ARM64 results increase CPU/byte across all qualified cells; most RSS values
also increase. The unpaced LTS UDP cohorts are helper-limited and mixed in speed.
All twelve balanced Travel comparisons verify, with lower observed prototype
speed across UDP/TCP/Stealth; one baseline A/A warmup failure remains recorded.
Cleanup and independent restoration checks pass. The final serial production
path is the same baseline physically tested on Travel.

The [complete Stage 4 evaluation](benchmarks/stage4-receive.md) retains source
identities, all workload results, profiles, initial/pinned comparisons, control
failures, build-only platform labels and coverage gaps. This rejects these
batch/barrier implementations, not all asynchronous/multicore receive designs.
After this stage merges, review Stage 5's evidence-backed acceleration shortlist
before implementation. No Stage 5 code is included in this stage PR.

## Stage 5 review, November 11

Frozen baseline: merged PR #9, `4c7cc53a0932fa31579729b603d80a9b9e9ecabf`.
Production remains the serial PR #8 receiver, including negotiated vnet
coalescing. Stage 4's added thread increased whole-process CPU cost; its profiles
still put roughly 44–49% of native UDP/TCP client samples in AEAD opening.

Start with independent comparisons of established packet-crypto implementations:
current ring 0.17.14, the existing RustCrypto ChaCha20Poly1305 0.10.1 dependency,
and AWS-LC's compatible AEAD API. Keep the IETF ChaCha20-Poly1305 construction,
WireGuard nonce/tag/wire format, session/replay/timer ownership and in-place
buffer contract. Do not write a cipher or change algorithms. Screen representative
packet sizes and cross-implementation interoperability first; primitive results
only decide which candidates merit complete-download integration. A faster
primitive alone is never a production speedup.

For competitive candidates, compare actual UDP/TCP/Stealth downloads with fixed
baseline auxiliaries, equal CPU allowances and vnet mode, balanced order, A/A
screens, exact payload verification and source/binary identities. Report absolute
CPU %, CPU/byte, RSS MiB and Mbps by measured platform, all control failures and
unmeasured targets. Separate instrumentation from normal-release measurements.
Include supported-LTS compatibility, native x64/ARM64 resource comparisons,
cross-platform regression/erasure checks, and guarded temporary latest-selected-
code Travel comparisons for all three transports with independent restoration.

Keep native WireGuard, advanced I/O/TLS, CPU/NIC coordination and kernel bypass
on the shortlist, assessing profile relevance, privilege requirements and
UDP/TCP/Stealth/API coverage before adding implementation complexity. A TLS
provider comparison must remain separate from a packet-AEAD change. Do not bundle
PGO or architecture-specific build tuning (Stage 6), additional helper threads,
persistent router/NIC changes or a release. One Stage 5 PR records accepted,
rejected and deferred candidates and keeps only whole-path-justified changes.
Review the Stage 6 plan before that implementation starts.

## Stage 5 result

[PR #10](https://github.com/riKirsti/ProtunNext/pull/10) retains existing ring
production code and dependencies. Existing/current RustCrypto are slower on
packet-sized primitive screens; AWS-LC packet opening is nearly tied. The
separate AWS-LC TLS-suite prototype does not reproduce its first focused x64
CPU reduction in the full matrix. All 224 normal-release downloads verify;
all 24 full native cells pass controls/timing. Full Stealth CPU cost is nearly
unchanged or higher, with higher RSS on both native architectures. LTS focused
results remain auxiliary-limited; no maximum-capacity benefit is established.

All twelve balanced Travel downloads verify; one baseline A/A warmup fails and
WAN control variability remains. Stealth has higher observed CPU/byte and RSS.
Cleanup and all twelve independent restoration checks pass. The final selected
production path is the baseline physically tested for all three transports.
All nine platform jobs and release-erasure checks pass for the tested code.

The [complete evaluation](benchmarks/stage5-acceleration.md) retains primitive
screens, focused/full cohorts, profiles, absolute CPU/memory/Mbps tables,
identities, compatibility constraints and unmeasured platforms. The TLS-suite
patch is archive-only; its dual-provider/Local Agent compatibility is unresolved.
No production helper, backend switch, new dependency, global-provider policy,
router setting or release ships. No fixed CPU/RAM ceiling was used. Deferred
native WireGuard/I/O/NIC/bypass designs are not performance-rejected or exhausted.
After merge, review Stage 6 before starting client-only PGO/build/platform tuning.
