# Final temporary Travel validation

Cudy TR3000 v1, two-core Cortex-A53, Linux 6.12.112, 496,724 KiB RAM,
MTU 1280, saved endpoint US-NY#544. Baseline: merged PR #13 `7c582a2`;
current selection: `2a6aeb9`. Both production runtime fingerprints are
`5d448b4aabf1041e3a13920294314097f88a30eeb9a2d4c799724c2c04112fc7`.
Subsequent report-only changes preserve this tested runtime.

Both binaries are ordinary portable ARM64-musl release, Rust 1.99.1 / LLVM
23.1.1, thin LTO, one codegen unit, panic=abort, unix without Local Agent,
ring, serial receiver and negotiated vnet. No PGO or Cortex-A53 flag. This is
an unchanged-runtime validation/characterization, not a new speedup comparison.
Independent baseline/current builds also produce identical executable bytes;
metadata and actual uploaded executable hashes match those local bytes.
Product defaults retain Local Agent; its performance remains unqualified here.

Each transport has its own guarded cohort: four baseline A/A sessions followed
by baseline/current/current/baseline. One continuous tunnel worker carries four
sequential independently SHA256-verified 100 MB ranged HTTPS fetches per timed
session. These are four TCP/TLS connections, not one 400 MB persistent stream.
Each part requires HTTP 206, exact Content-Range/length and SHA256
`2aa720bbba4fe59c980d62a1763575f43ebf214f28a5af2beadb970865d001b0`.
Separate 12 MB warmups verify their own expected hash; forwarding counters and
actual coalescing establish the selected path. CPU is whole-process CPU/wall,
100% = one core; RSS is lifetime peak process MiB including setup/warmup,
excluding kernel/other processes. WAN speed includes setup and laptop validation.
Summary values use two balanced observations per build only when all four verify.
All controls, failed attempts and individual observations remain below.

| Transport | CPU % B → current | RSS MiB B → current | Mbps B → current | A/A |
|---|---:|---:|---:|---|
| udp | 59.638 → 57.407 | 9.799 → 9.991 | 234.140 → 225.630 | fails/incomplete |
| tcp | 56.851 → 57.665 | 2.576 → 2.535 | 231.079 → 235.949 | passes |
| stealth | 67.136 → 70.872 | 3.379 → 3.401 | 198.644 → 215.678 | passes |

Differences between unchanged builds are observation variation, not cumulative
optimization gains. Failed A/A screens are inconclusive; no subset or replacement
cohort is selected. Bounded sessions do not establish thermal, NIC/loss, IPv6 WAN,
other devices/endpoints, concurrent evaluation flows or absolute maximum capacity.

### Travel udp

UTC start 2026-11-12T23:53:56.230868+00:00; elapsed 131.00s. Verified sessions 7/8. A/A drift: null. Direct before/after: 488.99513394767814 / 479.21635666149353 Mbps. Fatal: None.

| Role/slot | Build | Valid | Seconds | Mbps | CPU % | RSS MiB | CPU ns/B |
|---|---|---|---:|---:|---:|---:|---:|
| aa/0 | main | False | n/a | n/a | n/a | n/a | n/a |
| aa/1 | main | True | 12.312 | 259.916 | 64.550 | 9.906 | 19.869 |
| aa/2 | main | True | 14.963 | 213.854 | 55.644 | 10.055 | 20.816 |
| aa/3 | main | True | 13.017 | 245.832 | 61.417 | 9.734 | 19.988 |
| compare/0 | main | True | 14.819 | 215.944 | 56.709 | 9.992 | 21.010 |
| compare/1 | erase | True | 13.365 | 239.423 | 60.130 | 10.113 | 20.092 |
| compare/2 | erase | True | 15.050 | 212.631 | 54.808 | 9.871 | 20.622 |
| compare/3 | main | True | 12.605 | 253.868 | 62.717 | 9.609 | 19.765 |

Failed sessions: [{"build": "main", "role": "aa", "slot": 0, "error": "RuntimeError: warmup failed"}]. Failed warmup attempts: 5. [Complete evidence](final-data/travel-udp.json); [12 independent restoration checks](final-data/travel-udp-postflight.json).

### Travel tcp

UTC start 2026-11-12T23:56:33.231630+00:00; elapsed 145.55s. Verified sessions 8/8. A/A drift: {"Mbps": -3.2538113013989345, "cpubyte": 0.7187931228655797}. Direct before/after: 405.24740683189316 / 488.53777407841255 Mbps. Fatal: None.

| Role/slot | Build | Valid | Seconds | Mbps | CPU % | RSS MiB | CPU ns/B |
|---|---|---|---:|---:|---:|---:|---:|
| aa/0 | main | True | 11.443 | 279.654 | 65.283 | 2.598 | 18.676 |
| aa/1 | main | True | 13.151 | 243.333 | 59.074 | 2.586 | 19.424 |
| aa/2 | main | True | 14.449 | 221.462 | 55.180 | 2.555 | 19.934 |
| aa/3 | main | True | 15.543 | 205.879 | 52.593 | 2.598 | 20.437 |
| compare/0 | main | True | 13.315 | 240.328 | 58.416 | 2.586 | 19.447 |
| compare/1 | erase | True | 14.135 | 226.382 | 55.868 | 2.574 | 19.744 |
| compare/2 | erase | True | 13.012 | 245.919 | 59.521 | 2.496 | 19.364 |
| compare/3 | main | True | 14.402 | 222.185 | 55.327 | 2.566 | 19.922 |

Failed sessions: []. Failed warmup attempts: 0. [Complete evidence](final-data/travel-tcp.json); [12 independent restoration checks](final-data/travel-tcp-postflight.json).

### Travel stealth

UTC start 2026-11-13T00:00:23.266082+00:00; elapsed 180.76s. Verified sessions 8/8. A/A drift: {"Mbps": 4.8837884988057745, "cpubyte": -1.6671983540617274}. Direct before/after: 396.23177637296953 / 495.4609276085829 Mbps. Fatal: None.

| Role/slot | Build | Valid | Seconds | Mbps | CPU % | RSS MiB | CPU ns/B |
|---|---|---|---:|---:|---:|---:|---:|
| aa/0 | main | True | 14.262 | 224.376 | 72.313 | 3.293 | 25.784 |
| aa/1 | main | True | 14.850 | 215.489 | 72.790 | 3.551 | 27.024 |
| aa/2 | main | True | 15.019 | 213.059 | 69.620 | 3.281 | 26.142 |
| aa/3 | main | True | 17.203 | 186.009 | 65.884 | 3.547 | 28.337 |
| compare/0 | main | True | 15.269 | 209.573 | 68.854 | 3.270 | 26.285 |
| compare/1 | erase | True | 15.423 | 207.485 | 69.792 | 3.508 | 26.923 |
| compare/2 | erase | True | 14.273 | 224.194 | 71.969 | 3.297 | 25.682 |
| compare/3 | main | True | 16.995 | 188.285 | 65.462 | 3.492 | 27.815 |

Failed sessions: []. Failed warmup attempts: 0. [Complete evidence](final-data/travel-stealth.json); [12 independent restoration checks](final-data/travel-stealth-postflight.json).

## Restoration

Each cohort completes inside its own 600-second watchdog. Only unique owned
temporary binaries/config copies/interfaces and narrow socket-bound laptop
source/destination/port forwarding rules are used. Independent new-SSH postflight
checks verify Direct mode, boot/netifd identity, links, policy/default routes,
packages, socket limits and saved configuration/account/key fingerprints unchanged;
owned files/interface/processes/guard absent and test table empty. All twelve
checks pass after each cohort. Private settings/fingerprints are not published.
No permanent installation, Home router, package/configuration change or release.
