# Stage 7: same-host builds and sustained-download acceptance

Frozen baseline: merged PR #11 `83582248f3a8e153c6ee75d2846ef7c7064b6323`.
Tested implementation: `2288fe4f412ed9fe7832a11084cc10daf80fa1df`.
[Transition review](../performance-stages.md#stage-7-review-november-12) precedes
implementation. Subsequent evidence/report commits change no production runtime.

**Selection:** keep opt-in combined PGO + host-native CPU compilation as a native
Linux efficiency recipe, and fresh portable PGO as a supported candidate for the
measured Travel NoLocalAgent path. Keep portable production defaults unchanged.
Neither becomes a universal build/profile or a higher-maximum-speed claim.
The tested Cortex-A53 CPU tuning is not reintroduced on Travel. Existing ring,
serial processing, receive coalescing, production source/dependencies and bounds
remain unchanged. No installed device binary, persistent setting or release.

All **1,728 native payloads verify**. Of 216 workload comparisons, **211 are
usable**, three fail A/A stability and two are invalid because a requested rate
was not sustained. The two invalid samples still verify their bytes. Every
unpaced cell is auxiliary-limited, so none ranks maximum client capacity. No
failure or noisy observation is dropped, pooled away or called a speedup.

The combined build generally reduces CPU/byte more than the individual factors
on the same native machines. At 200 Mbps its transport geometric means improve
8.18–8.80% on x64 (one TCP control cell inconclusive) and 4.44–5.87% on ARM64.
Unpaced means improve 9.53–10.27% on x64 and 4.32–5.98% on ARM64, with small/mixed
observed Mbps changes and memory tradeoffs. These are CPU-efficiency findings;
the unpaced helper limits prevent attributing a maximum-speed gain to the client.

[Physical Travel testing](stage7-travel.md) verifies 32 timed sessions / 128
independently checked 100 MB parts. UDP, TCP and a separate passing Stealth repeat
observe CPU/byte -6.89%, -6.55% and -6.17%, with observed Mbps +8.30%, +7.20% and
+4.71%. Peak RSS changes -4.53%, +2.96% and +2.17%. The initial noisy Stealth
cohort is retained separately. These bounded WAN observations support the
device-specific PGO candidate, not universal performance or long-term capacity.
Cleanup and twelve independent checks pass after each physical cohort.

## Experimental contracts and scope

- Each scenario runs all four build factors on one native host: fixed portable
  baseline, PGO, CPU-specific and PGO+CPU. The baseline/helper is built once,
  copied without modification and verified byte-identical across factors.
  Candidate comparisons rotate by workload, with four baseline A/A observations
  and two baseline/two candidate balanced observations in each cell.
- CPU flags apply only to the client. Portable and combined PGO each train fresh
  profiles using their matching target flags, with two/four training flows held
  apart from one/eight-flow evaluation. Helpers remain fixed uninstrumented
  baseline. Matching LLVM tools, source/target/features/flags, profile/raw/binary
  hashes, final absence of instrumentation and actual running helper/client
  executable identities are checked. Profiles are not reusable deployment files
  for arbitrary revisions, features or targets.
- Three independent host allocations per architecture are kept separate:
  `paced` verifies 128 MiB at 200 Mbps; `capacity` verifies unpaced 4 GiB;
  `delay` uses the paced payload with 5 ms netem on outer-loopback transmissions
  in a disposable client namespace. Nominal additional round-trip delay is
  10 ms; actual RTT/jitter/drop behavior is not independently measured. No loss
  is configured, but finite queue behavior is not a loss-free physical model.
  Functional smokes are unshaped and excluded from timed observations.
- qdisc setup must succeed in the owned namespace and appear in each timed log;
  the binary reports a matching scenario. No host interface/route/sysctl or NIC
  configuration changes. Namespaces and owned process groups are disposed after
  each case. Native CPU flags/identity apply to the current host only; a
  Neoverse-native binary is never copied to Cortex-A53 or a virtual LTS guest.
- All native hosts use four available CPUs and kernel 7.0.0-1017-azure. x64 is
  AMD EPYC 7763; ARM64 is Neoverse-N2 / Cobalt 100. Client, peer, load and server
  use separate pinned CPUs. The same-host guarantee is within a scenario;
  absolute values across allocations do not isolate a delay/load/kernel effect.
- Rust 1.99.1 / LLVM 23.1.1, thin LTO, codegen-units=1, panic=abort,
  `--no-default-features --features unix`, serial ring and vnet apply to these
  controlled variants. The diagnostic overlay is identical across each pair.
  Runtime fingerprints match at
  `5d448b4aabf1041e3a13920294314097f88a30eeb9a2d4c799724c2c04112fc7`;
  verified compiler flags and different final binaries establish the build
  comparison, rather than suggesting a runtime-source change.

## Absolute resource summaries

CPU 100% = one core; memory = lifetime peak client-process RSS MiB, including
setup/warmup; Mbps = verified application payload rate. Kernel, peer/load/server
memory and asynchronous kernel CPU are excluded. Values below are geometric
means of four workload cells per transport: IPv4 HTTP / IPv6 HTTPS × one/eight
flows. A summary may include an inconclusive control observation, explicitly
counted in U/4. Missing/invalid cells produce n/a rather than a favorable mean
over a different workload subset. Every individual cell remains below.

U/4 = usable controls/timing cells; C/4 = cells eligible for capacity ranking.
An existing 5% A/A screen measures stability, not a minimum optimization gain.
Transfer duration >=2s, interval skew <=2%, and auxiliary <0.85 core are the
existing measurement-quality screens. There is no imposed client CPU/RAM ceiling.

### paced

| Platform | Build | Transport | CPU % B → C | RSS MiB B → C | Mbps B → C | CPU ns/B change | U/4; C/4 |
|---|---|---|---:|---:|---:|---:|---|
| Linux x64 | pgo | udp | 7.575 → 7.315 | 8.101 → 7.962 | 200.376 → 200.381 | -3.44% | 4; 0 |
| Linux x64 | pgo | tcp | 7.494 → 7.246 | 7.535 → 7.310 | 200.379 → 200.379 | -3.31% | 4; 0 |
| Linux x64 | pgo | stealth | 8.207 → 7.716 | 5.205 → 4.919 | 200.379 → 200.381 | -5.98% | 4; 0 |
| Linux x64 | cpu | udp | 7.710 → 7.459 | 8.122 → 7.894 | 200.379 → 200.377 | -3.24% | 4; 0 |
| Linux x64 | cpu | tcp | 7.519 → 7.203 | 7.455 → 7.473 | 200.379 → 200.379 | -4.21% | 4; 0 |
| Linux x64 | cpu | stealth | 8.172 → 7.973 | 5.246 → 5.281 | 200.373 → 200.374 | -2.42% | 4; 0 |
| Linux x64 | pgo-cpu | udp | 7.613 → 6.973 | 8.062 → 8.187 | 200.376 → 200.380 | -8.42% | 4; 0 |
| Linux x64 | pgo-cpu | tcp | 7.498 → 6.884 | 7.207 → 7.262 | 200.379 → 200.379 | -8.18% | 3; 0 |
| Linux x64 | pgo-cpu | stealth | 8.185 → 7.464 | 5.155 → 4.942 | 200.373 → 200.374 | -8.80% | 4; 0 |
| Linux ARM64 | pgo | udp | 6.021 → 5.787 | 7.364 → 7.410 | 200.398 → 200.399 | -3.89% | 4; 0 |
| Linux ARM64 | pgo | tcp | 5.658 → 5.542 | 6.750 → 6.673 | 200.399 → 200.400 | -2.04% | 4; 0 |
| Linux ARM64 | pgo | stealth | 6.831 → 6.539 | 4.628 → 4.510 | 200.394 → 200.394 | -4.27% | 4; 0 |
| Linux ARM64 | cpu | udp | 6.071 → 5.970 | 7.371 → 7.539 | 200.398 → 200.398 | -1.66% | 4; 0 |
| Linux ARM64 | cpu | tcp | 5.687 → 5.563 | 6.792 → 6.823 | 200.399 → 200.399 | -2.19% | 4; 0 |
| Linux ARM64 | cpu | stealth | 6.860 → 6.769 | 4.544 → 4.525 | 200.394 → 200.394 | -1.33% | 4; 0 |
| Linux ARM64 | pgo-cpu | udp | 6.088 → 5.738 | 7.318 → 7.440 | 200.397 → 200.399 | -5.75% | 4; 0 |
| Linux ARM64 | pgo-cpu | tcp | 5.735 → 5.480 | 6.839 → 6.794 | 200.399 → 200.400 | -4.44% | 4; 0 |
| Linux ARM64 | pgo-cpu | stealth | 6.826 → 6.425 | 4.585 → 4.478 | 200.393 → 200.394 | -5.87% | 4; 0 |

### capacity

| Platform | Build | Transport | CPU % B → C | RSS MiB B → C | Mbps B → C | CPU ns/B change | U/4; C/4 |
|---|---|---|---:|---:|---:|---:|---|
| Linux x64 | pgo | udp | 71.221 → 67.867 | 8.138 → 8.020 | 2363.667 → 2364.896 | -4.76% | 4; 0 |
| Linux x64 | pgo | tcp | 73.250 → 69.557 | 7.684 → 7.698 | 2441.286 → 2437.812 | -4.91% | 4; 0 |
| Linux x64 | pgo | stealth | 75.577 → 70.937 | 5.323 → 5.025 | 2248.001 → 2237.072 | -5.68% | 4; 0 |
| Linux x64 | cpu | udp | 70.581 → 67.661 | 8.377 → 8.329 | 2340.859 → 2352.971 | -4.63% | 4; 0 |
| Linux x64 | cpu | tcp | 73.417 → 70.173 | 7.630 → 7.742 | 2442.091 → 2448.778 | -4.68% | 4; 0 |
| Linux x64 | cpu | stealth | 75.853 → 72.967 | 5.379 → 5.300 | 2256.097 → 2243.137 | -3.25% | 4; 0 |
| Linux x64 | pgo-cpu | udp | 70.838 → 64.446 | 8.185 → 8.227 | 2347.256 → 2366.300 | -9.76% | 4; 0 |
| Linux x64 | pgo-cpu | tcp | 73.755 → 66.029 | 7.613 → 7.505 | 2446.151 → 2440.645 | -10.27% | 4; 0 |
| Linux x64 | pgo-cpu | stealth | 75.617 → 68.163 | 5.327 → 4.992 | 2245.719 → 2237.707 | -9.53% | 4; 0 |
| Linux ARM64 | pgo | udp | 87.024 → 84.158 | 7.461 → 7.306 | 2938.154 → 2933.540 | -3.14% | 4; 0 |
| Linux ARM64 | pgo | tcp | 85.681 → 82.611 | 7.180 → 7.105 | 3068.765 → 3075.119 | -3.78% | 4; 0 |
| Linux ARM64 | pgo | stealth | 87.419 → 81.860 | 4.702 → 4.577 | 2551.328 → 2531.626 | -5.63% | 4; 0 |
| Linux ARM64 | cpu | udp | 86.366 → 85.110 | 7.558 → 7.472 | 2948.916 → 2941.622 | -1.21% | 4; 0 |
| Linux ARM64 | cpu | tcp | 85.619 → 84.071 | 7.077 → 7.120 | 3061.242 → 3057.910 | -1.70% | 4; 0 |
| Linux ARM64 | cpu | stealth | 85.828 → 84.552 | 4.779 → 4.543 | 2533.785 → 2531.796 | -1.41% | 4; 0 |
| Linux ARM64 | pgo-cpu | udp | 87.603 → 82.551 | 7.767 → 7.121 | 2949.692 → 2905.034 | -4.32% | 4; 0 |
| Linux ARM64 | pgo-cpu | tcp | 85.541 → 81.504 | 6.842 → 6.927 | 3073.383 → 3061.703 | -4.36% | 4; 0 |
| Linux ARM64 | pgo-cpu | stealth | 86.838 → 81.493 | 4.695 → 4.575 | 2547.877 → 2543.038 | -5.98% | 4; 0 |

### delay

| Platform | Build | Transport | CPU % B → C | RSS MiB B → C | Mbps B → C | CPU ns/B change | U/4; C/4 |
|---|---|---|---:|---:|---:|---:|---|
| Linux x64 | pgo | udp | 7.154 → 6.904 | 8.439 → 8.332 | 200.016 → 200.016 | -3.49% | 4; 0 |
| Linux x64 | pgo | tcp | n/a | n/a | n/a | n/a | 3; 0 |
| Linux x64 | pgo | stealth | 7.540 → 7.168 | 5.561 → 5.191 | 200.013 → 200.012 | -4.93% | 4; 0 |
| Linux x64 | cpu | udp | 7.137 → 6.900 | 8.429 → 8.456 | 200.017 → 200.017 | -3.32% | 4; 0 |
| Linux x64 | cpu | tcp | 6.920 → 6.903 | 7.752 → 8.132 | 200.013 → 200.015 | -0.25% | 3; 0 |
| Linux x64 | cpu | stealth | 7.564 → 7.335 | 5.423 → 5.381 | 200.011 → 200.014 | -3.03% | 4; 0 |
| Linux x64 | pgo-cpu | udp | 7.132 → 6.600 | 8.702 → 8.256 | 200.017 → 200.016 | -7.46% | 4; 0 |
| Linux x64 | pgo-cpu | tcp | 7.028 → 6.387 | 8.102 → 7.831 | 200.014 → 200.019 | -9.12% | 3; 0 |
| Linux x64 | pgo-cpu | stealth | 7.559 → 6.851 | 5.547 → 5.271 | 200.015 → 200.016 | -9.36% | 4; 0 |
| Linux ARM64 | pgo | udp | 5.704 → 5.513 | 7.539 → 7.478 | 200.024 → 200.026 | -3.36% | 4; 0 |
| Linux ARM64 | pgo | tcp | 5.382 → 5.200 | 7.590 → 7.167 | 200.017 → 200.027 | -3.38% | 4; 0 |
| Linux ARM64 | pgo | stealth | 6.555 → 6.283 | 4.939 → 4.667 | 200.020 → 200.022 | -4.14% | 4; 0 |
| Linux ARM64 | cpu | udp | 5.746 → 5.557 | 7.616 → 7.678 | 200.025 → 200.026 | -3.28% | 4; 0 |
| Linux ARM64 | cpu | tcp | 5.392 → 5.333 | 7.311 → 7.326 | 200.026 → 200.027 | -1.10% | 4; 0 |
| Linux ARM64 | cpu | stealth | 6.577 → 6.439 | 4.907 → 4.692 | 200.022 → 200.022 | -2.11% | 4; 0 |
| Linux ARM64 | pgo-cpu | udp | 5.646 → 5.357 | 7.654 → 7.651 | 200.026 → 200.026 | -5.12% | 4; 0 |
| Linux ARM64 | pgo-cpu | tcp | n/a | n/a | n/a | n/a | 3; 0 |
| Linux ARM64 | pgo-cpu | stealth | 6.487 → 6.158 | 4.888 → 4.771 | 200.022 → 200.022 | -5.07% | 4; 0 |

## Measurement failures and interpretation

- Paced x64 `pgo-cpu / tcp-v4-http-1` fails A/A stability. Its observations are
  retained, not treated as a fully stable four-cell TCP summary.
- Delay x64 `cpu / tcp-v4-http-8` and `pgo-cpu / tcp-v6-https-1` also fail A/A.
- Delay x64 `pgo / tcp-v6-https-1`, sample 54 (candidate), verifies all 128 MiB
  but sustains 91.44 rather than 200 Mbps. Delay ARM64 `pgo-cpu /
  tcp-v6-https-1`, sample 56 (baseline), verifies all bytes but sustains 167.38
  Mbps. Those two comparison cells are invalid/incomplete; their full-transport
  aggregates are n/a. Four analyzer errors are two rate misses plus their two
  consequent missing/invalid-sample notices, not four failed payloads.
- [The delay workflow](https://github.com/riKirsti/ProtunNext/actions/runs/43260006760)
  therefore ends failed at its measurement-quality assertion on both hosts.
  This failure is retained. It must not be relabeled as a passing performance
  run or attributed conclusively to compiler tuning. Delayed TCP behavior is not
  fully accepted by this study, even though bytes and tunnel identity verify.
- [Paced](https://github.com/riKirsti/ProtunNext/actions/runs/43259997502) and
  [unpaced](https://github.com/riKirsti/ProtunNext/actions/runs/43260002426)
  workflows complete successfully. All 72 unpaced cells pass A/A and timing,
  but all 72 have a saturated auxiliary role. Their observed Mbps ratios cannot
  establish maximum client speed. The same helpers make comparisons controlled,
  not automatically free of bottlenecks.
- No observation from different hosts/scenarios is pooled to hide these limits.
  Two balanced samples per build/cell are not a statistical significance or
  thermal-stability proof. The device's first Stealth cohort similarly fails
  its WAN controls; its successful repeat uses the same binaries/profile and is
  retained separately, with no source/profile selection to obtain a better run.

## Correctness, platforms and reproducibility

[All nine platform jobs](https://github.com/riKirsti/ProtunNext/actions/runs/43259943500),
[release erasure](https://github.com/riKirsti/ProtunNext/actions/runs/43259943269)
and [native fixture/analyzer checks](https://github.com/riKirsti/ProtunNext/actions/runs/43259943432)
pass for tested implementation `2288fe4`. Local suites pass 22 download and 12
audit tests, including rejection of partial/undeclared combined flags, wrong
CPU/profile/helper identity, absent actual impairment and payload/rate mismatch.
Each tuned native build also passes release library regressions and linked
Drop/recycle inspection under its actual flags, then rejects a genuinely
missing-wipe control. The device PGO configuration passes the same linked
positive/negative check. No insecure control is executed as a tunnel or deployed.
The executable fixture alone is not memory readback; linked inspection is a
regression tripwire, not formal proof of every compiler path.

All six final native artifacts are reanalyzed from raw samples locally; their
stored analyses reproduce within floating-point rounding. Baseline/candidate
binary hashes and selected raw/merged profile hashes are independently checked
against artifact bytes. The [data index](stage7-data/index.json) retains six
cohorts, common per-factor environments, all 216 cells and their errors. GitHub
artifacts retain every raw sample, plan, log, binary, profile and disassembly.

Native Linux x64/ARM64 and the physical Linux ARM64 Travel path are performance
measured. Windows, Apple and Android are build/API-contract checked but download
performance unmeasured. Supported Linux LTS is functionally checked; Stage 7 LTS
download performance is unmeasured. Native results and Stage 6's earlier LTS
results are not substituted for missing platform data. Benchmarks exercise the
NoLocalAgent path; account-enabled CLI/SDK PGO performance remains unqualified.
This stage does not silently disable account features in normal builds.

To reproduce, dispatch the existing native download workflow at the tested
source with the frozen baseline, `run_downloads=true`, `build_mode=accept-paced`,
`accept-capacity` or `accept-delay`, `tun_mode=vnet`, `client_cores=1`, prototypes
disabled and no block filter. The controller fixes scenario payload/rate and
rotates factors; changing ordinary rate inputs does not change that contract.
It creates fresh profiles and never promotes an artifact to a global default.
See [build instructions](../../experiments/build/README.md#stage-7-same-host-acceptance)
and [physical method / complete evidence](stage7-travel.md).

## Final boundary and remaining evidence

The planned seven-stage candidate evaluation is recorded; absolute best
performance across architectures is not proven. Production keeps the accepted
receive-path changes from earlier stages, ring, serial processing and portable
defaults. Keep combined tuning as an opt-in native efficiency experiment and
device PGO as a measured NoLocalAgent candidate, with fresh matching training and
explicit portability/resource limits. No global compiler/provider policy or
unqualified portable profile is shipped.

Before a maximum-throughput or universal production build recommendation, remove
the peer/load bottleneck, resolve/characterize the delayed TCP rate misses, and
measure target NIC/RTT/loss/thermal behavior plus account-enabled CLI/SDK and the
unmeasured OS targets. Deferred native WireGuard/I/O/TLS/NIC/bypass designs remain
open; failed earlier prototypes did not exhaust them. Further implementation
requires reviewing a focused follow-up plan, not assuming this study proved the
end of worthwhile optimization. No release is created by this PR.

## Every native workload

B = the fixed portable baseline; C = the declared candidate. All observations,
including inconclusive and invalid cells, are retained. Capacity ranking is
false throughout for the reasons above.

### paced / x64 / pgo

| Workload | CPU % B → C | RSS MiB B → C | Mbps B → C | CPU ns/B B → C (change) | Status / capacity rank |
|---|---:|---:|---:|---:|---|
| stealth-v4-http-1 | 7.756 → 7.273 | 4.955 → 4.690 | 200.070 → 200.075 | 3.101 → 2.908 (-6.23%) | usable; False |
| stealth-v4-http-8 | 8.027 → 7.592 | 5.878 → 5.371 | 200.690 → 200.699 | 3.200 → 3.026 (-5.42%) | usable; False |
| stealth-v6-https-1 | 8.609 → 7.981 | 4.929 → 4.781 | 200.065 → 200.068 | 3.443 → 3.191 (-7.29%) | usable; False |
| stealth-v6-https-8 | 8.466 → 8.045 | 5.113 → 4.861 | 200.690 → 200.685 | 3.375 → 3.207 (-4.96%) | usable; False |
| tcp-v4-http-1 | 7.047 → 6.914 | 7.158 → 7.064 | 200.070 → 200.073 | 2.818 → 2.765 (-1.88%) | usable; False |
| tcp-v4-http-8 | 7.274 → 7.045 | 8.292 → 7.730 | 200.695 → 200.692 | 2.900 → 2.808 (-3.14%) | usable; False |
| tcp-v6-https-1 | 8.110 → 7.672 | 7.141 → 7.097 | 200.069 → 200.066 | 3.243 → 3.068 (-5.40%) | usable; False |
| tcp-v6-https-8 | 7.589 → 7.378 | 7.606 → 7.366 | 200.684 → 200.686 | 3.025 → 2.941 (-2.79%) | usable; False |
| udp-v4-http-1 | 7.582 → 7.278 | 7.781 → 7.694 | 200.071 → 200.072 | 3.032 → 2.910 (-4.01%) | usable; False |
| udp-v4-http-8 | 7.242 → 7.103 | 8.239 → 8.201 | 200.696 → 200.694 | 2.887 → 2.832 (-1.92%) | usable; False |
| udp-v6-https-1 | 7.771 → 7.359 | 7.938 → 7.636 | 200.066 → 200.068 | 3.107 → 2.943 (-5.31%) | usable; False |
| udp-v6-https-8 | 7.717 → 7.524 | 8.461 → 8.343 | 200.673 → 200.691 | 3.076 → 2.999 (-2.50%) | usable; False |

### paced / x64 / cpu

| Workload | CPU % B → C | RSS MiB B → C | Mbps B → C | CPU ns/B B → C (change) | Status / capacity rank |
|---|---:|---:|---:|---:|---|
| stealth-v4-http-1 | 7.971 → 7.732 | 5.095 → 5.000 | 200.065 → 200.072 | 3.187 → 3.092 (-3.00%) | usable; False |
| stealth-v4-http-8 | 7.807 → 7.700 | 5.863 → 5.927 | 200.690 → 200.688 | 3.112 → 3.069 (-1.37%) | usable; False |
| stealth-v6-https-1 | 8.516 → 8.385 | 4.867 → 5.000 | 200.067 → 200.060 | 3.405 → 3.353 (-1.54%) | usable; False |
| stealth-v6-https-8 | 8.414 → 8.096 | 5.209 → 5.249 | 200.672 → 200.677 | 3.354 → 3.228 (-3.77%) | usable; False |
| tcp-v4-http-1 | 7.233 → 6.925 | 7.273 → 7.176 | 200.071 → 200.070 | 2.892 → 2.769 (-4.25%) | usable; False |
| tcp-v4-http-8 | 7.267 → 6.877 | 7.504 → 7.802 | 200.700 → 200.697 | 2.897 → 2.741 (-5.36%) | usable; False |
| tcp-v6-https-1 | 7.966 → 7.456 | 7.228 → 7.218 | 200.065 → 200.068 | 3.186 → 2.981 (-6.41%) | usable; False |
| tcp-v6-https-8 | 7.635 → 7.579 | 7.831 → 7.717 | 200.681 → 200.684 | 3.044 → 3.021 (-0.74%) | usable; False |
| udp-v4-http-1 | 7.521 → 7.250 | 7.866 → 7.622 | 200.072 → 200.071 | 3.007 → 2.899 (-3.60%) | usable; False |
| udp-v4-http-8 | 7.735 → 7.402 | 8.668 → 8.115 | 200.692 → 200.682 | 3.083 → 2.951 (-4.30%) | usable; False |
| udp-v6-https-1 | 7.697 → 7.648 | 7.909 → 7.535 | 200.070 → 200.068 | 3.078 → 3.058 (-0.64%) | usable; False |
| udp-v6-https-8 | 7.891 → 7.545 | 8.071 → 8.334 | 200.682 → 200.686 | 3.146 → 3.008 (-4.38%) | usable; False |

### paced / x64 / pgo-cpu

| Workload | CPU % B → C | RSS MiB B → C | Mbps B → C | CPU ns/B B → C (change) | Status / capacity rank |
|---|---:|---:|---:|---:|---|
| stealth-v4-http-1 | 8.021 → 7.363 | 4.992 → 4.734 | 200.069 → 200.072 | 3.207 → 2.944 (-8.21%) | usable; False |
| stealth-v4-http-8 | 7.836 → 7.332 | 5.553 → 5.213 | 200.689 → 200.669 | 3.124 → 2.923 (-6.43%) | usable; False |
| stealth-v6-https-1 | 8.753 → 7.597 | 4.898 → 4.843 | 200.065 → 200.069 | 3.500 → 3.038 (-13.21%) | usable; False |
| stealth-v6-https-8 | 8.156 → 7.570 | 5.203 → 4.992 | 200.672 → 200.686 | 3.252 → 3.018 (-7.19%) | usable; False |
| tcp-v4-http-1 | 7.021 → 6.544 | 6.840 → 6.880 | 200.072 → 200.070 | 2.807 → 2.617 (-6.79%) | inconclusive-control/timing; False |
| tcp-v4-http-8 | 7.312 → 6.750 | 7.769 → 7.510 | 200.696 → 200.700 | 2.915 → 2.690 (-7.70%) | usable; False |
| tcp-v6-https-1 | 8.038 → 6.993 | 6.926 → 7.114 | 200.062 → 200.069 | 3.214 → 2.796 (-13.00%) | usable; False |
| tcp-v6-https-8 | 7.658 → 7.270 | 7.331 → 7.566 | 200.687 → 200.678 | 3.053 → 2.898 (-5.06%) | usable; False |
| udp-v4-http-1 | 7.603 → 6.696 | 7.685 → 8.139 | 200.072 → 200.070 | 3.040 → 2.678 (-11.93%) | usable; False |
| udp-v4-http-8 | 7.386 → 6.964 | 8.195 → 8.260 | 200.688 → 200.695 | 2.944 → 2.776 (-5.71%) | usable; False |
| udp-v6-https-1 | 7.822 → 7.252 | 7.646 → 7.826 | 200.060 → 200.067 | 3.128 → 2.900 (-7.29%) | usable; False |
| udp-v6-https-8 | 7.647 → 6.989 | 8.771 → 8.539 | 200.685 → 200.689 | 3.049 → 2.786 (-8.62%) | usable; False |

### paced / arm / pgo

| Workload | CPU % B → C | RSS MiB B → C | Mbps B → C | CPU ns/B B → C (change) | Status / capacity rank |
|---|---:|---:|---:|---:|---|
| stealth-v4-http-1 | 6.830 → 6.370 | 4.410 → 4.285 | 200.078 → 200.078 | 2.731 → 2.547 (-6.73%) | usable; False |
| stealth-v4-http-8 | 6.517 → 6.360 | 4.968 → 4.984 | 200.714 → 200.715 | 2.597 → 2.535 (-2.41%) | usable; False |
| stealth-v6-https-1 | 7.292 → 6.890 | 4.320 → 4.214 | 200.076 → 200.076 | 2.916 → 2.755 (-5.51%) | usable; False |
| stealth-v6-https-8 | 6.710 → 6.551 | 4.847 → 4.597 | 200.710 → 200.709 | 2.674 → 2.611 (-2.36%) | usable; False |
| tcp-v4-http-1 | 5.518 → 5.441 | 6.520 → 6.647 | 200.078 → 200.079 | 2.206 → 2.175 (-1.40%) | usable; False |
| tcp-v4-http-8 | 5.382 → 5.294 | 6.972 → 7.376 | 200.725 → 200.725 | 2.145 → 2.110 (-1.64%) | usable; False |
| tcp-v6-https-1 | 6.167 → 5.989 | 6.387 → 5.968 | 200.077 → 200.077 | 2.466 → 2.395 (-2.89%) | usable; False |
| tcp-v6-https-8 | 5.594 → 5.470 | 7.149 → 6.774 | 200.719 → 200.719 | 2.230 → 2.180 (-2.22%) | usable; False |
| udp-v4-http-1 | 6.045 → 5.769 | 7.654 → 7.201 | 200.078 → 200.078 | 2.417 → 2.307 (-4.56%) | usable; False |
| udp-v4-http-8 | 5.625 → 5.563 | 7.234 → 7.714 | 200.722 → 200.722 | 2.242 → 2.217 (-1.11%) | usable; False |
| udp-v6-https-1 | 6.611 → 6.147 | 7.016 → 7.127 | 200.077 → 200.079 | 2.643 → 2.458 (-7.02%) | usable; False |
| udp-v6-https-8 | 5.848 → 5.686 | 7.571 → 7.615 | 200.716 → 200.717 | 2.331 → 2.266 (-2.78%) | usable; False |

### paced / arm / cpu

| Workload | CPU % B → C | RSS MiB B → C | Mbps B → C | CPU ns/B B → C (change) | Status / capacity rank |
|---|---:|---:|---:|---:|---|
| stealth-v4-http-1 | 6.807 → 6.629 | 4.402 → 4.343 | 200.077 → 200.077 | 2.722 → 2.651 (-2.61%) | usable; False |
| stealth-v4-http-8 | 6.479 → 6.402 | 4.909 → 4.779 | 200.716 → 200.713 | 2.582 → 2.552 (-1.20%) | usable; False |
| stealth-v6-https-1 | 7.413 → 7.411 | 4.410 → 4.341 | 200.075 → 200.076 | 2.964 → 2.963 (-0.02%) | usable; False |
| stealth-v6-https-8 | 6.775 → 6.674 | 4.472 → 4.652 | 200.709 → 200.711 | 2.700 → 2.660 (-1.49%) | usable; False |
| tcp-v4-http-1 | 5.551 → 5.435 | 6.527 → 6.159 | 200.078 → 200.079 | 2.220 → 2.173 (-2.10%) | usable; False |
| tcp-v4-http-8 | 5.366 → 5.378 | 7.040 → 6.998 | 200.725 → 200.723 | 2.139 → 2.144 (+0.23%) | usable; False |
| tcp-v6-https-1 | 6.285 → 5.865 | 6.439 → 6.900 | 200.077 → 200.077 | 2.513 → 2.345 (-6.69%) | usable; False |
| tcp-v6-https-8 | 5.588 → 5.586 | 7.191 → 7.285 | 200.717 → 200.719 | 2.227 → 2.226 (-0.05%) | usable; False |
| udp-v4-http-1 | 6.274 → 6.190 | 7.347 → 7.465 | 200.078 → 200.078 | 2.509 → 2.475 (-1.36%) | usable; False |
| udp-v4-http-8 | 5.663 → 5.566 | 7.417 → 8.033 | 200.721 → 200.721 | 2.257 → 2.218 (-1.72%) | usable; False |
| udp-v6-https-1 | 6.543 → 6.448 | 6.903 → 7.081 | 200.077 → 200.078 | 2.616 → 2.578 (-1.45%) | usable; False |
| udp-v6-https-8 | 5.842 → 5.719 | 7.847 → 7.610 | 200.716 → 200.716 | 2.329 → 2.280 (-2.10%) | usable; False |

### paced / arm / pgo-cpu

| Workload | CPU % B → C | RSS MiB B → C | Mbps B → C | CPU ns/B B → C (change) | Status / capacity rank |
|---|---:|---:|---:|---:|---|
| stealth-v4-http-1 | 6.737 → 6.113 | 4.389 → 4.285 | 200.077 → 200.078 | 2.694 → 2.444 (-9.27%) | usable; False |
| stealth-v4-http-8 | 6.545 → 6.264 | 4.844 → 4.828 | 200.714 → 200.715 | 2.609 → 2.497 (-4.28%) | usable; False |
| stealth-v6-https-1 | 7.281 → 6.827 | 4.472 → 4.347 | 200.076 → 200.077 | 2.911 → 2.730 (-6.23%) | usable; False |
| stealth-v6-https-8 | 6.762 → 6.518 | 4.648 → 4.472 | 200.708 → 200.708 | 2.695 → 2.598 (-3.60%) | usable; False |
| tcp-v4-http-1 | 5.555 → 5.397 | 6.595 → 6.535 | 200.079 → 200.078 | 2.221 → 2.158 (-2.85%) | usable; False |
| tcp-v4-http-8 | 5.392 → 5.186 | 7.159 → 7.028 | 200.723 → 200.725 | 2.149 → 2.067 (-3.82%) | usable; False |
| tcp-v6-https-1 | 6.374 → 5.940 | 6.533 → 6.534 | 200.076 → 200.078 | 2.549 → 2.375 (-6.82%) | usable; False |
| tcp-v6-https-8 | 5.664 → 5.424 | 7.091 → 7.097 | 200.718 → 200.719 | 2.257 → 2.162 (-4.23%) | usable; False |
| udp-v4-http-1 | 6.216 → 5.822 | 7.278 → 7.268 | 200.078 → 200.078 | 2.485 → 2.328 (-6.33%) | usable; False |
| udp-v4-http-8 | 5.619 → 5.447 | 7.301 → 7.637 | 200.721 → 200.723 | 2.240 → 2.171 (-3.07%) | usable; False |
| udp-v6-https-1 | 6.740 → 6.144 | 6.934 → 7.092 | 200.077 → 200.078 | 2.695 → 2.457 (-8.85%) | usable; False |
| udp-v6-https-8 | 5.833 → 5.563 | 7.784 → 7.784 | 200.714 → 200.716 | 2.325 → 2.217 (-4.64%) | usable; False |

### capacity / x64 / pgo

| Workload | CPU % B → C | RSS MiB B → C | Mbps B → C | CPU ns/B B → C (change) | Status / capacity rank |
|---|---:|---:|---:|---:|---|
| stealth-v4-http-1 | 75.227 → 70.340 | 5.171 → 4.880 | 2477.041 → 2477.807 | 2.430 → 2.271 (-6.52%) | usable; False |
| stealth-v4-http-8 | 76.498 → 72.051 | 5.533 → 5.285 | 2229.660 → 2212.342 | 2.745 → 2.605 (-5.08%) | usable; False |
| stealth-v6-https-1 | 74.672 → 69.722 | 5.084 → 4.766 | 2256.303 → 2243.213 | 2.648 → 2.487 (-6.08%) | usable; False |
| stealth-v6-https-8 | 75.924 → 71.659 | 5.520 → 5.189 | 2049.353 → 2036.713 | 2.964 → 2.815 (-5.03%) | usable; False |
| tcp-v4-http-1 | 72.609 → 68.698 | 7.615 → 7.734 | 2689.593 → 2694.094 | 2.160 → 2.040 (-5.54%) | usable; False |
| tcp-v4-http-8 | 73.983 → 71.106 | 7.965 → 7.765 | 2441.400 → 2435.440 | 2.424 → 2.336 (-3.65%) | usable; False |
| tcp-v6-https-1 | 72.470 → 68.456 | 7.335 → 7.779 | 2445.844 → 2448.656 | 2.370 → 2.237 (-5.65%) | usable; False |
| tcp-v6-https-8 | 73.952 → 69.999 | 7.835 → 7.517 | 2211.671 → 2198.279 | 2.675 → 2.547 (-4.77%) | usable; False |
| udp-v4-http-1 | 70.420 → 66.946 | 7.696 → 7.717 | 2602.565 → 2597.688 | 2.165 → 2.062 (-4.75%) | usable; False |
| udp-v4-http-8 | 73.176 → 70.305 | 8.410 → 7.897 | 2368.226 → 2378.098 | 2.472 → 2.365 (-4.32%) | usable; False |
| udp-v6-https-1 | 69.745 → 66.272 | 8.159 → 8.461 | 2365.356 → 2376.787 | 2.359 → 2.231 (-5.44%) | usable; False |
| udp-v6-https-8 | 71.591 → 68.013 | 8.308 → 8.023 | 2141.034 → 2130.301 | 2.675 → 2.554 (-4.52%) | usable; False |

### capacity / x64 / cpu

| Workload | CPU % B → C | RSS MiB B → C | Mbps B → C | CPU ns/B B → C (change) | Status / capacity rank |
|---|---:|---:|---:|---:|---|
| stealth-v4-http-1 | 75.116 → 72.381 | 5.127 → 5.123 | 2488.148 → 2475.594 | 2.415 → 2.339 (-3.15%) | usable; False |
| stealth-v4-http-8 | 77.384 → 75.026 | 5.822 → 6.122 | 2241.198 → 2235.739 | 2.762 → 2.685 (-2.81%) | usable; False |
| stealth-v6-https-1 | 74.277 → 70.972 | 5.180 → 4.966 | 2256.705 → 2255.845 | 2.633 → 2.517 (-4.41%) | usable; False |
| stealth-v6-https-8 | 76.674 → 73.552 | 5.417 → 5.066 | 2058.731 → 2027.745 | 2.979 → 2.902 (-2.61%) | usable; False |
| tcp-v4-http-1 | 72.700 → 69.636 | 7.733 → 7.426 | 2689.047 → 2704.663 | 2.163 → 2.060 (-4.77%) | usable; False |
| tcp-v4-http-8 | 74.434 → 71.315 | 7.737 → 7.951 | 2434.410 → 2443.830 | 2.446 → 2.335 (-4.56%) | usable; False |
| tcp-v6-https-1 | 72.430 → 68.841 | 7.208 → 7.634 | 2447.739 → 2456.095 | 2.367 → 2.242 (-5.28%) | usable; False |
| tcp-v6-https-8 | 74.124 → 70.929 | 7.860 → 7.969 | 2219.680 → 2214.972 | 2.672 → 2.562 (-4.11%) | usable; False |
| udp-v4-http-1 | 70.364 → 67.251 | 8.273 → 8.124 | 2599.903 → 2620.467 | 2.165 → 2.053 (-5.18%) | usable; False |
| udp-v4-http-8 | 72.620 → 69.141 | 8.298 → 8.683 | 2356.410 → 2302.954 | 2.465 → 2.402 (-2.58%) | usable; False |
| udp-v6-https-1 | 68.312 → 65.846 | 8.428 → 7.826 | 2297.272 → 2373.530 | 2.379 → 2.219 (-6.71%) | usable; False |
| udp-v6-https-8 | 71.096 → 68.452 | 8.509 → 8.719 | 2133.443 → 2139.969 | 2.666 → 2.559 (-4.01%) | usable; False |

### capacity / x64 / pgo-cpu

| Workload | CPU % B → C | RSS MiB B → C | Mbps B → C | CPU ns/B B → C (change) | Status / capacity rank |
|---|---:|---:|---:|---:|---|
| stealth-v4-http-1 | 74.707 → 66.876 | 5.068 → 4.810 | 2475.375 → 2451.982 | 2.414 → 2.182 (-9.63%) | usable; False |
| stealth-v4-http-8 | 76.842 → 71.986 | 5.779 → 5.343 | 2230.627 → 2244.527 | 2.756 → 2.566 (-6.90%) | usable; False |
| stealth-v6-https-1 | 74.359 → 65.675 | 5.175 → 4.834 | 2258.056 → 2244.796 | 2.634 → 2.341 (-11.16%) | usable; False |
| stealth-v6-https-8 | 76.593 → 68.278 | 5.314 → 5.000 | 2039.950 → 2029.523 | 3.004 → 2.691 (-10.40%) | usable; False |
| tcp-v4-http-1 | 72.825 → 65.027 | 7.398 → 7.674 | 2696.410 → 2685.513 | 2.161 → 1.937 (-10.35%) | usable; False |
| tcp-v4-http-8 | 75.349 → 68.590 | 8.239 → 7.647 | 2435.959 → 2431.455 | 2.475 → 2.257 (-8.80%) | usable; False |
| tcp-v6-https-1 | 72.741 → 63.839 | 7.205 → 6.794 | 2457.586 → 2455.309 | 2.368 → 2.080 (-12.16%) | usable; False |
| tcp-v6-https-8 | 74.136 → 66.755 | 7.650 → 7.955 | 2218.035 → 2213.189 | 2.674 → 2.413 (-9.76%) | usable; False |
| udp-v4-http-1 | 69.466 → 63.958 | 8.172 → 7.880 | 2544.728 → 2618.840 | 2.184 → 1.954 (-10.53%) | usable; False |
| udp-v4-http-8 | 72.684 → 67.163 | 8.331 → 8.307 | 2354.341 → 2368.049 | 2.470 → 2.269 (-8.13%) | usable; False |
| udp-v6-https-1 | 69.722 → 62.072 | 8.413 → 8.219 | 2377.717 → 2362.846 | 2.346 → 2.102 (-10.41%) | usable; False |
| udp-v6-https-8 | 71.530 → 64.695 | 7.838 → 8.518 | 2130.938 → 2139.656 | 2.685 → 2.419 (-9.92%) | usable; False |

### capacity / arm / pgo

| Workload | CPU % B → C | RSS MiB B → C | Mbps B → C | CPU ns/B B → C (change) | Status / capacity rank |
|---|---:|---:|---:|---:|---|
| stealth-v4-http-1 | 85.846 → 80.462 | 4.535 → 4.347 | 2664.367 → 2645.101 | 2.578 → 2.434 (-5.59%) | usable; False |
| stealth-v4-http-8 | 88.263 → 83.584 | 4.785 → 4.775 | 2551.793 → 2527.810 | 2.767 → 2.645 (-4.40%) | usable; False |
| stealth-v6-https-1 | 86.829 → 81.977 | 4.532 → 4.535 | 2558.327 → 2557.247 | 2.715 → 2.565 (-5.55%) | usable; False |
| stealth-v6-https-8 | 88.770 → 81.448 | 4.972 → 4.660 | 2435.957 → 2402.365 | 2.915 → 2.712 (-6.97%) | usable; False |
| tcp-v4-http-1 | 84.722 → 82.009 | 6.668 → 6.730 | 3205.265 → 3221.245 | 2.115 → 2.037 (-3.68%) | usable; False |
| tcp-v4-http-8 | 85.395 → 83.869 | 7.784 → 7.409 | 3038.546 → 3052.623 | 2.248 → 2.198 (-2.24%) | usable; False |
| tcp-v6-https-1 | 85.303 → 81.443 | 7.034 → 7.139 | 3106.760 → 3083.129 | 2.197 → 2.113 (-3.79%) | usable; False |
| tcp-v6-https-8 | 87.329 → 83.143 | 7.279 → 7.158 | 2931.010 → 2949.573 | 2.384 → 2.255 (-5.39%) | usable; False |
| udp-v4-http-1 | 85.410 → 83.959 | 7.656 → 7.340 | 3079.201 → 3101.488 | 2.219 → 2.166 (-2.41%) | usable; False |
| udp-v4-http-8 | 87.244 → 84.516 | 7.699 → 6.916 | 2913.596 → 2887.009 | 2.396 → 2.342 (-2.23%) | usable; False |
| udp-v6-https-1 | 85.875 → 82.670 | 7.523 → 7.389 | 2931.631 → 2936.981 | 2.343 → 2.252 (-3.91%) | usable; False |
| udp-v6-https-8 | 89.629 → 85.511 | 6.986 → 7.597 | 2833.491 → 2816.104 | 2.531 → 2.429 (-4.01%) | usable; False |

### capacity / arm / cpu

| Workload | CPU % B → C | RSS MiB B → C | Mbps B → C | CPU ns/B B → C (change) | Status / capacity rank |
|---|---:|---:|---:|---:|---|
| stealth-v4-http-1 | 83.923 → 84.015 | 4.658 → 4.285 | 2605.746 → 2608.827 | 2.577 → 2.576 (-0.01%) | usable; False |
| stealth-v4-http-8 | 85.567 → 83.394 | 5.023 → 5.097 | 2532.678 → 2502.924 | 2.703 → 2.666 (-1.38%) | usable; False |
| stealth-v6-https-1 | 85.070 → 83.453 | 4.597 → 4.341 | 2547.147 → 2553.272 | 2.672 → 2.615 (-2.14%) | usable; False |
| stealth-v6-https-8 | 88.828 → 87.411 | 4.847 → 4.493 | 2451.956 → 2464.477 | 2.898 → 2.837 (-2.10%) | usable; False |
| tcp-v4-http-1 | 83.756 → 83.142 | 6.955 → 7.083 | 3156.946 → 3185.999 | 2.122 → 2.088 (-1.64%) | usable; False |
| tcp-v4-http-8 | 86.674 → 84.784 | 7.097 → 7.343 | 3081.581 → 3061.318 | 2.250 → 2.216 (-1.53%) | usable; False |
| tcp-v6-https-1 | 84.735 → 83.831 | 7.026 → 6.842 | 3092.924 → 3092.011 | 2.192 → 2.169 (-1.04%) | usable; False |
| tcp-v6-https-8 | 87.358 → 84.537 | 7.234 → 7.220 | 2918.642 → 2899.370 | 2.394 → 2.333 (-2.59%) | usable; False |
| udp-v4-http-1 | 84.635 → 83.187 | 7.590 → 7.285 | 3103.022 → 3078.969 | 2.182 → 2.161 (-0.94%) | usable; False |
| udp-v4-http-8 | 86.319 → 85.786 | 7.722 → 7.722 | 2905.149 → 2927.308 | 2.377 → 2.344 (-1.37%) | usable; False |
| udp-v6-https-1 | 86.099 → 83.755 | 7.502 → 7.715 | 2948.850 → 2921.805 | 2.336 → 2.293 (-1.82%) | usable; False |
| udp-v6-https-8 | 88.452 → 87.788 | 7.420 → 7.182 | 2844.748 → 2843.298 | 2.487 → 2.470 (-0.70%) | usable; False |

### capacity / arm / pgo-cpu

| Workload | CPU % B → C | RSS MiB B → C | Mbps B → C | CPU ns/B B → C (change) | Status / capacity rank |
|---|---:|---:|---:|---:|---|
| stealth-v4-http-1 | 85.894 → 80.672 | 4.472 → 4.466 | 2653.389 → 2658.596 | 2.590 → 2.428 (-6.26%) | usable; False |
| stealth-v4-http-8 | 88.267 → 82.831 | 4.887 → 4.784 | 2544.931 → 2557.054 | 2.775 → 2.591 (-6.60%) | usable; False |
| stealth-v6-https-1 | 85.893 → 80.801 | 4.527 → 4.341 | 2567.036 → 2553.921 | 2.677 → 2.531 (-5.44%) | usable; False |
| stealth-v6-https-8 | 87.321 → 81.685 | 4.910 → 4.722 | 2431.112 → 2408.860 | 2.873 → 2.713 (-5.59%) | usable; False |
| tcp-v4-http-1 | 83.675 → 79.806 | 6.841 → 6.827 | 3203.282 → 3183.514 | 2.090 → 2.005 (-4.03%) | usable; False |
| tcp-v4-http-8 | 86.340 → 82.356 | 6.707 → 6.970 | 3051.572 → 3045.175 | 2.263 → 2.164 (-4.41%) | usable; False |
| tcp-v6-https-1 | 84.550 → 80.606 | 6.785 → 6.915 | 3086.470 → 3079.895 | 2.192 → 2.094 (-4.46%) | usable; False |
| tcp-v6-https-8 | 87.654 → 83.294 | 7.037 → 6.998 | 2957.235 → 2943.047 | 2.371 → 2.264 (-4.52%) | usable; False |
| udp-v4-http-1 | 86.670 → 82.661 | 7.714 → 7.260 | 3086.430 → 3073.246 | 2.246 → 2.152 (-4.22%) | usable; False |
| udp-v4-http-8 | 88.282 → 82.133 | 7.970 → 6.934 | 2911.383 → 2858.578 | 2.426 → 2.299 (-5.25%) | usable; False |
| udp-v6-https-1 | 86.572 → 81.012 | 7.785 → 6.905 | 2960.698 → 2906.804 | 2.339 → 2.230 (-4.69%) | usable; False |
| udp-v6-https-8 | 88.910 → 84.434 | 7.601 → 7.398 | 2845.488 → 2788.957 | 2.500 → 2.422 (-3.11%) | usable; False |

### delay / x64 / pgo

| Workload | CPU % B → C | RSS MiB B → C | Mbps B → C | CPU ns/B B → C (change) | Status / capacity rank |
|---|---:|---:|---:|---:|---|
| stealth-v4-http-1 | 8.261 → 7.685 | 5.032 → 4.718 | 199.696 → 199.694 | 3.309 → 3.079 (-6.97%) | usable; False |
| stealth-v4-http-8 | 6.743 → 6.505 | 6.488 → 6.156 | 200.332 → 200.327 | 2.693 → 2.598 (-3.53%) | usable; False |
| stealth-v6-https-1 | 8.334 → 7.889 | 5.229 → 4.781 | 199.698 → 199.697 | 3.339 → 3.160 (-5.34%) | usable; False |
| stealth-v6-https-8 | 6.963 → 6.695 | 5.602 → 5.230 | 200.329 → 200.332 | 2.781 → 2.673 (-3.86%) | usable; False |
| tcp-v4-http-1 | 7.476 → 7.063 | 7.961 → 7.866 | 199.700 → 199.698 | 2.995 → 2.829 (-5.53%) | usable; False |
| tcp-v4-http-8 | 6.203 → 6.150 | 8.591 → 8.638 | 200.340 → 200.339 | 2.477 → 2.456 (-0.86%) | usable; False |
| tcp-v6-https-1 | n/a | n/a | n/a | n/a | invalid/incomplete |
| tcp-v6-https-8 | 6.441 → 6.096 | 8.217 → 7.596 | 200.332 → 200.331 | 2.572 → 2.434 (-5.36%) | usable; False |
| udp-v4-http-1 | 7.722 → 7.390 | 8.417 → 8.254 | 199.697 → 199.699 | 3.094 → 2.960 (-4.31%) | usable; False |
| udp-v4-http-8 | 6.319 → 6.012 | 8.756 → 8.721 | 200.337 → 200.339 | 2.523 → 2.401 (-4.86%) | usable; False |
| udp-v6-https-1 | 8.305 → 8.024 | 8.245 → 7.768 | 199.697 → 199.698 | 3.327 → 3.215 (-3.38%) | usable; False |
| udp-v6-https-8 | 6.464 → 6.374 | 8.348 → 8.620 | 200.333 → 200.330 | 2.581 → 2.546 (-1.38%) | usable; False |

### delay / x64 / cpu

| Workload | CPU % B → C | RSS MiB B → C | Mbps B → C | CPU ns/B B → C (change) | Status / capacity rank |
|---|---:|---:|---:|---:|---|
| stealth-v4-http-1 | 8.307 → 8.082 | 5.038 → 5.019 | 199.696 → 199.696 | 3.328 → 3.238 (-2.71%) | usable; False |
| stealth-v4-http-8 | 6.767 → 6.465 | 5.970 → 5.906 | 200.325 → 200.334 | 2.702 → 2.582 (-4.47%) | usable; False |
| stealth-v6-https-1 | 8.439 → 8.129 | 5.148 → 5.031 | 199.695 → 199.695 | 3.381 → 3.256 (-3.68%) | usable; False |
| stealth-v6-https-8 | 6.899 → 6.815 | 5.584 → 5.621 | 200.330 → 200.331 | 2.755 → 2.722 (-1.21%) | usable; False |
| tcp-v4-http-1 | 7.441 → 7.238 | 7.250 → 7.706 | 199.689 → 199.700 | 2.981 → 2.900 (-2.73%) | usable; False |
| tcp-v4-http-8 | 6.110 → 6.068 | 8.727 → 8.577 | 200.339 → 200.338 | 2.440 → 2.423 (-0.67%) | inconclusive-control/timing; False |
| tcp-v6-https-1 | 7.859 → 7.589 | 7.152 → 7.365 | 199.694 → 199.695 | 3.148 → 3.040 (-3.43%) | usable; False |
| tcp-v6-https-8 | 6.418 → 6.810 | 7.982 → 8.984 | 200.332 → 200.328 | 2.563 → 2.719 (+6.11%) | usable; False |
| udp-v4-http-1 | 7.613 → 7.388 | 7.930 → 8.719 | 199.700 → 199.699 | 3.050 → 2.960 (-2.96%) | usable; False |
| udp-v4-http-8 | 6.227 → 6.002 | 8.735 → 8.392 | 200.342 → 200.341 | 2.486 → 2.397 (-3.61%) | usable; False |
| udp-v6-https-1 | 8.399 → 8.133 | 8.309 → 8.151 | 199.695 → 199.699 | 3.365 → 3.258 (-3.17%) | usable; False |
| udp-v6-https-8 | 6.518 → 6.287 | 8.772 → 8.572 | 200.330 → 200.331 | 2.603 → 2.511 (-3.54%) | usable; False |

### delay / x64 / pgo-cpu

| Workload | CPU % B → C | RSS MiB B → C | Mbps B → C | CPU ns/B B → C (change) | Status / capacity rank |
|---|---:|---:|---:|---:|---|
| stealth-v4-http-1 | 8.323 → 7.487 | 5.148 → 4.870 | 199.695 → 199.697 | 3.334 → 2.999 (-10.04%) | usable; False |
| stealth-v4-http-8 | 6.703 → 6.285 | 6.385 → 5.924 | 200.335 → 200.337 | 2.677 → 2.510 (-6.24%) | usable; False |
| stealth-v6-https-1 | 8.493 → 7.461 | 5.055 → 4.849 | 199.697 → 199.696 | 3.402 → 2.989 (-12.15%) | usable; False |
| stealth-v6-https-8 | 6.890 → 6.275 | 5.697 → 5.520 | 200.335 → 200.335 | 2.751 → 2.506 (-8.93%) | usable; False |
| tcp-v4-http-1 | 7.505 → 6.807 | 7.461 → 7.372 | 199.698 → 199.702 | 3.006 → 2.727 (-9.30%) | usable; False |
| tcp-v4-http-8 | 6.310 → 5.710 | 8.768 → 8.455 | 200.336 → 200.341 | 2.520 → 2.280 (-9.52%) | usable; False |
| tcp-v6-https-1 | 7.839 → 7.080 | 8.023 → 7.524 | 199.695 → 199.696 | 3.140 → 2.836 (-9.68%) | inconclusive-control/timing; False |
| tcp-v6-https-8 | 6.572 → 6.047 | 8.211 → 8.021 | 200.329 → 200.337 | 2.624 → 2.415 (-7.99%) | usable; False |
| udp-v4-http-1 | 7.681 → 6.953 | 8.366 → 7.741 | 199.698 → 199.699 | 3.077 → 2.785 (-9.48%) | usable; False |
| udp-v4-http-8 | 6.276 → 5.926 | 9.042 → 8.399 | 200.338 → 200.335 | 2.506 → 2.366 (-5.57%) | usable; False |
| udp-v6-https-1 | 8.316 → 7.480 | 8.586 → 8.235 | 199.700 → 199.700 | 3.332 → 2.997 (-10.05%) | usable; False |
| udp-v6-https-8 | 6.454 → 6.156 | 8.828 → 8.676 | 200.332 → 200.331 | 2.577 → 2.458 (-4.62%) | usable; False |

### delay / arm / pgo

| Workload | CPU % B → C | RSS MiB B → C | Mbps B → C | CPU ns/B B → C (change) | Status / capacity rank |
|---|---:|---:|---:|---:|---|
| stealth-v4-http-1 | 6.395 → 6.088 | 4.584 → 4.285 | 199.705 → 199.705 | 2.562 → 2.439 (-4.80%) | usable; False |
| stealth-v4-http-8 | 6.346 → 6.169 | 5.847 → 5.417 | 200.343 → 200.344 | 2.534 → 2.463 (-2.79%) | usable; False |
| stealth-v6-https-1 | 6.861 → 6.555 | 4.467 → 4.279 | 199.701 → 199.703 | 2.749 → 2.626 (-4.46%) | usable; False |
| stealth-v6-https-8 | 6.629 → 6.332 | 4.969 → 4.778 | 200.334 → 200.335 | 2.647 → 2.529 (-4.48%) | usable; False |
| tcp-v4-http-1 | 5.234 → 5.047 | 7.070 → 7.134 | 199.706 → 199.705 | 2.097 → 2.022 (-3.58%) | usable; False |
| tcp-v4-http-8 | 5.166 → 5.130 | 8.846 → 7.456 | 200.353 → 200.353 | 2.063 → 2.049 (-0.69%) | usable; False |
| tcp-v6-https-1 | 5.776 → 5.388 | 7.113 → 6.545 | 199.670 → 199.702 | 2.314 → 2.159 (-6.73%) | usable; False |
| tcp-v6-https-8 | 5.371 → 5.241 | 7.458 → 7.577 | 200.340 → 200.346 | 2.145 → 2.093 (-2.42%) | usable; False |
| udp-v4-http-1 | 5.702 → 5.349 | 7.170 → 7.331 | 199.706 → 199.705 | 2.284 → 2.143 (-6.18%) | usable; False |
| udp-v4-http-8 | 5.460 → 5.396 | 8.071 → 7.975 | 200.346 → 200.352 | 2.180 → 2.155 (-1.17%) | usable; False |
| udp-v6-https-1 | 5.949 → 5.805 | 7.062 → 7.013 | 199.704 → 199.702 | 2.383 → 2.325 (-2.42%) | usable; False |
| udp-v6-https-8 | 5.717 → 5.512 | 7.904 → 7.627 | 200.340 → 200.345 | 2.283 → 2.201 (-3.59%) | usable; False |

### delay / arm / cpu

| Workload | CPU % B → C | RSS MiB B → C | Mbps B → C | CPU ns/B B → C (change) | Status / capacity rank |
|---|---:|---:|---:|---:|---|
| stealth-v4-http-1 | 6.391 → 6.363 | 4.533 → 4.347 | 199.705 → 199.705 | 2.560 → 2.549 (-0.43%) | usable; False |
| stealth-v4-http-8 | 6.334 → 6.225 | 5.686 → 5.215 | 200.342 → 200.344 | 2.529 → 2.486 (-1.72%) | usable; False |
| stealth-v6-https-1 | 7.048 → 6.739 | 4.535 → 4.472 | 199.702 → 199.703 | 2.823 → 2.700 (-4.38%) | usable; False |
| stealth-v6-https-8 | 6.561 → 6.438 | 4.961 → 4.779 | 200.339 → 200.338 | 2.620 → 2.571 (-1.87%) | usable; False |
| tcp-v4-http-1 | 5.229 → 5.140 | 6.526 → 6.937 | 199.707 → 199.707 | 2.095 → 2.059 (-1.70%) | usable; False |
| tcp-v4-http-8 | 5.148 → 5.208 | 8.471 → 7.902 | 200.353 → 200.352 | 2.055 → 2.080 (+1.17%) | usable; False |
| tcp-v6-https-1 | 5.698 → 5.676 | 6.972 → 7.159 | 199.703 → 199.705 | 2.283 → 2.274 (-0.39%) | usable; False |
| tcp-v6-https-8 | 5.513 → 5.325 | 7.411 → 7.341 | 200.343 → 200.346 | 2.201 → 2.126 (-3.41%) | usable; False |
| udp-v4-http-1 | 5.701 → 5.395 | 7.091 → 7.722 | 199.706 → 199.706 | 2.284 → 2.161 (-5.37%) | usable; False |
| udp-v4-http-8 | 5.544 → 5.406 | 7.967 → 7.967 | 200.350 → 200.349 | 2.214 → 2.159 (-2.48%) | usable; False |
| udp-v6-https-1 | 6.084 → 5.877 | 7.782 → 7.266 | 199.704 → 199.705 | 2.437 → 2.354 (-3.39%) | usable; False |
| udp-v6-https-8 | 5.668 → 5.564 | 7.651 → 7.775 | 200.340 → 200.343 | 2.263 → 2.222 (-1.83%) | usable; False |

### delay / arm / pgo-cpu

| Workload | CPU % B → C | RSS MiB B → C | Mbps B → C | CPU ns/B B → C (change) | Status / capacity rank |
|---|---:|---:|---:|---:|---|
| stealth-v4-http-1 | 6.365 → 6.008 | 4.465 → 4.285 | 199.705 → 199.706 | 2.550 → 2.407 (-5.62%) | usable; False |
| stealth-v4-http-8 | 6.275 → 6.039 | 5.625 → 5.657 | 200.345 → 200.345 | 2.506 → 2.411 (-3.76%) | usable; False |
| stealth-v6-https-1 | 6.859 → 6.353 | 4.465 → 4.410 | 199.703 → 199.702 | 2.748 → 2.545 (-7.39%) | usable; False |
| stealth-v6-https-8 | 6.464 → 6.241 | 5.090 → 4.845 | 200.338 → 200.336 | 2.581 → 2.492 (-3.45%) | usable; False |
| tcp-v4-http-1 | 5.324 → 4.943 | 7.341 → 6.968 | 199.705 → 199.707 | 2.133 → 1.980 (-7.16%) | usable; False |
| tcp-v4-http-8 | 5.294 → 5.027 | 7.513 → 8.012 | 200.350 → 200.352 | 2.114 → 2.007 (-5.05%) | usable; False |
| tcp-v6-https-1 | n/a | n/a | n/a | n/a | invalid/incomplete |
| tcp-v6-https-8 | 5.489 → 5.188 | 7.696 → 7.331 | 200.346 → 200.348 | 2.192 → 2.072 (-5.48%) | usable; False |
| udp-v4-http-1 | 5.478 → 5.068 | 7.638 → 7.972 | 199.706 → 199.706 | 2.194 → 2.030 (-7.48%) | usable; False |
| udp-v4-http-8 | 5.479 → 5.271 | 7.587 → 7.909 | 200.351 → 200.352 | 2.188 → 2.105 (-3.79%) | usable; False |
| udp-v6-https-1 | 5.985 → 5.688 | 7.584 → 7.025 | 199.704 → 199.703 | 2.398 → 2.279 (-4.96%) | usable; False |
| udp-v6-https-8 | 5.657 → 5.420 | 7.811 → 7.737 | 200.344 → 200.342 | 2.259 → 2.164 (-4.19%) | usable; False |
