# Real-download baseline and receive-path profiles

PR #5 is measurement infrastructure. Shipped source and dependencies match merged PR #4; no production speedup is claimed. Both archives use the same injected fixture and diagnostic overlays. Runtime fingerprint: `55fd65e17d7a7de1d866dacd3268e05e9e12a8b521ec896d61df3e14e3bb0c82`.

Each normal-release sample uses real inner kernel TCP through the production TUN/Connection worker: 2 GiB body plus 4 MiB warmup per flow, exact byte verification, 1/8 flows, IPv4 HTTP / IPv6 HTTPS, UDP / TCP / Stealth. Four A/A controls and four ABBA/BAAB samples per cell. The two variants contain identical runtime sources, so their differences describe measurement variation.

CPU 100% means one core of client process time including synchronous kernel calls. RSS is lifetime process high-water memory; it excludes kernel buffers and auxiliary processes. Tables use geometric means of valid same-runtime samples, not improvements. No missing row is imputed. Partial-cell numbers remain descriptive only.

Capacity eligibility requires all samples complete, A/A throughput and CPU/byte within 5%, measured duration >=2 s, interval skew <=2%, and no peer wire, server/control or load process at >=0.85 core. This is an engineering screen, not statistical confidence or proof of no bottleneck.

Validation: 20 Python analyzer tests and 39 injected Rust tests pass. The corrected Linux 6.18 full rerun completed 96/96 downloads, and the focused affected-case rerun completed 8/8 on each of the four targets (32/32). One ARM64 job failed after its downloads at profiler installation because Ubuntu's generic tools metapackage referenced an unavailable dependency; its valid downloads are retained and profiler package selection was corrected separately.

## Representative single-flow IPv4 HTTP baseline

| Target | Transport | Download Mbps | Client CPU % | CPU ns/B | Peak RSS MiB | Capacity eligible |
|---|---|---:|---:|---:|---:|---|
| Linux 6.12.115 x64, KVM | udp | 2842.5 | 99.4 | 2.797 | 5.69 | yes |
| Linux 6.12.115 x64, KVM | tcp | 3015.9 | 100.0 | 2.652 | 5.16 | yes |
| Linux 6.12.115 x64, KVM | stealth | 2853.4 | 100.0 | 2.803 | 6.71 | yes |
| Linux 6.18.60 x64, KVM | udp | 3332.0 | 99.8 | 2.396 | 5.99 | no: auxiliary limited / screen failed |
| Linux 6.18.60 x64, KVM | tcp | 3271.7 | 100.0 | 2.445 | 5.93 | no: auxiliary limited / screen failed |
| Linux 6.18.60 x64, KVM | stealth | 3124.4 | 100.0 | 2.560 | 6.84 | no: auxiliary limited / screen failed |
| Linux 7.0.0-1017-azure x64, native CI | udp | 3284.7 | 95.9 | 2.335 | 10.64 | no: auxiliary limited / screen failed |
| Linux 7.0.0-1017-azure x64, native CI | tcp | 3283.6 | 98.6 | 2.402 | 10.23 | no: auxiliary limited / screen failed |
| Linux 7.0.0-1017-azure x64, native CI | stealth | 3060.5 | 97.2 | 2.540 | 10.00 | no: auxiliary limited / screen failed |
| Linux 7.0.0-1017-azure ARM64, native CI | udp | 2431.6 | 99.9 | 3.286 | 10.22 | no: auxiliary limited / screen failed |
| Linux 7.0.0-1017-azure ARM64, native CI | tcp | 2462.6 | 100.0 | 3.249 | 10.17 | no: auxiliary limited / screen failed |
| Linux 7.0.0-1017-azure ARM64, native CI | stealth | 2126.3 | 100.0 | 3.762 | 10.52 | no: auxiliary limited / screen failed |

Native CI figures are auxiliary limited and cannot rank client maximum capacity. Do not compare targets as a controlled architecture/kernel experiment. They are different allocations. No router, physical NIC, Proton WAN, RTT/loss or thermal acceptance is represented.

## Retained cohorts and all workload cells

The original full campaign produced 382 valid timed downloads from 384 planned samples. Two failed Stealth IPv6 HTTPS/8-flow samples exposed a test-peer Rustls backpressure adapter defect. Its nonempty zero-byte write is now translated into WouldBlock, and a real TLS regression verifies zero-capacity rejection and exact payload delivery after restoring capacity. Original failures remain below; corrected cohorts are separate. The earlier IPv6 multicast/crossed-handshake fixture failure also has a real-session negative control and peer AllowedIPs filter regression.

Early fixture-development data (BusyBox option incompatibility, IPv6 readiness, too-short 512 MiB transfers and missing peer AllowedIPs filtering) are unsuitable for production-performance conclusions. They were not selected into this baseline.

### Linux 6.12.115 x64, KVM — initial full cohort

[Raw evidence](https://github.com/riKirsti/ProtunNext/actions/runs/42140088347/artifacts/13466729158). Valid timed samples: 96/96. Candidate archive: `a182fa358b10cf35cf5ea49884c728a76b403fd9`. Same runtime sources: True. Capacity-eligible cells: 12/12.

| Workload | Valid samples | Download Mbps | Client CPU % | CPU ns/B | Peak RSS MiB | Validity | Capacity eligible |
|---|---:|---:|---:|---:|---:|---|---|
| udp-v4-http-1 | 8/8 | 2842.5 | 99.4 | 2.797 | 5.69 | usable | True |
| udp-v4-http-8 | 8/8 | 2399.8 | 99.9 | 3.330 | 6.55 | usable | True |
| udp-v6-https-1 | 8/8 | 2718.9 | 99.8 | 2.937 | 5.00 | usable | True |
| udp-v6-https-8 | 8/8 | 2440.5 | 99.9 | 3.276 | 6.06 | usable | True |
| tcp-v4-http-1 | 8/8 | 3015.9 | 100.0 | 2.652 | 5.16 | usable | True |
| tcp-v4-http-8 | 8/8 | 2706.0 | 100.0 | 2.956 | 5.95 | usable | True |
| tcp-v6-https-1 | 8/8 | 2761.0 | 100.0 | 2.897 | 4.77 | usable | True |
| tcp-v6-https-8 | 8/8 | 2551.7 | 100.0 | 3.135 | 5.87 | usable | True |
| stealth-v4-http-1 | 8/8 | 2853.4 | 100.0 | 2.803 | 6.71 | usable | True |
| stealth-v4-http-8 | 8/8 | 2546.7 | 100.0 | 3.141 | 6.74 | usable | True |
| stealth-v6-https-1 | 8/8 | 2514.2 | 99.1 | 3.152 | 6.54 | usable | True |
| stealth-v6-https-8 | 8/8 | 2450.3 | 99.9 | 3.263 | 6.85 | usable | True |

The artifact summary also retains separate baseline/candidate values, controls, identities, raw logs and error classifications.

### Linux 6.18.60 x64, KVM — initial full cohort

[Raw evidence](https://github.com/riKirsti/ProtunNext/actions/runs/42140088347/artifacts/13468223459). Valid timed samples: 95/96. Candidate archive: `a182fa358b10cf35cf5ea49884c728a76b403fd9`. Same runtime sources: True. Capacity-eligible cells: 1/12.

| Workload | Valid samples | Download Mbps | Client CPU % | CPU ns/B | Peak RSS MiB | Validity | Capacity eligible |
|---|---:|---:|---:|---:|---:|---|---|
| udp-v4-http-1 | 8/8 | 3332.0 | 99.8 | 2.396 | 5.99 | usable; auxiliary limited | False |
| udp-v4-http-8 | 8/8 | 3042.3 | 99.9 | 2.627 | 6.64 | usable; auxiliary limited | False |
| udp-v6-https-1 | 8/8 | 3146.4 | 99.8 | 2.539 | 4.97 | usable; auxiliary limited | False |
| udp-v6-https-8 | 8/8 | 2905.9 | 99.9 | 2.751 | 5.99 | usable; auxiliary limited | False |
| tcp-v4-http-1 | 8/8 | 3271.7 | 100.0 | 2.445 | 5.93 | usable; auxiliary limited | False |
| tcp-v4-http-8 | 8/8 | 3126.4 | 100.0 | 2.559 | 6.05 | usable; auxiliary limited | False |
| tcp-v6-https-1 | 8/8 | 2942.8 | 100.0 | 2.718 | 5.61 | inconclusive-control/timing | False |
| tcp-v6-https-8 | 8/8 | 2798.2 | 100.0 | 2.859 | 6.00 | usable | True |
| stealth-v4-http-1 | 8/8 | 3124.4 | 100.0 | 2.560 | 6.84 | usable; auxiliary limited | False |
| stealth-v4-http-8 | 8/8 | 3024.9 | 100.0 | 2.644 | 6.91 | usable; auxiliary limited | False |
| stealth-v6-https-1 | 8/8 | 2837.7 | 100.0 | 2.819 | 6.39 | usable; auxiliary limited | False |
| stealth-v6-https-8 | 7/8 | 2727.9 | 100.0 | 2.932 | 6.94 | invalid/incomplete | False |

The artifact summary also retains separate baseline/candidate values, controls, identities, raw logs and error classifications.

### Linux 7.0.0-1017-azure x64, native CI — initial full cohort

[Raw evidence](https://github.com/riKirsti/ProtunNext/actions/runs/42140088347/artifacts/13467744076). Valid timed samples: 96/96. Candidate archive: `a182fa358b10cf35cf5ea49884c728a76b403fd9`. Same runtime sources: True. Capacity-eligible cells: 0/12.

| Workload | Valid samples | Download Mbps | Client CPU % | CPU ns/B | Peak RSS MiB | Validity | Capacity eligible |
|---|---:|---:|---:|---:|---:|---|---|
| udp-v4-http-1 | 8/8 | 3284.7 | 95.9 | 2.335 | 10.64 | usable; auxiliary limited | False |
| udp-v4-http-8 | 8/8 | 3077.0 | 95.0 | 2.471 | 11.43 | usable; auxiliary limited | False |
| udp-v6-https-1 | 8/8 | 3185.7 | 98.2 | 2.467 | 10.39 | usable; auxiliary limited | False |
| udp-v6-https-8 | 8/8 | 3032.6 | 97.4 | 2.569 | 11.07 | usable; auxiliary limited | False |
| tcp-v4-http-1 | 8/8 | 3283.6 | 98.6 | 2.402 | 10.23 | usable; auxiliary limited | False |
| tcp-v4-http-8 | 8/8 | 3189.0 | 97.9 | 2.456 | 11.35 | usable; auxiliary limited | False |
| tcp-v6-https-1 | 8/8 | 3220.8 | 100.0 | 2.484 | 10.36 | usable; auxiliary limited | False |
| tcp-v6-https-8 | 8/8 | 3117.0 | 99.7 | 2.559 | 11.03 | usable; auxiliary limited | False |
| stealth-v4-http-1 | 8/8 | 3060.5 | 97.2 | 2.540 | 10.00 | usable; auxiliary limited | False |
| stealth-v4-http-8 | 8/8 | 2885.0 | 96.6 | 2.680 | 11.68 | usable; auxiliary limited | False |
| stealth-v6-https-1 | 8/8 | 3018.7 | 99.6 | 2.640 | 9.80 | usable; auxiliary limited | False |
| stealth-v6-https-8 | 8/8 | 2893.1 | 98.8 | 2.732 | 11.21 | usable; auxiliary limited | False |

The artifact summary also retains separate baseline/candidate values, controls, identities, raw logs and error classifications.

### Linux 7.0.0-1017-azure ARM64, native CI — initial full cohort

[Raw evidence](https://github.com/riKirsti/ProtunNext/actions/runs/42140088347/artifacts/13467129755). Valid timed samples: 95/96. Candidate archive: `a182fa358b10cf35cf5ea49884c728a76b403fd9`. Same runtime sources: True. Capacity-eligible cells: 0/12.

| Workload | Valid samples | Download Mbps | Client CPU % | CPU ns/B | Peak RSS MiB | Validity | Capacity eligible |
|---|---:|---:|---:|---:|---:|---|---|
| udp-v4-http-1 | 8/8 | 2431.6 | 99.9 | 3.286 | 10.22 | usable; auxiliary limited | False |
| udp-v4-http-8 | 8/8 | 2429.7 | 100.0 | 3.292 | 10.92 | usable; auxiliary limited | False |
| udp-v6-https-1 | 8/8 | 2630.5 | 100.0 | 3.041 | 9.71 | usable; auxiliary limited | False |
| udp-v6-https-8 | 8/8 | 2579.0 | 100.0 | 3.102 | 10.58 | usable; auxiliary limited | False |
| tcp-v4-http-1 | 8/8 | 2462.6 | 100.0 | 3.249 | 10.17 | usable; auxiliary limited | False |
| tcp-v4-http-8 | 8/8 | 2431.4 | 100.0 | 3.290 | 10.19 | usable; auxiliary limited | False |
| tcp-v6-https-1 | 8/8 | 2622.6 | 100.0 | 3.050 | 9.87 | usable; auxiliary limited | False |
| tcp-v6-https-8 | 8/8 | 2525.0 | 100.0 | 3.168 | 10.08 | usable; auxiliary limited | False |
| stealth-v4-http-1 | 8/8 | 2126.3 | 100.0 | 3.762 | 10.52 | usable; auxiliary limited | False |
| stealth-v4-http-8 | 8/8 | 2093.0 | 100.0 | 3.822 | 10.94 | usable; auxiliary limited | False |
| stealth-v6-https-1 | 8/8 | 2231.9 | 100.0 | 3.584 | 10.35 | usable; auxiliary limited | False |
| stealth-v6-https-8 | 7/8 | 2178.0 | 100.0 | 3.673 | 10.50 | invalid/incomplete | False |

The artifact summary also retains separate baseline/candidate values, controls, identities, raw logs and error classifications.

### Linux 6.18.60 x64, KVM — corrected peer, separate full rerun

[Raw evidence](https://github.com/riKirsti/ProtunNext/actions/runs/42142606408/artifacts/13468807946). Valid timed samples: 96/96. Candidate archive: `c6e03a9305c950cbeaa160193affd4ea9a58440d`. Same runtime sources: True. Capacity-eligible cells: 3/12.

| Workload | Valid samples | Download Mbps | Client CPU % | CPU ns/B | Peak RSS MiB | Validity | Capacity eligible |
|---|---:|---:|---:|---:|---:|---|---|
| udp-v4-http-1 | 8/8 | 3084.2 | 99.6 | 2.584 | 6.51 | usable; auxiliary limited | False |
| udp-v4-http-8 | 8/8 | 2826.1 | 99.9 | 2.828 | 6.59 | usable; auxiliary limited | False |
| udp-v6-https-1 | 8/8 | 2924.7 | 99.8 | 2.731 | 5.13 | usable; auxiliary limited | False |
| udp-v6-https-8 | 8/8 | 2606.1 | 99.9 | 3.068 | 5.91 | usable; auxiliary limited | False |
| tcp-v4-http-1 | 8/8 | 3114.1 | 100.0 | 2.568 | 6.12 | usable; auxiliary limited | False |
| tcp-v4-http-8 | 8/8 | 2988.4 | 100.0 | 2.676 | 6.06 | usable; auxiliary limited | False |
| tcp-v6-https-1 | 8/8 | 2895.3 | 100.0 | 2.763 | 6.05 | usable | True |
| tcp-v6-https-8 | 8/8 | 2756.1 | 100.0 | 2.902 | 5.97 | usable | True |
| stealth-v4-http-1 | 8/8 | 2996.5 | 100.0 | 2.669 | 6.68 | usable; auxiliary limited | False |
| stealth-v4-http-8 | 8/8 | 2874.7 | 100.0 | 2.782 | 6.85 | usable; auxiliary limited | False |
| stealth-v6-https-1 | 8/8 | 2712.0 | 100.0 | 2.949 | 6.66 | usable; auxiliary limited | False |
| stealth-v6-https-8 | 8/8 | 2660.7 | 100.0 | 3.006 | 6.93 | usable | True |

The artifact summary also retains separate baseline/candidate values, controls, identities, raw logs and error classifications.

### Linux 6.18.60 x64, KVM — corrected peer, focused rerun

[Raw evidence](https://github.com/riKirsti/ProtunNext/actions/runs/42143801058/artifacts/13469601239). Valid timed samples: 8/8. Candidate archive: `e58e22b9e7fc8eddeea100a8d7cb0ebef34d1b6f`. Same runtime sources: True. Capacity-eligible cells: 0/1.

| Workload | Valid samples | Download Mbps | Client CPU % | CPU ns/B | Peak RSS MiB | Validity | Capacity eligible |
|---|---:|---:|---:|---:|---:|---|---|
| stealth-v6-https-8 | 8/8 | 2399.0 | 99.9 | 3.331 | 6.85 | usable; auxiliary limited | False |

The artifact summary also retains separate baseline/candidate values, controls, identities, raw logs and error classifications.

### Linux 7.0.0-1017-azure ARM64, native CI — corrected peer, focused rerun

[Raw evidence](https://github.com/riKirsti/ProtunNext/actions/runs/42143801058/artifacts/13469172610). Valid timed samples: 8/8. Candidate archive: `e58e22b9e7fc8eddeea100a8d7cb0ebef34d1b6f`. Same runtime sources: True. Capacity-eligible cells: 0/1.

| Workload | Valid samples | Download Mbps | Client CPU % | CPU ns/B | Peak RSS MiB | Validity | Capacity eligible |
|---|---:|---:|---:|---:|---:|---|---|
| stealth-v6-https-8 | 8/8 | 2299.6 | 100.0 | 3.478 | 10.78 | usable; auxiliary limited | False |

The artifact summary also retains separate baseline/candidate values, controls, identities, raw logs and error classifications.

### Linux 6.12.115 x64, KVM — corrected peer, focused rerun

[Raw evidence](https://github.com/riKirsti/ProtunNext/actions/runs/42143801058/artifacts/13469706432). Valid timed samples: 8/8. Candidate archive: `e58e22b9e7fc8eddeea100a8d7cb0ebef34d1b6f`. Same runtime sources: True. Capacity-eligible cells: 0/1.

| Workload | Valid samples | Download Mbps | Client CPU % | CPU ns/B | Peak RSS MiB | Validity | Capacity eligible |
|---|---:|---:|---:|---:|---:|---|---|
| stealth-v6-https-8 | 8/8 | 1535.2 | 99.6 | 5.192 | 6.91 | usable; auxiliary limited | False |

The artifact summary also retains separate baseline/candidate values, controls, identities, raw logs and error classifications.

### Linux 7.0.0-1017-azure x64, native CI — corrected peer, focused rerun

[Raw evidence](https://github.com/riKirsti/ProtunNext/actions/runs/42143801058/artifacts/13468958565). Valid timed samples: 8/8. Candidate archive: `e58e22b9e7fc8eddeea100a8d7cb0ebef34d1b6f`. Same runtime sources: True. Capacity-eligible cells: 0/1.

| Workload | Valid samples | Download Mbps | Client CPU % | CPU ns/B | Peak RSS MiB | Validity | Capacity eligible |
|---|---:|---:|---:|---:|---:|---|---|
| stealth-v6-https-8 | 8/8 | 2312.0 | 97.9 | 3.388 | 11.24 | usable; auxiliary limited | False |

The artifact summary also retains separate baseline/candidate values, controls, identities, raw logs and error classifications.

## Separate CPU profiles

Direct client-process perf captures, including worker threads, on native Linux 7 CI. Separate frame-pointer/debug-info builds; their throughput is excluded from normal-release tables. Values sum self samples by kernel mode or named crypto routine; rounded symbol percentages are approximate. No lost samples were reported. System-wide four-CPU captures are retained separately and include peer/load/background work. Their PID-filtered percentages use a different denominator and are not mixed with these percentages.

| Target | Transport | Kernel-mode self samples % | ChaCha20-Poly1305 open self samples % | AES-GCM decrypt self samples % |
|---|---|---:|---:|---:|
| ARM64 | udp | 54.75 | 35.16 | 0.00 |
| ARM64 | tcp | 55.82 | 32.55 | 0.00 |
| ARM64 | stealth | 48.52 | 30.21 | 8.87 |

[ARM64 raw profiles](https://github.com/riKirsti/ProtunNext/actions/runs/42141106018/artifacts/13467338152) include perf data, reports, source/binary hashes, host kernel and compiler. These earlier profile artifacts lack executable files; the updated workflow retains executables for future captures.

| Target | Transport | Kernel-mode self samples % | ChaCha20-Poly1305 open self samples % | AES-GCM decrypt self samples % |
|---|---|---:|---:|---:|
| x64 | udp | 73.81 | 15.23 | 0.00 |
| x64 | tcp | 74.93 | 14.36 | 0.00 |
| x64 | stealth | 73.23 | 11.51 | 2.68 |

[x64 raw profiles](https://github.com/riKirsti/ProtunNext/actions/runs/42141106018/artifacts/13468495226) include perf data, reports, source/binary hashes, host kernel and compiler. These earlier profile artifacts lack executable files; the updated workflow retains executables for future captures.

Kernel-mode samples include TUN injection, copies, allocation, synchronization and inner TCP/loopback work. They do not establish that TUN writes alone dominate or predict the split on a forwarding router. TUN/coalescing is the next experimental hypothesis, subject to actual device and NIC validation.

## Corrected profiling workflow verification

[All nine cross-platform regression jobs passed](https://github.com/riKirsti/ProtunNext/actions/runs/42143804198) on commit `e58e22b9e7fc8eddeea100a8d7cb0ebef34d1b6f`. Later changes select an available profiler package and document results; shipped source is unchanged. [The corrected profiling-only workflow](https://github.com/riKirsti/ProtunNext/actions/runs/42144360650) passed both native targets on `6efacc19b4801c002912592913437bf9fa30e009`.

This separate allocation repeated all six verified 1 GiB diagnostic downloads per target (process/system capture, three transports). Matching executables are now retained and their hashes were checked against both profile environments. These profiles are additional observations, not pooled with the first allocation or normal-release throughput. The different self-sample split on x64 reinforces that exact percentages are allocation/workload observations, not universal constants.

| Target | Transport | Kernel-mode self samples % | ChaCha20-Poly1305 open self samples % | AES-GCM decrypt self samples % |
|---|---|---:|---:|---:|
| ARM64, repeat allocation | UDP | 57.76 | 30.61 | 0.00 |
| ARM64, repeat allocation | TCP | 59.20 | 28.83 | 0.00 |
| ARM64, repeat allocation | Stealth | 47.08 | 29.59 | 9.12 |
| x64, repeat allocation | UDP | 63.00 | 24.75 | 0.00 |
| x64, repeat allocation | TCP | 65.13 | 22.34 | 0.00 |
| x64, repeat allocation | Stealth | 61.65 | 22.10 | 2.85 |

Raw profiles and matching executables: [ARM64](https://github.com/riKirsti/ProtunNext/actions/runs/42144360650/artifacts/13468428469), [x64](https://github.com/riKirsti/ProtunNext/actions/runs/42144360650/artifacts/13469169038).

## Decision and next implementation

1. Prioritize Linux TUN delivery and kernel-boundary costs. Prototype proper receive coalescing with negotiated offload metadata/checksums, compare against the current ordinary-packet path, and keep one production default chosen by repeatable evidence. Raw TUN caller compatibility must remain intact.
2. On ARM64, also test receive-side parallel crypto with bounded queues and correct replay/key-epoch/ordering behavior. These profiles justify an experiment; they do not establish a gain or a ring defect.
3. Keep Windows/Apple platform-specific measurement ahead of performance promotion. Functional build checks are not download-speed evidence. ARM64 native Linux 7 does not substitute for supported ARM64 Linux 6.12/6.18, Android or Apple tests.
4. Before tuning larger pools/socket buffers, add kernel-memory and queue/loaded-latency accounting. Evaluate CPU/RAM versus sustained download gain without the former fixed experimental resource rejection thresholds.
5. Keep kTLS, whole-pipeline io_uring, zero-copy RX, SIMD/multi-buffer crypto, CPU/NIC placement, native-WireGuard transport bridges and held-out PGO available as measured follow-ups. Nothing in this baseline universally accepts or rejects them.

No router was accessed or modified for this phase. No release is created. Account/Plus/lifecycle/routing features are untouched; their physical-device/live acceptance remains separate from source/build regression checks.

Official implementation references: [WireGuard namespace/socket ownership](https://www.wireguard.com/netns/), [Rustls canonical stream implementation](https://github.com/rustls/rustls/blob/v/0.23.46/rustls/src/stream.rs), [Rustls plaintext writer](https://github.com/rustls/rustls/blob/v/0.23.46/rustls/src/conn.rs), [Linux TUN](https://github.com/torvalds/linux/blob/v6.12/drivers/net/tun.c), [perf record](https://man7.org/linux/man-pages/man1/perf-record.1.html). These establish semantics/capabilities, not an unmeasured ProtunNext gain.
