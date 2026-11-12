# Stage 8: sustained temporary Travel PGO tests

Cudy TR3000 v1, two-core Cortex-A53, Linux 6.12.112, 496,724 KiB RAM, saved
Proton endpoint US-NY#544, MTU 1280. Frozen baseline: merged PR #12 `64caaa2`;
candidate archive: `aa43d3d`. Both contain the same production runtime fingerprint
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

**All three cohorts fail the existing A/A stability screen.** Observed values
below are retained for resource accounting and correctness evidence; none is
an accepted new speedup or PGO-efficiency result. No passing subset, pooling or
replacement cohort is selected. Stage 7's separately measured evidence retains
its original identities and qualification.

| Transport | CPU % B → PGO | RSS MiB B → PGO | Mbps B → PGO | CPU ns/B B → PGO (change) | A/A |
|---|---:|---:|---:|---:|---|
| udp | 54.642 → 55.898 | 9.854 → 10.069 | 220.855 → 232.393 | 19.794 → 19.243 (-2.78%) | fails |
| tcp | 57.923 → 59.759 | 2.562 → 2.681 | 237.396 → 265.737 | 19.520 → 17.993 (-7.82%) | fails |
| stealth | 68.220 → 62.969 | 3.376 → 3.271 | 203.103 → 198.689 | 26.872 → 25.355 (-5.65%) | fails |

All **24 timed sessions / 96 individual 100 MB parts verify** (9.6 GB measured).
All snapshots end TunnelEstablished with actual coalescing and zero invalid
packets, outer EOF/I/O errors or offload fallbacks. Interpret observations with
each cohort's A/A result below. No failed stability screen is removed or pooled
with another cohort. These bounded WAN results do not establish maximum capacity.

### udp cohort

Started 2026-11-12T23:13:35.965263+00:00; elapsed 152.69s. Direct checks 281.72 / 285.57 Mbps. A/A outer 0/3 versus inner 1/2: speed drift +10.20%, CPU/byte drift -1.32%. Existing 5% stability screen fails; this is not an optimization-gain threshold.

| Role/slot | Build | Verified parts | Seconds | Mbps | CPU % | RSS MiB | CPU ns/B |
|---|---|---:|---:|---:|---:|---:|---:|
| aa/0 | main | 4/4 | 13.557 | 236.048 | 58.815 | 9.875 | 19.934 |
| aa/1 | main | 4/4 | 14.432 | 221.722 | 57.481 | 9.906 | 20.741 |
| aa/2 | main | 4/4 | 12.502 | 255.951 | 62.792 | 9.770 | 19.628 |
| aa/3 | main | 4/4 | 16.162 | 197.989 | 51.905 | 10.074 | 20.973 |
| compare/0 | main | 4/4 | 13.085 | 244.549 | 60.548 | 9.855 | 19.808 |
| compare/1 | erase | 4/4 | 14.636 | 218.638 | 53.160 | 9.934 | 19.452 |
| compare/2 | erase | 4/4 | 12.955 | 247.014 | 58.777 | 10.207 | 19.037 |
| compare/3 | main | 4/4 | 16.044 | 199.457 | 49.313 | 9.852 | 19.780 |

Failed measurement warmup attempts: 0. [Complete evidence](stage8-data/travel-udp.json); [independent restoration](stage8-data/travel-udp-postflight.json).

### tcp cohort

Started 2026-11-12T23:16:59.650745+00:00; elapsed 134.75s. Direct checks 483.20 / 493.22 Mbps. A/A outer 0/3 versus inner 1/2: speed drift -7.21%, CPU/byte drift +2.59%. Existing 5% stability screen fails; this is not an optimization-gain threshold.

| Role/slot | Build | Verified parts | Seconds | Mbps | CPU % | RSS MiB | CPU ns/B |
|---|---|---:|---:|---:|---:|---:|---:|
| aa/0 | main | 4/4 | 11.038 | 289.916 | 66.411 | 2.598 | 18.326 |
| aa/1 | main | 4/4 | 14.212 | 225.164 | 55.528 | 2.547 | 19.730 |
| aa/2 | main | 4/4 | 11.164 | 286.643 | 65.912 | 2.578 | 18.397 |
| aa/3 | main | 4/4 | 12.376 | 258.571 | 60.819 | 2.598 | 18.818 |
| compare/0 | main | 4/4 | 12.933 | 247.429 | 60.023 | 2.523 | 19.408 |
| compare/1 | erase | 4/4 | 13.388 | 239.015 | 55.711 | 2.652 | 18.648 |
| compare/2 | erase | 4/4 | 10.831 | 295.448 | 64.102 | 2.711 | 17.361 |
| compare/3 | main | 4/4 | 14.049 | 227.771 | 55.896 | 2.602 | 19.633 |

Failed measurement warmup attempts: 0. [Complete evidence](stage8-data/travel-tcp.json); [independent restoration](stage8-data/travel-tcp-postflight.json).

### stealth cohort

Started 2026-11-12T23:19:59.697818+00:00; elapsed 188.94s. Direct checks 437.16 / 397.05 Mbps. A/A outer 0/3 versus inner 1/2: speed drift +19.70%, CPU/byte drift -5.85%. Existing 5% stability screen fails; this is not an optimization-gain threshold.

| Role/slot | Build | Verified parts | Seconds | Mbps | CPU % | RSS MiB | CPU ns/B |
|---|---|---:|---:|---:|---:|---:|---:|
| aa/0 | main | 4/4 | 21.057 | 151.965 | 55.194 | 3.305 | 29.057 |
| aa/1 | main | 4/4 | 14.983 | 213.581 | 68.788 | 3.281 | 25.767 |
| aa/2 | main | 4/4 | 17.181 | 186.247 | 65.603 | 3.512 | 28.180 |
| aa/3 | main | 4/4 | 17.515 | 182.705 | 64.371 | 3.477 | 28.190 |
| compare/0 | main | 4/4 | 17.006 | 188.173 | 63.700 | 3.293 | 27.083 |
| compare/1 | erase | 4/4 | 15.269 | 209.572 | 66.271 | 3.391 | 25.299 |
| compare/2 | erase | 4/4 | 16.988 | 188.372 | 59.830 | 3.156 | 25.410 |
| compare/3 | main | 4/4 | 14.597 | 219.218 | 73.062 | 3.461 | 26.664 |

Failed measurement warmup attempts: 0. [Complete evidence](stage8-data/travel-stealth.json); [independent restoration](stage8-data/travel-stealth-postflight.json).

## Restoration and scope

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

[Fresh training/profile/build and restoration evidence](stage8-data/travel-training.json). Raw counters and private snapshots are retained locally; the final profile hash binds the selected training files.

[Exact temporary driver source](stage8-data/travel-driver.txt). The same driver
is injected into frozen baseline and current production-source archives. All
subsequent Stage 8 commits change only experiments, diagnostics and evidence;
the tested production runtime fingerprint remains current.
