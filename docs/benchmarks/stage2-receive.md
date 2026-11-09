# Stage 2 receive-path evaluation

Baseline: merged PR #6, `02fac0940cdd508a10821048bf1b5a0fe7eed93c`.

**Decision:** retain direct complete-frame extraction and MTU-sized borrowed-TLS prefetch as receive/resource-efficiency improvements. Do not advertise a maximum-download-speed gain. Keep UDP batches at 16 and the existing worker bounds. The maximum-sized prefetch candidate is superseded. This Stage 2 PR is ready for review; Stage 3 requires its own plan review before implementation.

All 504 planned timed downloads across the retained cohorts below are valid. Cohorts are separate allocations/source identities, not a pooled speedup experiment. The [superseded canceled matched-load launch](https://github.com/riKirsti/ProtunNext/actions/runs/42714854574) is retained separately and excluded from these completed-cohort totals.

## First candidate: direct extraction and maximum-sized prefetch buffers

Tested candidate: `a1e158525396a982049a5b212ff17aee3d078726`. The later MTU-sized prefetch correction is a separate candidate below.

All 288/288 planned downloads are valid, with no invalid-data errors. Every one of the 36 workload cells is auxiliary-limited. One native x64 TCP cell additionally fails its control/timing screen. These observations cannot establish maximum client capacity or a repeatable download-speed gain. The 0.85-core auxiliary screen is a measurement-quality rule, not a CPU budget for ProtunNext.

CPU is client process ns per verified downloaded byte. RSS is lifetime client process high-water memory and excludes kernel/auxiliary memory. Positive cost changes mean higher cost. Variants use fixed baseline peer/load binaries, same-binary controls and balanced order. No cells or failed screens are removed from the tables.

### Linux 6.12.115 x64, KVM

[Raw evidence](https://github.com/riKirsti/ProtunNext/actions/runs/42712051992/artifacts/13716019379).

| Workload | Mbps base / candidate | Speed change | CPU ns/B base / candidate | CPU cost change | RSS MiB base / candidate | Screen |
|---|---:|---:|---:|---:|---:|---|
| udp-v4-http-1 | 3391.18 / 3377.12 | -0.41% | 2.288 / 2.303 | +0.65% | 6.23 / 6.09 | usable; auxiliary-limited |
| udp-v4-http-8 | 3063.10 / 3021.23 | -1.37% | 2.603 / 2.642 | +1.52% | 6.47 / 6.85 | usable; auxiliary-limited |
| udp-v6-https-1 | 3076.43 / 3109.84 | +1.09% | 2.511 / 2.507 | -0.16% | 6.16 / 5.95 | usable; auxiliary-limited |
| udp-v6-https-8 | 2905.26 / 2874.33 | -1.06% | 2.671 / 2.712 | +1.53% | 6.04 / 6.20 | usable; auxiliary-limited |
| tcp-v4-http-1 | 3340.55 / 3444.05 | +3.10% | 2.391 / 2.260 | -5.51% | 5.55 / 4.90 | usable; auxiliary-limited |
| tcp-v4-http-8 | 3097.47 / 3219.09 | +3.93% | 2.583 / 2.475 | -4.17% | 5.82 / 5.92 | usable; auxiliary-limited |
| tcp-v6-https-1 | 3215.40 / 3250.45 | +1.09% | 2.486 / 2.401 | -3.38% | 4.36 / 5.29 | usable; auxiliary-limited |
| tcp-v6-https-8 | 3037.85 / 2967.98 | -2.30% | 2.624 / 2.619 | -0.20% | 5.59 / 5.28 | usable; auxiliary-limited |
| stealth-v4-http-1 | 2911.67 / 2924.24 | +0.43% | 2.728 / 2.713 | -0.53% | 4.69 / 5.57 | usable; auxiliary-limited |
| stealth-v4-http-8 | 2726.17 / 2744.66 | +0.68% | 2.932 / 2.913 | -0.65% | 6.68 / 7.04 | usable; auxiliary-limited |
| stealth-v6-https-1 | 2479.57 / 2467.50 | -0.49% | 2.946 / 2.926 | -0.70% | 5.85 / 5.05 | usable; auxiliary-limited |
| stealth-v6-https-8 | 2497.52 / 2500.40 | +0.12% | 3.136 / 3.124 | -0.36% | 6.48 / 6.02 | usable; auxiliary-limited |

### Native x64, Linux 7.0.0-1017-azure

[Raw evidence](https://github.com/riKirsti/ProtunNext/actions/runs/42712048884/artifacts/13717592095).

| Workload | Mbps base / candidate | Speed change | CPU ns/B base / candidate | CPU cost change | RSS MiB base / candidate | Screen |
|---|---:|---:|---:|---:|---:|---|
| udp-v4-http-1 | 2128.40 / 2113.57 | -0.70% | 3.350 / 3.364 | +0.41% | 9.84 / 10.07 | usable; auxiliary-limited |
| udp-v4-http-8 | 1964.79 / 1917.79 | -2.39% | 3.592 / 3.674 | +2.28% | 11.26 / 10.45 | usable; auxiliary-limited |
| udp-v6-https-1 | 2092.34 / 2132.20 | +1.91% | 3.613 / 3.553 | -1.67% | 9.62 / 9.88 | usable; auxiliary-limited |
| udp-v6-https-8 | 1946.47 / 1942.93 | -0.18% | 3.855 / 3.848 | -0.18% | 10.69 / 10.69 | usable; auxiliary-limited |
| tcp-v4-http-1 | 2217.87 / 2154.91 | -2.84% | 3.334 / 3.367 | +1.00% | 9.75 / 9.38 | usable; auxiliary-limited |
| tcp-v4-http-8 | 2055.23 / 2074.13 | +0.92% | 3.577 / 3.534 | -1.22% | 10.93 / 11.02 | usable; auxiliary-limited |
| tcp-v6-https-1 | 2118.39 / 2168.04 | +2.34% | 3.776 / 3.555 | -5.84% | 10.89 / 9.43 | inconclusive-control/timing; auxiliary-limited |
| tcp-v6-https-8 | 2024.03 / 2046.19 | +1.09% | 3.951 / 3.746 | -5.20% | 10.89 / 10.57 | usable; auxiliary-limited |
| stealth-v4-http-1 | 2044.59 / 2039.49 | -0.25% | 3.589 / 3.598 | +0.25% | 9.72 / 11.11 | usable; auxiliary-limited |
| stealth-v4-http-8 | 1894.29 / 1928.22 | +1.79% | 3.855 / 3.790 | -1.69% | 11.14 / 12.24 | usable; auxiliary-limited |
| stealth-v6-https-1 | 2077.73 / 2070.53 | -0.35% | 3.748 / 3.755 | +0.19% | 9.29 / 10.27 | usable; auxiliary-limited |
| stealth-v6-https-8 | 1907.85 / 1863.20 | -2.34% | 4.032 / 4.101 | +1.71% | 10.95 / 11.47 | usable; auxiliary-limited |

### Native ARM64, Linux 7.0.0-1017-azure

[Raw evidence](https://github.com/riKirsti/ProtunNext/actions/runs/42712048884/artifacts/13716598684).

| Workload | Mbps base / candidate | Speed change | CPU ns/B base / candidate | CPU cost change | RSS MiB base / candidate | Screen |
|---|---:|---:|---:|---:|---:|---|
| udp-v4-http-1 | 2396.89 / 2396.58 | -0.01% | 3.338 / 3.338 | +0.01% | 10.02 / 10.17 | usable; auxiliary-limited |
| udp-v4-http-8 | 2383.36 / 2411.46 | +1.18% | 3.355 / 3.317 | -1.14% | 10.98 / 10.40 | usable; auxiliary-limited |
| udp-v6-https-1 | 2584.93 / 2609.56 | +0.95% | 3.095 / 3.066 | -0.94% | 10.51 / 9.53 | usable; auxiliary-limited |
| udp-v6-https-8 | 2516.27 / 2557.62 | +1.64% | 3.179 / 3.128 | -1.59% | 10.20 / 10.10 | usable; auxiliary-limited |
| tcp-v4-http-1 | 2474.67 / 2509.35 | +1.40% | 3.233 / 3.188 | -1.38% | 9.88 / 10.41 | usable; auxiliary-limited |
| tcp-v4-http-8 | 2404.01 / 2434.08 | +1.25% | 3.328 / 3.286 | -1.24% | 10.72 / 10.77 | usable; auxiliary-limited |
| tcp-v6-https-1 | 2589.92 / 2670.70 | +3.12% | 3.089 / 2.995 | -3.03% | 9.84 / 9.87 | usable; auxiliary-limited |
| tcp-v6-https-8 | 2524.93 / 2561.81 | +1.46% | 3.168 / 3.123 | -1.44% | 10.07 / 10.41 | usable; auxiliary-limited |
| stealth-v4-http-1 | 2096.59 / 2102.71 | +0.29% | 3.816 / 3.805 | -0.29% | 10.76 / 11.39 | usable; auxiliary-limited |
| stealth-v4-http-8 | 2103.22 / 2130.81 | +1.31% | 3.804 / 3.754 | -1.30% | 10.74 / 11.35 | usable; auxiliary-limited |
| stealth-v6-https-1 | 2224.90 / 2264.08 | +1.76% | 3.596 / 3.533 | -1.73% | 11.10 / 11.15 | usable; auxiliary-limited |
| stealth-v6-https-8 | 2153.69 / 2196.90 | +2.01% | 3.715 / 3.641 | -1.97% | 10.48 / 11.34 | usable; auxiliary-limited |

## MTU-sized buffering and matched-load comparison

The first prefetch implementation requested maximum-sized buffers even for ordinary MTU-sized frames. The revised candidate selects capacity from the next framing header and preserves the compact pool; split headers use buffered handling. Tests verify queue/pool bounds and unchanged total reservation for a complete burst of small frames.

Current candidate: `837ac15e53645e6f6179d876a96a66da3ff02485`. Matched-load tests use 200 Mbps aggregate server pacing and an exact 128 MiB verified payload per sample, with the same controls/order. The analyzer verifies the offered-rate identity and sustained rate, and explicitly excludes paced samples from maximum-capacity ranking. This is a test workload, not a production speed/resource limit.

The 192/192 matched-load downloads pass A/A and timing screens; all 24 cells have auxiliary headroom. Their configured rate prevents any maximum-capacity conclusion. Native ARM64 TCP CPU/byte falls 0.89–3.21% and Stealth CPU/byte falls 0.40–4.14% across the four cells per protocol. Native x64 CPU changes are small/mixed (TCP -1.15% to +1.18%, Stealth -1.84% to +1.19%); no universal CPU win or formal confidence interval is claimed from one allocation per architecture.

Stealth peak client RSS falls in all matched-load cells: 25.95–34.38% on native x64 and 22.32–37.17% on native ARM64. The unpaced eight-flow IPv4 HTTP Stealth case separately reproduces the memory reduction on Linux 6.12 (6.98 to 3.71 MiB, -46.79%) and 6.18 (7.09 to 3.51 MiB, approximately -50.5%). Those focused cases remain helper-limited. This is resident high-water process memory including setup/warmup, not an allocated-heap or kernel-memory claim.

### 200 Mbps native x64

[Raw evidence](https://github.com/riKirsti/ProtunNext/actions/runs/42715341710/artifacts/13718047504).

| Workload | Mbps base / candidate | Speed change | CPU ns/B base / candidate | CPU cost change | RSS MiB base / candidate | Screen |
|---|---:|---:|---:|---:|---:|---|
| udp-v4-http-1 | 200.08 / 200.08 | +0.00% | 2.637 / 2.605 | -1.19% | 9.61 / 9.66 | usable; matched-load only |
| udp-v4-http-8 | 200.70 / 200.71 | +0.00% | 2.858 / 2.837 | -0.72% | 11.33 / 11.46 | usable; matched-load only |
| udp-v6-https-1 | 200.07 / 200.07 | +0.00% | 2.952 / 2.976 | +0.81% | 8.94 / 8.62 | usable; matched-load only |
| udp-v6-https-8 | 200.70 / 200.70 | +0.00% | 3.143 / 3.133 | -0.31% | 10.95 / 11.12 | usable; matched-load only |
| tcp-v4-http-1 | 200.08 / 200.08 | -0.00% | 2.636 / 2.605 | -1.15% | 9.73 / 9.05 | usable; matched-load only |
| tcp-v4-http-8 | 200.70 / 200.71 | +0.00% | 2.849 / 2.852 | +0.11% | 11.26 / 11.28 | usable; matched-load only |
| tcp-v6-https-1 | 200.07 / 200.07 | +0.00% | 2.995 / 3.031 | +1.18% | 8.59 / 8.09 | usable; matched-load only |
| tcp-v6-https-8 | 200.70 / 200.70 | +0.00% | 3.144 / 3.120 | -0.75% | 9.51 / 9.57 | usable; matched-load only |
| stealth-v4-http-1 | 200.08 / 200.08 | +0.00% | 2.928 / 2.874 | -1.84% | 10.09 / 7.13 | usable; matched-load only |
| stealth-v4-http-8 | 200.70 / 200.70 | +0.00% | 3.013 / 3.049 | +1.19% | 11.45 / 8.48 | usable; matched-load only |
| stealth-v6-https-1 | 200.07 / 200.07 | +0.00% | 3.082 / 3.110 | +0.91% | 9.05 / 5.94 | usable; matched-load only |
| stealth-v6-https-8 | 200.69 / 200.70 | +0.00% | 3.352 / 3.375 | +0.71% | 10.69 / 7.10 | usable; matched-load only |

### 200 Mbps native ARM64

[Raw evidence](https://github.com/riKirsti/ProtunNext/actions/runs/42715341710/artifacts/13717832765).

| Workload | Mbps base / candidate | Speed change | CPU ns/B base / candidate | CPU cost change | RSS MiB base / candidate | Screen |
|---|---:|---:|---:|---:|---:|---|
| udp-v4-http-1 | 200.08 / 200.08 | -0.00% | 3.686 / 3.698 | +0.33% | 8.65 / 8.73 | usable; matched-load only |
| udp-v4-http-8 | 200.70 / 200.70 | +0.00% | 3.499 / 3.522 | +0.64% | 10.74 / 10.51 | usable; matched-load only |
| udp-v6-https-1 | 200.07 / 200.07 | +0.00% | 3.776 / 3.740 | -0.97% | 8.36 / 8.95 | usable; matched-load only |
| udp-v6-https-8 | 200.70 / 200.70 | -0.00% | 3.348 / 3.336 | -0.35% | 10.62 / 10.27 | usable; matched-load only |
| tcp-v4-http-1 | 200.08 / 200.08 | +0.00% | 3.545 / 3.508 | -1.05% | 10.10 / 9.42 | usable; matched-load only |
| tcp-v4-http-8 | 200.70 / 200.70 | +0.00% | 3.382 / 3.273 | -3.21% | 10.85 / 10.72 | usable; matched-load only |
| tcp-v6-https-1 | 200.07 / 200.08 | +0.00% | 3.642 / 3.609 | -0.89% | 7.89 / 7.84 | usable; matched-load only |
| tcp-v6-https-8 | 200.70 / 200.70 | +0.00% | 3.314 / 3.266 | -1.46% | 8.49 / 8.47 | usable; matched-load only |
| stealth-v4-http-1 | 200.07 / 200.07 | -0.00% | 4.075 / 4.059 | -0.40% | 10.31 / 7.22 | usable; matched-load only |
| stealth-v4-http-8 | 200.69 / 200.70 | +0.00% | 3.807 / 3.738 | -1.81% | 10.41 / 8.09 | usable; matched-load only |
| stealth-v6-https-1 | 200.07 / 200.07 | +0.00% | 4.171 / 3.999 | -4.14% | 9.08 / 5.71 | usable; matched-load only |
| stealth-v6-https-8 | 200.69 / 200.69 | +0.00% | 3.794 / 3.741 | -1.39% | 8.60 / 5.91 | usable; matched-load only |

### Focused unpaced Linux 6.12.115 Stealth

[Raw evidence](https://github.com/riKirsti/ProtunNext/actions/runs/42715345739/artifacts/13717478333).

| Workload | Mbps base / candidate | Speed change | CPU ns/B base / candidate | CPU cost change | RSS MiB base / candidate | Screen |
|---|---:|---:|---:|---:|---:|---|
| stealth-v4-http-8 | 2072.40 / 2073.78 | +0.07% | 3.860 / 3.856 | -0.09% | 6.98 / 3.71 | usable; auxiliary-limited |

### Focused unpaced Linux 6.18.60 Stealth

[Raw evidence](https://github.com/riKirsti/ProtunNext/actions/runs/42716120468/artifacts/13718322046).

| Workload | Mbps base / candidate | Speed change | CPU ns/B base / candidate | CPU cost change | RSS MiB base / candidate | Screen |
|---|---:|---:|---:|---:|---:|---|
| stealth-v4-http-8 | 1985.90 / 2037.62 | +2.60% | 4.004 / 3.919 | -2.12% | 7.09 / 3.51 | usable; auxiliary-limited |

### Focused unpaced Linux 6.18.60 TCP

[Raw evidence](https://github.com/riKirsti/ProtunNext/actions/runs/42716123697/artifacts/13717314818).

| Workload | Mbps base / candidate | Speed change | CPU ns/B base / candidate | CPU cost change | RSS MiB base / candidate | Screen |
|---|---:|---:|---:|---:|---:|---|
| tcp-v4-http-8 | 3136.51 / 3097.45 | -1.25% | 2.548 / 2.555 | +0.26% | 5.79 / 6.08 | usable; auxiliary-limited |

## Selection and validation

| Candidate | Decision | Evidence / limit |
|---|---|---|
| Direct complete-frame extraction | Retain | Avoids the extra ring copy; native ARM64 CPU reductions are observed at matched load; x64 CPU results are mixed |
| Maximum-sized borrowed-TLS prefetch | Superseded | Unnecessary large-buffer promotion increased memory use |
| MTU-sized borrowed-TLS prefetch | Retain | Preserves bounded ownership and lowers Stealth RSS across native x64/ARM64 and both tested LTS kernels |
| Larger budgets, worker queues and pools | Keep current bounds | No persistent TUN pressure is observed; queue occupancy alone does not prove that increasing capacities would help |
| 32-packet UDP batches | Keep 16 | Prior PR #6 experiment remains inconclusive; UDP is an unchanged control here |

### Recommendation by target

| Target | Measured result | Recommendation |
|---|---|---|
| Linux 6.12 x64 KVM | Stealth eight-flow RSS -46.79%; speed/CPU essentially tied in the focused final test | Retain the resource-efficiency change; no maximum-speed claim |
| Linux 6.18 x64 KVM | Stealth eight-flow RSS approximately -50.5%; observed speed +2.60%, CPU/byte about -2%; helper-limited | Retain the resource-efficiency change; speed remains descriptive |
| Native ARM64 Linux 7 | At 200 Mbps, TCP CPU/byte -0.89% to -3.21%, Stealth -0.40% to -4.14%; Stealth RSS -22.32% to -37.17% | Retain; supported-LTS ARM64 and device performance remain unverified |
| Native x64 Linux 7 | At 200 Mbps, CPU changes small/mixed; Stealth RSS -25.95% to -34.38% | Retain for memory efficiency; no general x64 CPU-speed claim |
| Windows / Apple / Android | Cross-platform functional/build checks pass; no device download measurements in this stage | Shared logic is regression-tested; no platform download-speed claim |

Cumulative first-candidate samples contain at most 25 pool-pressure events per run and no TUN-backpressure events; worker queue high-water marks reach 63. The focused final Linux 6.12 sample cohort has no pool-pressure events. Increasing the processing budget alone cannot remove the existing 63-packet queue guards. No larger allocation is accepted or rejected on a fabricated resource ceiling.

[All nine platform jobs passed](https://github.com/riKirsti/ProtunNext/actions/runs/42715241805) on production code `837ac15e53645e6f6179d876a96a66da3ff02485`. [Native probe checks](https://github.com/riKirsti/ProtunNext/actions/runs/42715341710) passed 48 injected Rust tests per architecture, including fragment and prediction transitions, wraparound, malformed frames/reset, output and pool pressure, prefetched ownership/cleanup and compact reservation. The local Windows all-target/all-feature suite also passes, including real authenticated TCP/TLS, 12 TLS regressions, key updates, backpressure and the established-session allocation check.

The final analyzer adds a negative control rejecting an undeclared rate limit in an otherwise unpaced plan. All 23 Python analyzer tests pass locally. All 504 retained samples are re-analyzed with the stricter rate-identity check and remain valid. Subsequent evidence and analyzer-only changes do not change the tested production Rust.

## Scope and remaining evidence

UDP batching remains 16. Worker budgets, worker queue limits and pool count remain unchanged. The new ready queue is capped at 64 and draws from the same bounded pool, leaving two buffers available to the worker/control path. It never waits to fill a batch. TUN coalescing and parallel crypto belong to later stages.

Windows local functional/TLS tests pass; native builds/contracts are regression evidence, not Windows/Android/Apple download-performance evidence. Physical NIC/device, WAN, matched latency, upload/duplex and sustained thermal acceptance are not established by these loopback cohorts. No router settings were changed and no release is created.

Follow the [stage roadmap](../performance-stages.md). Review Stage 3 against the agreed plan before any Stage 3 implementation starts.
