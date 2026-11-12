# Stage 7: sustained temporary Travel PGO tests

Cudy TR3000 v1, two-core Cortex-A53, Linux 6.12.112, 496,724 KiB RAM, saved
Proton endpoint US-NY#544, MTU 1280. Frozen baseline: merged PR #11 `8358224`;
candidate archive: `2288fe4`. Both contain the same production runtime fingerprint
`5d448b4aabf1041e3a13920294314097f88a30eeb9a2d4c799724c2c04112fc7`.
No subsequent report-only change alters the tested runtime.

Both use Rust 1.99.1 / LLVM 23.1.1, ARM64 Linux-musl, release thin LTO,
codegen-units=1, panic=abort, `--no-default-features --features unix`, ring,
the serial receiver and negotiated vnet. Only the candidate consumes its fresh
device-trained PGO profile. No Cortex-A53 tuning flag, installed application,
saved setting or package is changed. Local Agent/account performance is unmeasured.

## Method and verification

Each transport has an independently guarded cohort: four baseline A/A sessions,
then baseline/PGO/PGO/baseline. Every timed session uses one continuously running
tunnel worker for **four consecutive 100,000,000-byte ranged HTTPS fetches**.
These are four sequential TCP/TLS connections, not one persistent 400 MB TCP
stream. Each part verifies HTTP 206, exact Content-Range, length and SHA-256
`2aa720bbba4fe59c980d62a1763575f43ebf214f28a5af2beadb970865d001b0`
from the fixed linux-6.12.112.tar.xz CDN prefix. No aggregate payload SHA is
invented: all four part hashes and byte counts are retained. Forward counters
prove the tunnel path. Direct checks use single 100 MB fetches, outside timing.

CPU is whole-process CPU time over the complete timed session, 100% = one core;
RSS is lifetime peak process MiB including setup/warmup, excluding kernel/other
processes. CPU ns/byte accounts for unequal rates. WAN Mbps includes all four
TCP/TLS setups and laptop verification work; it is not native-loopback capacity.
Summary values are geometric means of two balanced observations per build.
IPv6, concurrent evaluation flows, other devices/endpoints and long thermal/loss
behavior remain unmeasured. These longer sessions are still bounded tests.

Six fresh training cases use all transports with two/four concurrent verified
12 MB HTTPS flows, distinct from evaluation's sequential single-flow pattern.
The first UDP attempt has failed warmups; its successful retry and other cases
verify. Only successful-case counters are merged with matching LLVM tools.
Final PGO has no generation instrumentation, passes linked Drop/recycle checks
and rejects an isolated missing-wipe control. No insecure control is deployed.
Training cleanup and all twelve independent checks pass. Instrumented data is
never a speed result. Build/profile/source hashes are retained below.

## Observed comparisons

| Transport | CPU % B → PGO | RSS MiB B → PGO | Mbps B → PGO | CPU ns/B B → PGO (change) | A/A |
|---|---:|---:|---:|---:|---|
| udp | 58.332 → 58.823 | 9.786 → 9.343 | 230.117 → 249.217 | 20.280 → 18.883 (-6.89%) | passes |
| tcp | 58.635 → 58.741 | 2.551 → 2.626 | 239.266 → 256.497 | 19.607 → 18.323 (-6.55%) | passes |
| stealth | 68.612 → 69.435 | 3.370 → 3.394 | 206.099 → 225.451 | 26.635 → 24.640 (-7.49%) | fails |
| stealth-repeat | 69.766 → 68.545 | 3.139 → 3.207 | 215.136 → 225.274 | 25.945 → 24.343 (-6.17%) | passes |

All **32 timed sessions / 128 individual 100 MB parts verify** (12.8 GB measured),
including a separate eight-session Stealth repeat after noisy first controls.
The first cohort is retained; no pooling or replacement hides its failed screen.
All snapshots end TunnelEstablished with actual coalescing and zero invalid
packets, outer EOF/I/O errors or offload fallbacks. Interpret changes with the
A/A results and WAN variability below; neither more bytes nor a passing screen
proves repeatable maximum capacity or every platform's optimal configuration.

### udp cohort

Started 2026-11-12T16:46:59.426791+00:00; elapsed 145.94s. Direct checks 493.53 / 312.30 Mbps. A/A outer 0/3 versus inner 1/2: speed drift +0.81%, CPU/byte drift -0.62%. Existing 5% stability screen passes; this is not an optimization-gain threshold.

| Role/slot | Build | Verified parts | Seconds | Mbps | CPU % | RSS MiB | CPU ns/B |
|---|---|---:|---:|---:|---:|---:|---:|
| aa/0 | main | 4/4 | 14.630 | 218.734 | 57.022 | 9.539 | 20.856 |
| aa/1 | main | 4/4 | 12.404 | 257.991 | 63.225 | 9.242 | 19.607 |
| aa/2 | main | 4/4 | 14.890 | 214.911 | 55.743 | 9.668 | 20.751 |
| aa/3 | main | 4/4 | 12.830 | 249.424 | 61.582 | 9.625 | 19.754 |
| compare/0 | main | 4/4 | 14.961 | 213.890 | 55.618 | 10.098 | 20.804 |
| compare/1 | erase | 4/4 | 14.041 | 227.907 | 54.389 | 9.586 | 19.092 |
| compare/2 | erase | 4/4 | 11.742 | 272.519 | 63.620 | 9.105 | 18.677 |
| compare/3 | main | 4/4 | 12.925 | 247.574 | 61.178 | 9.484 | 19.769 |

Failed measurement warmup attempts: 0. [Complete evidence](stage7-data/travel-udp.json); [independent restoration](stage7-data/travel-udp-postflight.json).

### tcp cohort

Started 2026-11-12T16:50:12.750350+00:00; elapsed 137.31s. Direct checks 482.27 / 491.74 Mbps. A/A outer 0/3 versus inner 1/2: speed drift +1.85%, CPU/byte drift -0.55%. Existing 5% stability screen passes; this is not an optimization-gain threshold.

| Role/slot | Build | Verified parts | Seconds | Mbps | CPU % | RSS MiB | CPU ns/B |
|---|---|---:|---:|---:|---:|---:|---:|
| aa/0 | main | 4/4 | 13.273 | 241.090 | 58.474 | 2.559 | 19.405 |
| aa/1 | main | 4/4 | 11.276 | 283.777 | 65.301 | 2.598 | 18.411 |
| aa/2 | main | 4/4 | 12.802 | 249.956 | 60.730 | 2.582 | 19.438 |
| aa/3 | main | 4/4 | 11.283 | 283.609 | 66.094 | 2.602 | 18.645 |
| compare/0 | main | 4/4 | 13.384 | 239.092 | 58.253 | 2.566 | 19.493 |
| compare/1 | erase | 4/4 | 12.551 | 254.965 | 58.476 | 2.691 | 18.349 |
| compare/2 | erase | 4/4 | 12.401 | 258.039 | 59.008 | 2.562 | 18.296 |
| compare/3 | main | 4/4 | 13.365 | 239.440 | 59.020 | 2.535 | 19.721 |

Failed measurement warmup attempts: 0. [Complete evidence](stage7-data/travel-tcp.json); [independent restoration](stage7-data/travel-tcp-postflight.json).

### stealth cohort

Started 2026-11-12T16:53:15.225038+00:00; elapsed 185.30s. Direct checks 481.41 / 451.82 Mbps. A/A outer 0/3 versus inner 1/2: speed drift -20.37%, CPU/byte drift +9.35%. Existing 5% stability screen fails; this is not an optimization-gain threshold.

| Role/slot | Build | Verified parts | Seconds | Mbps | CPU % | RSS MiB | CPU ns/B |
|---|---|---:|---:|---:|---:|---:|---:|
| aa/0 | main | 4/4 | 14.094 | 227.043 | 72.669 | 3.223 | 25.606 |
| aa/1 | main | 4/4 | 22.580 | 141.716 | 55.094 | 3.297 | 31.103 |
| aa/2 | main | 4/4 | 14.898 | 214.792 | 71.840 | 3.387 | 26.758 |
| aa/3 | main | 4/4 | 15.135 | 211.432 | 71.832 | 3.480 | 27.180 |
| compare/0 | main | 4/4 | 15.211 | 210.371 | 68.690 | 3.301 | 26.123 |
| compare/1 | erase | 4/4 | 14.334 | 223.243 | 69.385 | 3.367 | 24.866 |
| compare/2 | erase | 4/4 | 14.055 | 227.681 | 69.485 | 3.422 | 24.416 |
| compare/3 | main | 4/4 | 15.848 | 201.913 | 68.535 | 3.441 | 27.156 |

Failed measurement warmup attempts: 0. [Complete evidence](stage7-data/travel-stealth.json); [independent restoration](stage7-data/travel-stealth-postflight.json).

### stealth-repeat cohort

Started 2026-11-12T17:00:05.796047+00:00; elapsed 192.09s. Direct checks 354.38 / 463.44 Mbps. A/A outer 0/3 versus inner 1/2: speed drift +2.67%, CPU/byte drift +0.28%. Existing 5% stability screen passes; this is not an optimization-gain threshold.

| Role/slot | Build | Verified parts | Seconds | Mbps | CPU % | RSS MiB | CPU ns/B |
|---|---|---:|---:|---:|---:|---:|---:|
| aa/0 | main | 4/4 | 15.482 | 206.688 | 67.543 | 3.246 | 26.144 |
| aa/1 | main | 4/4 | 15.654 | 204.416 | 67.782 | 3.266 | 26.528 |
| aa/2 | main | 4/4 | 19.383 | 165.096 | 61.437 | 3.461 | 29.771 |
| aa/3 | main | 4/4 | 20.658 | 154.904 | 58.165 | 3.387 | 30.040 |
| compare/0 | main | 4/4 | 15.034 | 212.846 | 69.540 | 3.133 | 26.138 |
| compare/1 | erase | 4/4 | 13.548 | 236.195 | 72.436 | 3.332 | 24.536 |
| compare/2 | erase | 4/4 | 14.894 | 214.858 | 64.864 | 3.086 | 24.152 |
| compare/3 | main | 4/4 | 14.716 | 217.450 | 69.992 | 3.145 | 25.753 |

Failed measurement warmup attempts: 0. [Complete evidence](stage7-data/travel-stealth-repeat.json); [independent restoration](stage7-data/travel-stealth-repeat-postflight.json).

## Restoration and scope

The passing UDP/TCP cohorts and the separate passing Stealth repeat observe
CPU/byte reductions of 6.89%, 6.55% and 6.17%, with Mbps changes of +8.30%,
+7.20% and +4.71%. RSS changes -4.53%, +2.96% and +2.17%, respectively. These
support fresh, version/feature/target-matched PGO as a candidate for this measured
NoLocalAgent device path. Two balanced samples per build, one saved endpoint and
WAN variability do not establish a universal or long-term capacity improvement.
The first Stealth control failure remains evidence; its observations are not
pooled with the repeat. No CPU-specific device combination is reinstated.

Each protocol finishes within its own 600-second watchdog. Only unique owned
temporary binaries/config copies/interfaces and narrow socket-bound laptop
source/destination/port forwarding rules are used. Separate source-port ranges
avoid cross-cohort TCP reuse. No host NIC or saved router route/configuration,
software, account/key or Direct mode is changed. Cleanup and all twelve checks
pass independently after **each** cohort: unchanged mode, boot/netifd identity,
links, policy/default routes, packages, socket limits and saved configuration
fingerprints; owned files/interface/processes/guard absent and test table empty.
Private configuration and its fingerprints are not published. No Home router
or release is involved.

[Fresh training/profile/build and restoration evidence](stage7-data/travel-training.json). Raw counters and private snapshots are retained locally; the final profile hash binds the selected training files.

The [verbatim temporary driver](stage7-data/travel-driver.txt), not part of normal
Cargo targets, has SHA-256 `83b6e175d168247011a217a56db202498703a438a64b1e6ea1105ea52805cff8`.
It is injected identically into the two owned archives; an external owner must
provide narrow routing, watchdog and restoration. It contains no saved keys or
device-specific credentials. It is an audit artifact, not an installer.
