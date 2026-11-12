# Stage 6: physical Travel router builds

Temporary tests on Cudy TR3000 v1, two-core ARM64 Cortex-A53, Linux 6.12.112,
496,724 KiB RAM, MTU 1280, saved server US-NY#544. CPU-specific and PGO builds
are evaluated independently against frozen merged PR #10. Runtime source is
identical in all three archives: `5d448b4aabf1041e3a13920294314097f88a30eeb9a2d4c799724c2c04112fc7`.
Candidates are archived from `b7d667325b3110d492af009b7270f569735bfe46`;
subsequent artifact-retention/documentation commits do not change that runtime.

Both builds use Rust 1.99.1 / LLVM 23.1.1, ARM64 Linux-musl, release thin LTO,
codegen-units=1, panic=abort, `--no-default-features --features unix`, ring,
the serial receiver and negotiated vnet. The existing identical temporary live
driver runs through the production Connection/TUN worker. The CPU build uses
`-Ctarget-cpu=cortex-a53`, never the GitHub runner's native instruction set.
The PGO build uses a fresh device-trained profile and no native CPU flag.
The [actual CPU and declared compiler features](stage6-data/travel-cpu-features.json)
record Cortex-A53 implementer/part and the available AES/PMULL/SHA/CRC/ASIMD
capabilities; no Neoverse/SVE binary is deployed.

## Verification and measurement

Each comparison has four baseline A/A sessions and then baseline/candidate/
candidate/baseline for each transport. The laptop sends certificate-verified
IPv4 HTTPS through narrowly scoped owned forwarding/NAT rules to a fixed CDN
address. Every timed transfer requires HTTP 206, exact Content-Range, precisely
100,000,000 bytes, SHA-256
`2aa720bbba4fe59c980d62a1763575f43ebf214f28a5af2beadb970865d001b0`
and tunnel-forward counters. Source binary stamps and uploaded binary hashes
are checked. TCP/TLS setup is included in WAN Mbps. This is not the persistent
inner TCP fixture used in GitHub Actions, nor a LAN maximum-capacity test.

CPU is whole-process CPU time / measured interval, 100% = one core. CPU ns/byte
accounts for unequal speeds. RSS is lifetime peak process resident memory,
including setup/warmup, excluding kernel and other processes. Summary values
are geometric means of two balanced samples per build/transport. Direct WAN
checks and A/A drift are retained. No physical ranking is inferred across these
separate allocations/cohorts. IPv6, multiple evaluation flows, long thermal/RTT/
loss behavior and other physical platforms are unmeasured.

## Separate client-only device PGO training

Six verified training cases use UDP/TCP/Stealth × two/four concurrent HTTPS
flows, distinct from the one-flow evaluation. Each flow verifies 12,000,000
bytes and SHA-256 `389a4dfd673664cfc1391c678a627e9f26822a6fc51dcb3108ead489f4497f67`.
Training uses saved IPv4 routing; no IPv6 training claim is made for this device.
The instrumented client exits normally before counters are collected. Only
profiles associated with six successful cases are merged using the compiler's
matching LLVM 23.1.1 tool. The final binary consumes that profile and has no
`__llvm_prf_cnts` section. Training is not a performance observation.

The first SSH file-transfer attempt fails because the router has no SFTP
subsystem, before any training case. Owned files are removed and all twelve
independent checks pass. The supported SSH/base64 retry completes training.
Its first UDP/two-flow attempt has failed warmups; the successful retry and all
other cases pass. Failed-case profiles remain local evidence and are excluded
from the merged profile. No packages or SSH server settings are changed.

Both final device compiler configurations pass linked Drop/recycle inspection
and reject an isolated no-fill/no-barrier negative control. Insecure controls
are never deployed. This linked check is a tripwire, not a formal memory proof.
The actual final binaries then execute the complete transport tests below.

## Results and individual sessions

### CPU comparison

Started 2026-11-12T14:54:51.860762+00:00; elapsed 205.58 seconds. All 24 timed sessions, including twelve balanced comparison downloads across the three transports, verify.

| Transport | CPU % B → C | RSS MiB B → C | Mbps B → C | CPU ns/byte B → C (change) |
|---|---:|---:|---:|---:|
| udp | 63.567 → 57.744 | 9.045 → 9.261 | 257.075 → 223.685 | 19.792 → 20.694 (+4.56%) |
| tcp | 56.208 → 60.811 | 2.578 → 2.373 | 236.382 → 252.525 | 19.028 → 19.273 (+1.28%) |
| stealth | 70.665 → 73.608 | 3.264 → 3.293 | 210.392 → 216.977 | 26.883 → 27.160 (+1.03%) |

A/A compares outer slots 0/3 with inner slots 1/2, matching the balanced order. The existing 5% throughput/CPU-per-byte screen describes measurement stability, not a required optimization gain.

- udp: speed drift -10.76%, CPU/byte drift +11.80%; fails stability screen.
- tcp: speed drift -3.89%, CPU/byte drift +2.37%; passes stability screen.
- stealth: speed drift -1.32%, CPU/byte drift -2.18%; passes stability screen.

Failed warmup attempts: 0. Direct checks: 368.70 / 426.16 Mbps. All measured snapshots end TunnelEstablished with zero invalid packets, outer EOF/I/O errors or offload fallbacks and actual coalescing.

| Transport | Role/slot | Build | Mbps | CPU % | RSS MiB | CPU ns/byte |
|---|---|---|---:|---:|---:|---:|
| udp | aa/0 | main | 171.830 | 43.337 | 8.430 | 20.183 |
| udp | aa/1 | main | 148.826 | 45.785 | 8.984 | 24.618 |
| udp | aa/2 | main | 222.939 | 56.685 | 8.812 | 20.347 |
| udp | aa/3 | main | 242.478 | 60.168 | 9.242 | 19.857 |
| udp | compare/0 | main | 268.426 | 65.564 | 8.562 | 19.548 |
| udp | compare/1 | erase | 201.796 | 54.472 | 9.504 | 21.603 |
| udp | compare/2 | erase | 247.948 | 61.214 | 9.023 | 19.823 |
| udp | compare/3 | main | 246.203 | 61.630 | 9.555 | 20.039 |
| tcp | aa/0 | main | 248.094 | 59.897 | 2.547 | 19.321 |
| tcp | aa/1 | main | 289.973 | 66.239 | 2.492 | 18.285 |
| tcp | aa/2 | main | 232.086 | 58.485 | 2.543 | 20.171 |
| tcp | aa/3 | main | 293.669 | 66.847 | 2.582 | 18.217 |
| tcp | compare/0 | main | 271.485 | 61.785 | 2.598 | 18.213 |
| tcp | compare/1 | erase | 251.703 | 60.862 | 2.348 | 19.353 |
| tcp | compare/2 | erase | 253.351 | 60.760 | 2.398 | 19.193 |
| tcp | compare/3 | main | 205.818 | 51.134 | 2.559 | 19.881 |
| stealth | aa/0 | main | 213.036 | 71.421 | 3.344 | 26.827 |
| stealth | aa/1 | main | 213.007 | 67.814 | 3.176 | 25.485 |
| stealth | aa/2 | main | 223.012 | 71.590 | 3.262 | 25.691 |
| stealth | aa/3 | main | 228.967 | 72.980 | 3.160 | 25.509 |
| stealth | compare/0 | main | 212.846 | 71.333 | 3.246 | 26.819 |
| stealth | compare/1 | erase | 226.251 | 73.134 | 3.238 | 25.873 |
| stealth | compare/2 | erase | 208.083 | 74.086 | 3.348 | 28.511 |
| stealth | compare/3 | main | 207.966 | 70.002 | 3.281 | 26.948 |

[Full sanitized evidence](stage6-data/travel-cpu.json), [independent restoration](stage6-data/travel-cpu-postflight.json). All twelve checks pass after automatic/explicit owned cleanup.

### PGO comparison

Started 2026-11-12T14:59:41.794157+00:00; elapsed 195.95 seconds. All 24 timed sessions, including twelve balanced comparison downloads across the three transports, verify.

| Transport | CPU % B → C | RSS MiB B → C | Mbps B → C | CPU ns/byte B → C (change) |
|---|---:|---:|---:|---:|
| udp | 62.099 → 60.511 | 8.819 → 8.821 | 243.288 → 256.015 | 20.430 → 18.915 (-7.41%) |
| tcp | 66.330 → 54.352 | 2.566 → 2.657 | 289.293 → 231.113 | 18.352 → 18.825 (+2.58%) |
| stealth | 67.968 → 69.489 | 3.060 → 3.308 | 194.987 → 221.406 | 27.896 → 25.119 (-9.95%) |

A/A compares outer slots 0/3 with inner slots 1/2, matching the balanced order. The existing 5% throughput/CPU-per-byte screen describes measurement stability, not a required optimization gain.

- udp: speed drift +5.18%, CPU/byte drift -2.51%; fails stability screen.
- tcp: speed drift -6.34%, CPU/byte drift +2.92%; fails stability screen.
- stealth: speed drift -3.50%, CPU/byte drift +3.31%; passes stability screen.

Failed warmup attempts: 0. Direct checks: 475.44 / 262.81 Mbps. All measured snapshots end TunnelEstablished with zero invalid packets, outer EOF/I/O errors or offload fallbacks and actual coalescing.

| Transport | Role/slot | Build | Mbps | CPU % | RSS MiB | CPU ns/byte |
|---|---|---|---:|---:|---:|---:|
| udp | aa/0 | main | 253.978 | 62.489 | 8.895 | 19.690 |
| udp | aa/1 | main | 263.754 | 65.122 | 8.781 | 19.762 |
| udp | aa/2 | main | 253.275 | 62.210 | 8.707 | 19.656 |
| udp | aa/3 | main | 237.753 | 61.665 | 8.332 | 20.757 |
| udp | compare/0 | main | 229.519 | 61.033 | 9.141 | 21.285 |
| udp | compare/1 | erase | 241.948 | 57.479 | 8.477 | 19.011 |
| udp | compare/2 | erase | 270.899 | 63.703 | 9.180 | 18.820 |
| udp | compare/3 | main | 257.883 | 63.185 | 8.508 | 19.609 |
| tcp | aa/0 | main | 256.177 | 61.852 | 2.438 | 19.321 |
| tcp | aa/1 | main | 227.035 | 58.007 | 2.602 | 20.449 |
| tcp | aa/2 | main | 256.092 | 61.979 | 2.301 | 19.372 |
| tcp | aa/3 | main | 258.724 | 62.553 | 2.449 | 19.354 |
| tcp | compare/0 | main | 286.336 | 66.265 | 2.535 | 18.520 |
| tcp | compare/1 | erase | 207.621 | 49.980 | 2.578 | 19.268 |
| tcp | compare/2 | erase | 257.263 | 59.106 | 2.738 | 18.393 |
| tcp | compare/3 | main | 292.281 | 66.395 | 2.598 | 18.184 |
| stealth | aa/0 | main | 212.709 | 71.964 | 3.363 | 27.071 |
| stealth | aa/1 | main | 138.870 | 55.925 | 3.434 | 32.223 |
| stealth | aa/2 | main | 195.370 | 69.027 | 3.426 | 28.271 |
| stealth | aa/3 | main | 136.977 | 53.964 | 2.895 | 31.528 |
| stealth | compare/0 | main | 193.410 | 67.876 | 3.035 | 28.082 |
| stealth | compare/1 | erase | 209.494 | 67.755 | 3.355 | 25.884 |
| stealth | compare/2 | erase | 233.995 | 71.267 | 3.262 | 24.377 |
| stealth | compare/3 | main | 196.577 | 68.060 | 3.086 | 27.712 |

[Full sanitized evidence](stage6-data/travel-pgo.json), [independent restoration](stage6-data/travel-pgo-postflight.json). All twelve checks pass after automatic/explicit owned cleanup.

## Selection and restoration

The Cortex-A53 build has higher observed CPU/byte for all three transports in
this short cohort (+4.56% UDP, +1.28% TCP, +1.03% Stealth). No Travel CPU-build
improvement is selected from it. Its UDP control fails stability; TCP/Stealth
controls pass without showing an efficiency improvement.

PGO observes CPU/byte -7.41% UDP, +2.58% TCP and -9.95% Stealth. UDP and TCP
controls fail WAN stability. Stealth passes that screen, with CPU 67.968% →
69.489%, RSS 3.060 → 3.308 MiB and Mbps 194.987 → 221.406 (+13.55%). That
Stealth observation is promising, with a memory increase; this short two-sample
per-build cohort and the Direct-check swing do not establish repeatable maximum
capacity. The TCP regression and incomplete transport-wide benefit preclude a
universal Travel PGO recommendation. Keep PGO as a further acceptance candidate;
do not extrapolate native-runner gains or silently replace the installed build.

Only unique owned temporary files/interfaces/rules are created. Socket-bound
laptop loads and a narrow source/destination/port rule leave normal Direct
traffic alone. A 600-second watchdog is armed before test configuration/binaries
are uploaded. Both comparisons finish inside that limit. Explicit cleanup and
separate new-SSH checks verify unchanged Direct mode, boot/netifd identity,
interfaces, rules/default route, installed packages, socket limits and saved
configuration/account/key fingerprints. Owned files/interface/processes/guard
are absent and the temporary route table is empty. Private settings and their
fingerprints are not published. No Home router, installed package, saved key/
account/configuration, persistent NIC change or release is involved.

[Training/profile/build identities](stage6-data/travel-training.json) include compiler version, selected raw/profile hashes, cases and optimized erasure evidence. Raw profiles and private before/after snapshots remain in the local stage evidence.
