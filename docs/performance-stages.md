# Agreed performance roadmap

**Campaign closure:** the user requested one final performance PR after PR #13,
then an end to this work for now. The [final policy](performance-policy.md)
supersedes prospective next-stage suggestions below; historical findings remain
unchanged. Deferred work is not automatically scheduled.

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
| 6 | Client-only PGO, architecture-specific builds and platform tuning | Evaluated in PR #11; retain optional independent build experiments, portable defaults unchanged |
| 7 | Combine measured winners and validate real workloads/platforms | Evaluated in PR #12; optional native combination and device PGO efficiency evidence, maximum capacity unresolved |
| 8 (focused follow-up) | Remove the measured peer bottleneck and diagnose delayed TCP before claiming client capacity | Evaluated in PR #13; tested helper prototypes do not establish headroom, maximum capacity remains inconclusive |
| Final selection | Review all evidence, settle production/build policy and validate the retained configuration | Final performance PR for now; review below |

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

## Stage 6 review, November 12

Frozen baseline: merged PR #10, `d6c070a0f9e211f5a5b14b7e8c8bc0f995d7b7a7`.
Keep the existing serial ring implementation, receive coalescing and production
dependency graph. Stage 5's TLS-suite experiment did not reproduce a CPU benefit
in the full matrix and increased RSS; primitive replacements did not justify
integration. Compiled Rust packet/queue/checksum/control code remains worth
measuring without changing crypto or introducing another backend/thread.

Compare two build experiments independently: client-only profile-guided
optimization, and explicit CPU-specific code generation. Keep thin LTO,
codegen-units=1, panic=abort and the same source/features except each declared
build difference. Preserve portable release defaults. CPU-specific binaries
must identify the actual CPU/feature requirements and cannot be presented as
portable builds or copied to a different device architecture. Do not combine
PGO and CPU tuning here; Stage 7 combines only measured winners.

PGO training uses fresh instrumented clients through the real kernel TCP/TUN
fixture, all three transports and separate IPv4/IPv6 HTTP(S) workloads. Peer,
server and load remain fixed uninstrumented baseline programs. Train on flow
counts distinct from the held-out one/eight-flow evaluation. Merge only fresh
counters with the matching compiler's LLVM tools; record source/target/compiler,
flags, training cases, profile and binary identities. Instrumented observations
are never speed estimates. The final candidate must consume the recorded profile
and evaluate as an ordinary optimized release program.

Validate actual downloads with equal CPU allowances and vnet contracts, balanced
order, A/A/timing/payload/identity/headroom screens. Equal source fingerprints
are expected for build-only changes; distinct compiler configuration and binary
identities must prove the comparison rather than imply an A/A speedup. Native
x64/ARM64 comparisons report absolute CPU %, CPU/byte, RSS MiB and Mbps across
UDP/TCP/Stealth. Keep supported-LTS compatibility separate from native tuning;
do not assume a host-native instruction set works in a virtual guest.

Preserve cross-platform behavior and erasure checks. Run guarded temporary Travel
comparisons of the latest selected code/build for all three transports, with
owned artifacts, narrow routing, watchdog and independent restoration. Mark
build-only/unmeasured targets explicitly. Keep only justified build recipes or
changes in one Stage 6 PR; no persistent router/NIC setting, release or invented
CPU/RAM ceiling. Record rejected/inconclusive cases and limitations. Review Stage
7 before starting combined-build and real-workload/platform acceptance.

## Stage 6 result

[PR #11](https://github.com/riKirsti/ProtunNext/pull/11) independently evaluates
client-only PGO and CPU-specific compilation. All 384 full native downloads
verify and all 48 workload cells pass A/A/timing checks. Native CPU/byte generally
falls, with memory tradeoffs and one ARM64 CPU-build regression. Paced 200 Mbps
measurements establish efficiency at that load, not higher maximum capacity.
All 48 LTS downloads in three separate cohorts verify, but auxiliary limits and
one final 6.18 A/A failure prevent a maximum-speed conclusion.

Both physical Travel comparisons verify all 24 downloads each, with cleanup and
twelve independent restoration checks passing after each. Cortex-A53 compilation
does not improve CPU/byte there. PGO's Stealth observation is promising, with
higher RSS; its TCP CPU/byte is higher and UDP/TCP WAN controls are noisy. Keep
target-specific build experiments available for further acceptance, without
changing portable defaults, installing a device binary or claiming a universal
winner. Production source/dependencies, ring and existing serial receiver remain.

The [complete Stage 6 report](benchmarks/stage6-builds.md) retains all 472 timed
GitHub observations, build/profile/CPU identities, failed/superseded controls,
unmeasured platforms and [physical-device evidence](benchmarks/stage6-travel.md).
No PGO/CPU combination, persistent NIC/router change or release ships. After
merge, review Stage 7's frozen baseline, supported winners, independent component
and combined comparisons, real workloads/platforms, capacity and completion
criteria before implementing it. Deferred architectures remain open questions.

## Stage 7 review, November 12

Frozen baseline: merged PR #11, `83582248f3a8e153c6ee75d2846ef7c7064b6323`.
Stage 6 finds native efficiency candidates, not established maximum-speed gains.
PGO and native CPU tuning ran on separate allocations; device CPU tuning did not
win, and device PGO is mixed. Keep portable defaults and the existing ring,
serial receiver and coalescing. No rejected backend/thread design is reinstated.

Compare portable, PGO, CPU and PGO+CPU on each same native x64/ARM64 machine.
Build the baseline once and hold its client/helper binaries fixed. Reuse the same
frozen source archives, train fresh matching profiles separately for portable and
CPU-specific PGO, and retain all compiler/profile/binary identities. Interleave
the candidate comparisons by workload, with A/A and balanced sample order.
Keep matched-load efficiency separate from unpaced sustained-download capacity;
retain auxiliary saturation, timing and control failures rather than declare a
maximum-speed winner from an invalid cell. Do not add a fixed gain/CPU/RAM cutoff.

Use held-out IPv4 HTTP/IPv6 HTTPS and one/eight-flow real kernel TCP downloads
across UDP/TCP/Stealth. Assess realistic network conditions in disposable test
namespaces if the fixture supports them without altering host networking; label
emulation separately from physical WAN acceptance. Report the measured platform,
workload duration, absolute CPU %, peak RSS MiB and verified Mbps for every build.
Keep Windows/Apple/Android and other unmeasured targets explicit. Preserve API,
security and tuned erasure checks, including genuine missing-wipe controls.

On Travel, compare portable release with independently device-trained PGO only;
do not combine the losing Cortex-A53 option. Repeat guarded all-transport tests
with owned temporary files/routes, watchdog and independent restoration. Retain
WAN variability and any failures. No saved setting, package, key/account, Direct
path, persistent NIC configuration or release changes.

One Stage 7 PR selects only justified target-specific recipes or changes, records
rejected/inconclusive options, and distinguishes observed efficiency from actual
sustained capacity. Completion requires full source-linked resource tables,
regression/erasure evidence and physical restoration; unmeasured capacity or
platforms remain limitations rather than claims that every architecture is
exhausted or absolute best performance is proven.

## Stage 7 result

[PR #12](https://github.com/riKirsti/ProtunNext/pull/12) compares portable, PGO,
CPU-specific and combined builds on the same native hosts, with a byte-identical
fixed baseline/helper and fresh flag-matched profiles. All 1,728 payloads verify;
211/216 comparison cells are usable, three fail A/A stability and two fail the
requested rate. Every unpaced 4 GiB cell is auxiliary-limited. Combined tuning
generally improves CPU/byte beyond its individual components, but no maximum
client-speed winner is established. The delayed TCP rate misses remain explicit.

Longer physical Travel sessions verify all 128 individual 100 MB parts in 32
timed sessions. UDP/TCP and a separate passing Stealth repeat observe PGO CPU/byte
reductions of 6.89% / 6.55% / 6.17%, with observed Mbps increases and small memory
tradeoffs. The first noisy Stealth cohort is retained separately. Cleanup and
twelve independent restoration checks pass after each cohort. Device CPU tuning
is not combined or installed. The measured path is NoLocalAgent, not universal
account-enabled CLI/SDK or OS performance qualification.

Keep opt-in native combined-build experiments and fresh device PGO as supported
efficiency candidates. Portable defaults, production source/dependencies, ring,
serial processing, bounded ownership and prior receive improvements remain.
No persistent router/NIC change, global compiler policy or release ships.
The [complete report](benchmarks/stage7-acceptance.md) retains absolute CPU/RSS/Mbps,
all cells, identities, controls, quality failures and unmeasured platforms.

The seven planned stages are evaluated; absolute best performance is not proven.
Remaining evidence includes client capacity without a helper bottleneck, delayed
TCP behavior, NIC/RTT/loss/thermal effects and account-enabled CLI/SDK / unmeasured
OS targets. Deferred architectures remain open. Review a focused follow-up plan
before any further implementation; do not turn these gaps into universal claims.

## Stage 8 focused follow-up review, November 12

Frozen baseline: merged PR #12 `64caaa24004db4f9cc901cc76d62d51bbc61d983`.
This extends the evaluated seven-stage roadmap to resolve its explicit capacity
and delayed-TCP gaps; it is not a retrospective change to earlier results.
Stage 7 raw baseline observations show the peer wire thread near 0.998–0.999
core, with client roughly 0.71–0.88 core, HTTP server 0.10–0.13 and load
0.16–0.32. Profiling the saturated peer is the next action, not another blind
client optimization or a claim from speeding up a test helper.

Capture separate frame-pointer/system profiles of the frozen peer/client paths.
Then screen benchmark-only peer improvements using existing in-place APIs and,
only if profiling supports it, a bounded TUN-read handoff on an auxiliary CPU.
Keep the production client serial, ring-based and otherwise unchanged. Do not
revive rejected production parallel-crypto designs or weaken protocol/payload,
ownership, erasure, framing, routing or handshake checks. Verify bounded helper
ownership and all-transport correctness before using a new helper for comparisons.

Hold each selected helper binary/configuration fixed across baseline and tuned
clients. Keep the client CPU allowance identical. Record each auxiliary thread
and any shared load/server allowance so extra helper CPU cannot be hidden or
misclassified as client capacity. Compare portable and fresh matching PGO/native
builds only after measuring headroom. If helpers still saturate, report the
remaining bottleneck rather than accept a maximum-speed ranking.

For the delayed TCP misses, collect qdisc/drop/backlog and inner/outer TCP state
in separately labeled diagnostic runs, alongside actual timing and payload
evidence. Do not erase old failures, manufacture a passing rate, or mix sampled
diagnostics into normal-release speed values. Temporary network emulation stays
inside disposable namespaces; no host/persistent NIC change.

One follow-up PR must retain absolute CPU %, peak RSS MiB and verified Mbps for
UDP/TCP/Stealth by measured platform, all identities/control failures, standard
platform/API and tuned erasure checks, and guarded temporary latest-code Travel
tests with watchdog and independent restoration. Device CPU tuning remains
excluded; preserve packages/settings/account/key/Direct mode and create no release.
Completion requires a justified helper choice, an explicit client-capacity
accept/reject/inconclusive result, delayed-TCP diagnosis and restored physical
tests. No fixed client CPU/RAM ceiling or minimum gain threshold is added.

## Stage 8 result

[PR #13](https://github.com/riKirsti/ProtunNext/pull/13) retains separate frozen
profiles, all 96 verified normal-release helper-screen payloads and 24 verified
sampled delayed-TCP cases. All twelve helper-screen cells pass A/A/timing, but
neither prototype creates sufficient auxiliary headroom. The bounded reader
raises x64 fixture throughput approximately 36–41% while regressing ARM64
12–14%; in-place processing alone gives small, inconsistent changes. These are
changes to the test system with byte-identical clients, not production speedups.

Reject both prototypes as default helpers; keep the existing legacy fixture.
Retain their opt-in experiment and bounded ownership tests for reproducibility.
Maximum client capacity remains **inconclusive**. As reviewed, do not repeat
tuned-client comparisons before establishing helper headroom. Production
runtime/dependencies, portable defaults and Stage 7's efficiency evidence remain.

The diagnostic TCP run does not reproduce the previous severe misses: all
cases sustain approximately 199.68–199.70 Mbps, with zero final client netem
drops. Peer socket retransmissions and receive-window pressure are still
observed. Sampling changes timing; no causal fix or universal TCP qualification
is claimed. Preserve the original failures and the unresolved status.

The [complete capacity/resource report](benchmarks/stage8-capacity.md),
[diagnostic evidence](benchmarks/stage8-diagnostics.md) and
[fresh temporary Travel validation](benchmarks/stage8-travel.md) retain separate
scopes, source/build/profile identities, controls, failures and restoration
checks. All 24 timed Travel sessions / 96 individual payload parts verify, with
cleanup and twelve independent checks passing after training and every cohort.
All three WAN A/A screens fail, so none supplies a new accepted speed/efficiency
gain. No device installation, persistent setting or release is proposed.
Further capacity work needs a newly reviewed focused plan and an independently
justified peer architecture; it is not permission to restart rejected production
parallel cryptography or claim that the remaining performance gaps are closed.

## Final selection review, November 12

The user closes performance work with one final PR after merged PR #13.
Frozen baseline: `7c582a28a528701ef3b3874f476903ab10f205a7`.
Review Stages 1–8, including negative results, controls, compiler/profile scope,
account-feature compatibility and device failures. Select the existing serial
ring-based bounded worker, direct receive/framing improvements, negotiated Linux
coalescing and portable product defaults. Keep target-specific builds optional;
do not promote NoLocalAgent profiles to full-feature products.

Consolidate the final policy and correct stale guidance. Preserve historical
reports and security/descriptor/API contracts. Run the existing normal-release
matched-load matrix on native x64/ARM64 using fixed baseline auxiliaries and
unchanged controls. Test current portable code temporarily on Travel for all
three transports with owned routing/artifacts, watchdog and independent
restoration. Retain absolute CPU/RSS/Mbps, identities, noise and failures.
An unchanged runtime comparison cannot establish a new optimization gain.

Completion is a reviewable final PR with the all-stage decision, practical build/
integration guidance, fresh validation and restored Travel. No new architecture,
mandatory tuning, persistent setting or release. Record unproven maximum capacity,
TCP impairment, account-enabled and unmeasured OS/device performance as limits;
they do not extend this campaign. Further performance research requires the user
to reopen it.
