TLS selection after the cross-platform and Travel reruns
======================================================

The production adapter is chosen at compile time. Ordinary certificate-verifying
TLS and Stealth use the same adapter on a target, with their existing separate
trust policies. There is no runtime switch, CPU-model dispatch or tuning matrix.

| Target | Adapter | Evidence and limits |
|---|---|---|
| Linux x64, 6.12 / 6.18 LTS | Unbuffered | Actual LTS full-worker TLS 1.3 paired medians +50.78% / +48.36%; CPU/byte -35.67% / -33.66% |
| Linux ARM64, including Travel | Unbuffered | Travel's three-pair/server WAN comparison favors it; a prior native Neoverse-N2 worker instead lost 3.56%. This is not a universal ARM64 win or native ARM64 6.18 acceptance |
| macOS / iOS / iPadOS 26 / 27 ARM64 | Buffered | macOS M1 VM framing tests favor buffered; iOS/iPadOS selection is provisional pending actual device/Network Extension tests |
| Windows 11 25H2 / 26H2 x64 | Buffered | Large framing reruns were noisy and inconclusive. Use the simpler bounded adapter until a representative worker comparison supports a change; 25H2 runtime performance remains unmeasured |
| Android 16 / 17, supplied 16 KB environment | Unbuffered | Retains the current implementation; native VpnService application performance selection is unverified |

These are engineering choices supported to different degrees, not a fastest-
backend guarantee. macOS framing is component evidence, not complete VPN
throughput. Keeping buffered on Windows does not claim a statistically proved
speedup. New performance measurements should compare the shipping worker and
the relevant device/API, without treating CPU architecture as a workload proxy.

The [one-hour campaign](benchmarks/decision-hour-20261104.md) retains its source
revisions, actual kernel versions, runner architectures, sample counts and
remaining gates. Its [summary JSON](benchmarks/decision-hour-20261104.json) and
[paired tables](benchmarks/decision-hour-20261104-paired.md) accompany the report.
The historical recommendation for Linux ARM64 must be read alongside the newer
Travel evidence rather than silently generalized to all ARM64 targets.

The [Travel rerun](benchmarks/travel-decision-20261105.md) used one physical
two-core Cortex-A53 running Linux 6.12.112. Twelve repeated bulk rounds are not
twelve independent machines. All 24 Stealth bulk downloads completed. Paired
unbuffered-versus-buffered changes at 50 MB were +32.96% on US-NY#601 and +25.41%
on US-NY#505, with about 25% less client-process CPU/byte. At 12 MB they were
+41.46% / +20.00%. Direct controls changed substantially during the WAN series;
do not call these fixed hardware limits or pool earlier-day samples as matched
pairs. The [summary](benchmarks/travel-decision-20261105.json) and
[raw evidence archive](benchmarks/travel-decision-20261105.zip) retain failures,
per-round measurements, source/binary fingerprints and cleanup checks.

The current PR keeps mmsg, outer UDP GSO/GRO, compact pools and in-place crypto.
It excludes software TUN segmentation, experimental io_uring, production
parallel crypto and mandatory PGO. Compact reserved capacity is not peak RSS;
in-place crypto's CPU benefit carries a measured memory tradeoff. A two-thread
crypto-primitive speedup is not a measured production VPN speedup. PGO results
vary by protocol and packet size and also optimize the benchmark peer/generator.

The expanded experimental 128-buffer io_uring receiver had a reproduced CQ
overflow liveness bug. The targeted correction and eight successful corrected
WAN downloads belong to that experiment. Neither receiver nor fix is present
in this production PR, and installing liburing is unnecessary. Do not attribute
all historical download failures to this defect. Kernel connection timeouts
also occurred; later readiness retries succeeded, but their root cause remains
unestablished.

The selected native Rust connection type is `tls::ClientConnection`:
`rustls::ClientConnection` on Windows/Apple, `UnbufferedClientConnection` elsewhere.
`tls::client*` supplies it, and custom ordinary-TLS callers pass that type to
`Transport::Tls`. This revises an unreleased Rust helper API; v3.0.5 host-facing
connection functions and generated platform bindings retain their contracts.

The buffered adapter restores the measured bounded Read/Write approach. It
keeps accepted plaintext in rustls across short writes, uses a 64 KiB application
output limit and drives readable traffic independently of blocked ciphertext
writes. Linux/Android retain direct record-to-decoder reception and the existing
unbuffered adapter unchanged. The tests retain explicit unbuffered coverage on
buffered-default hosts, and selected-adapter checks cover TLS 1.2/1.3, certificate
rejection, pressured vectored writes, full ACK queues, key updates, half-close
and independent read progress. The buffered handshake may accept plaintext
before verification; test setup must drive the handshake rather than assume
that write acceptance means it has completed.

Validation of this selection revision is recorded in PR #2's checks. Previous
performance campaigns justify the policy; they are not measurements of this
new commit. Physical-device/provider, complete Plus-function and release
acceptance remain separate gates. No router or installed software is changed
by this source/documentation revision.

Primary API contracts: [rustls 0.23.46 buffered connections](https://docs.rs/rustls/0.23.46/rustls/struct.ConnectionCommon.html),
[unbuffered state machine](https://docs.rs/rustls/0.23.46/rustls/unbuffered/index.html),
[liburing CQ overflow contract](https://github.com/axboe/liburing/blob/master/man/io_uring_cq_has_overflow.3).
