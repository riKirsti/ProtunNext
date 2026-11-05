# PR4 Travel router retest

**Recommendation: merge PR #4 for the measured Stealth CPU-efficiency improvement. Do not advertise a fixed WAN speed gain or a download-reliability fix.**

Target: Cudy TR3000 v1 ARM64, OpenWrt Linux 6.12.112, current configured server US-NY#187. Baseline is merged PR #3 (ae9c9e8); candidate is PR #4 (0dff661). Identical temporary test drivers are added to separate archives, built in distinct directories using Rust 1.99.1, normal release thin LTO and aarch64-unknown-linux-musl. The only extra RUSTFLAGS select the linker flavor. Actual on-router binary hashes and compiled source identities are verified. No installed runtime is replaced.

## Matched-load client CPU cost

24 valid samples, three complete A/A + ABBA/BAAB blocks. All three A/A controls pass the predeclared 5% screen and every sample achieves 1,000 packets/second within 5%. Payloads are 1,200 bytes; window is 32; warmup is one second and measurement is five seconds. Both variants deliver about 9.60 Mbps by design. The client may use both router cores; the fixed baseline peer and generator share core 0. This is a cost test, not capacity. CPU is client process utilization (100% = one core); peak RSS excludes kernel and auxiliary memory.

| Outer transport | CPU ns/byte PR3 → PR4 | Change | Client CPU % PR3 → PR4 | Peak RSS MiB PR3 → PR4 |
|---|---:|---:|---:|---:|
| udp | 33.42 → 34.07 | +1.94% | 4.01 → 4.09 | 2.29 → 2.35 |
| tcp | 35.02 → 34.50 | -1.48% | 4.20 → 4.14 | 1.73 → 1.73 |
| stealth | 49.55 → 46.81 | -5.52% | 5.95 → 5.62 | 2.35 → 2.29 |

Combined auxiliary utilization never exceeds 0.067 CPU core. UDP/TCP changes are small; no optimization is claimed for those paths. Allocation sizes are unchanged, so small peak-RSS differences are not a memory-saving claim.

## Actual HTTPS downloads

28 planned samples; 26 completed 100 MB downloads. The first two UDP A/A samples failed during HTTPS warmup (three SSLEOFError attempts each), before the 100 MB transfer. Later runs of the same candidate succeeded. No failure cause is established. All TCP and Stealth samples and both kernel WireGuard samples completed. Completed bodies have matching SHA-256, HTTP 206, exact Content-Range and length, certificate validation, bound source interface and tunnel forwarding-counter checks. The URL is a fixed prefix of cdn.kernel.org/pub/linux/kernel/v6.x/linux-6.12.112.tar.xz.

| Transport | PR3 download Mbps | PR4 download Mbps | Client CPU/byte change | Throughput control |
|---|---:|---:|---:|---|
| udp | 175.60 | 192.14 | -3.48% | Incomplete: 2/4 controls failed warmup |
| tcp | 173.84 | 194.80 | -2.63% | A/A +10.92%: exceeds 5% screen |
| stealth | 140.44 | 162.01 | -8.28% | A/A +9.19%: exceeds 5% screen |

Kernel WireGuard UDP reference: 305.41 and 369.81 Mbps (geometric mean 336.07). These are two observations, not a universal ceiling; kernel CPU/RAM is not comparable with userspace-process columns.

Stealth WAN client CPU/byte changes from 40.21 to 36.88 ns/byte. Its CPU-cost control changes -2.33%, within the cost screen. Its observed download increase is not accepted as a clean speedup because the throughput control fails. The 12 MB direct controls changed from 229.36 to 253.85 Mbps.

## Retained high-load cohort and limits

The original 72-sample CI-shaped matrix completed with zero invalid-data errors, but **0/9 cells pass the complete throughput/load screen** on Travel. The client was pinned to core 0 and auxiliaries shared core 1. Same-binary throughput changes range from -30.26% to +35.80%; all CPU/byte controls remain within 2.65%. None of the 10,000 pps cells maintains offered load. These measurements are retained and are not used for capacity ranking. The subsequent separately declared 1,000 pps cohort changes both offered load and CPU allocation to answer a matched-load CPU-cost question; it is not a retry until the high-load controls pass.

Across all synthetic runs, the workload is local IPv4 UDP echo through the real worker and chosen outer protocol. This is one router, one configured Proton server, short samples and no latency/loss/IPv6 acceptance matrix. The test does not isolate all asynchronous kernel work or establish every platform performance limit. Kernel image hashes were not collected from the router.

## General and platform-specific suggestions

- General: retain PR4 as a small, documented buffer-erasure optimization. Preserve erasure extent/timing and current production paths. Profile framing/copy costs next in a separate change; do not infer a crypto-provider defect or select a new backend from these results.
- Travel / Linux 6.12 ARM64: measured matched-load Stealth CPU cost improves 5.52%, with all cost controls passing. Keep existing mmsg, outer GSO/GRO and unbuffered userspace TLS. Maintain kernel WireGuard as the UDP reference; PR4 does not close its architectural gap.
- Linux 6.12/6.18 x64: the preceding actual-kernel CI burst comparisons measured -4.51% / -6.80% Stealth CPU/byte across three allocations each; this router adds different ARM64 evidence, not a replacement for those results.
- Other Linux ARM64, especially 6.18: native ARM64/Linux 7.0 CI is diagnostic only. Do not generalize this Cudy result to every ARM64 machine or claim a physical ARM64 6.18 result.
- Android 16/17 ARM64: the unbuffered code path uses the same helper and native builds pass; physical-device throughput remains unmeasured. Retain current VpnService behavior and routing.
- Windows 25H2/26H2 x64 and Apple 26/27 ARM64: retain the buffered adapter. It is not the optimized production path in PR4; no platform throughput gain is claimed. Cross-platform regression checks pass.

## Restoration and prior evidence

All three phase restoration checks pass. Final comparison against the initial pre-test state confirms Direct mode, saved configurations/account files, installed packages, policy rules, default route, interfaces, socket limits, boot/netifd identity unchanged. All owned stages are absent and no staged binaries remain running. No package, account key or installed runtime was changed. Detailed fingerprints remain private.

CI comparison: https://github.com/riKirsti/ProtunNext/actions/runs/42104739688 ; cross-platform verification: https://github.com/riKirsti/ProtunNext/actions/runs/42104742250 . Official erasure contract: https://github.com/RustCrypto/utils/blob/zeroize-v1.9.1/zeroize/src/barrier.rs . Documentation establishes the API contract; the numbers above come from the retained executions.
