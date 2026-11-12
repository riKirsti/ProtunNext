# Stage 6: client PGO and platform builds

Frozen baseline: merged PR #10 `d6c070a0f9e211f5a5b14b7e8c8bc0f995d7b7a7`.
[Transition review](../performance-stages.md#stage-6-review-november-12) preceded implementation.

**Selection:** retain two optional, separately measured build experiments for
further acceptance: client-only PGO and explicit CPU-specific compilation.
Portable release defaults, production Rust/vendor source and dependency versions
remain unchanged. Neither experiment is a universal deployment recommendation.
No combined build, backend, helper thread, persistent device setting or release.

All 384 full-matrix native downloads verify; all 48 cells pass A/A and timing
screens. PGO reduces CPU/byte in every native cell in this cohort. CPU-specific
x64 reduces CPU/byte in every cell; ARM64 includes one +0.95% regression alongside
reductions. Memory tradeoffs remain visible below. Paced 200 Mbps results establish
resource behavior at that load, **not higher maximum download speed**. The two
experiments ran on separate VM allocations; compare each with its own baseline,
not against the other experiment's absolute numbers.

## Native resource summary

Client process only; CPU 100% = one core, memory = peak RSS MiB, throughput =
verified application Mbps. Values are geometric means of four workload cells per
transport (IPv4 HTTP / IPv6 HTTPS × one/eight flows). Each cell itself uses two
baseline and two candidate balanced samples, preceded by four baseline A/A
observations. See every cell, control status and source/build identity below and
in [machine-readable results](stage6-data/index.json). Summaries do not replace
the individual cells or describe the peer/load/server's resource consumption.

| Build / platform | Transport | CPU % B → C | RSS MiB B → C | Mbps B → C | CPU/byte change |
|---|---|---:|---:|---:|---:|
| PGO / Linux x64 | udp | 7.049 → 6.719 | 8.242 → 8.050 | 200.382 → 200.374 | -4.67% |
| PGO / Linux x64 | tcp | 7.118 → 6.770 | 7.444 → 7.366 | 200.381 → 200.383 | -4.89% |
| PGO / Linux x64 | stealth | 7.837 → 7.577 | 5.173 → 4.972 | 200.380 → 200.380 | -3.32% |
| PGO / Linux ARM64 | udp | 5.794 → 5.699 | 7.272 → 7.050 | 200.398 → 200.397 | -1.63% |
| PGO / Linux ARM64 | tcp | 5.626 → 5.477 | 6.782 → 6.863 | 200.399 → 200.399 | -2.66% |
| PGO / Linux ARM64 | stealth | 6.791 → 6.443 | 4.657 → 4.514 | 200.393 → 200.393 | -5.13% |
| CPU / Linux x64 | udp | 6.747 → 6.482 | 7.856 → 7.964 | 200.386 → 200.386 | -3.93% |
| CPU / Linux x64 | tcp | 6.747 → 6.463 | 7.421 → 7.430 | 200.388 → 200.386 | -4.21% |
| CPU / Linux x64 | stealth | 7.369 → 7.142 | 5.180 → 5.174 | 200.385 → 200.383 | -3.09% |
| CPU / Linux ARM64 | udp | 5.905 → 5.831 | 7.480 → 7.367 | 200.398 → 200.398 | -1.26% |
| CPU / Linux ARM64 | tcp | 5.667 → 5.549 | 6.693 → 6.730 | 200.396 → 200.398 | -2.08% |
| CPU / Linux ARM64 | stealth | 6.810 → 6.684 | 4.650 → 4.550 | 200.393 → 200.393 | -1.86% |

Full native sources are `b7d667325b3110d492af009b7270f569735bfe46` versus the
frozen PR #10 baseline. Production runtime fingerprints match:
`5d448b4aabf1041e3a13920294314097f88a30eeb9a2d4c799724c2c04112fc7`.
The later `b154626` change only retains additional LTS artifacts; it does not
change tested runtime or compiler logic. Native x64 is AMD EPYC 7763, ARM64 is
Neoverse-N2 / Cobalt 100; both use kernel 7.0.0-1017-azure and four available
CPUs. Client, peer, load and server each have a separate pinned CPU. Both variants
use vnet, the existing serial receiver and ring. Fixed baseline binaries serve
all peer/server/load roles; actual running executable hashes are checked, in
addition to source IDs. TLS-record and parallel-receive prototypes are disabled.

Rust 1.99.1 / LLVM 23.1.1, release thin LTO, codegen-units=1, panic=abort and
`--no-default-features --features unix` apply to these controlled comparisons.
The identical diagnostic overlay is injected into both frozen archives.
Executable hashes are cached before setup/warmup rather than extending timing.

## Build provenance and correctness

The [reproducible experiment](../../experiments/build/README.md) records explicit
compiler/target/flags, source and executable hashes, CPU features and profiles in
each result's environment and GitHub artifact. Equal runtime source is expected
for this comparison; different flags and verified final binary hashes establish
the build difference. No portable release is silently replaced by an optimized
artifact. Native CPU mode uses `-Ctarget-cpu=native`, records target features and
checks the host CPU identity again at evaluation. Those binaries are not portable
and must not be copied to the Cortex-A53 Travel router.

PGO freshly trains only the instrumented client. Six verified cases cover all
transports, IPv4 HTTP/two flows and IPv6 HTTPS/four flows, versus held-out one/eight
flow evaluations. Helpers stay uninstrumented baseline. Matching LLVM tools merge
nonempty counters; hashes bind raw/merged data and generated/final binaries.
Final builds consume that exact profile and have no generation counter section.
Instrumented data is training, never a speed estimate. Missing-profile warnings
remain in build logs; training does not cover every function or error path.

Every final native/LTS optimized build runs release library regressions and
linked Drop/recycle erasure inspection, then rejects an isolated control that
omits the entire wipe. The executable fixture alone is not a memory-readback
proof; the linked checker is a regression tripwire, not formal proof. Allocation
zero-initialization is explicitly excluded from erasure evidence. Device builds
also pass linked inspection and reject their own missing-wipe controls; they are
cross-compiled, with functional execution supplied by the actual router tests.

[Nine platform regression jobs](https://github.com/riKirsti/ProtunNext/actions/runs/43241169614),
[release erasure verification](https://github.com/riKirsti/ProtunNext/actions/runs/43241169078)
and [native fixture/analyzer checks](https://github.com/riKirsti/ProtunNext/actions/runs/43241168602)
are the validation runs for the final experiment logic. Their actual terminal
statuses must be checked before this PR is ready. Windows, Apple and Android
have build/API-contract coverage, **no measured download performance** here.
No claim is extrapolated from Linux. Analyzer suites contain 18 download and 12
audit tests; retained workflow logs also cover injected runtime/socket tests.

## Supported LTS and physical device scope

Portable x64-musl PGO is separately exercised in Linux 6.12.115 and 6.18.60 guests,
focused UDP/IPv4 HTTP/one flow, unpaced 2 GiB downloads. UDP/TCP/Stealth native
coverage does not imply TCP/Stealth LTS performance coverage. All 48 timed LTS
downloads across three separately retained cohorts verify; all six cells are
auxiliary-limited. The final 6.18 repeat also fails its A/A CPU stability screen,
so it is inconclusive despite verified bytes and lower observed candidate CPU.
Neither its speed nor any helper-limited result establishes maximum client
capacity. Earlier successful repeats are not pooled to hide that failure.
The first two LTS runs retained metadata/logs but omitted raw profiles from
uploaded artifacts; the final repeat retains counters, merged profile, binaries
and disassembly. This retention correction is the only `b154626` change.

The [temporary physical Travel comparisons](stage6-travel.md) verify all 48
timed downloads, with twelve independent restoration checks passing after each
comparison. Cortex-A53 CPU/byte is higher for every transport. Device PGO is
mixed: lower UDP/Stealth CPU/byte, higher TCP CPU/byte, with failed UDP/TCP WAN
stability controls. Stealth's promising observation includes higher RSS and is
not a repeatable maximum-capacity finding. Neither becomes a universal Travel
build recommendation. These tests evaluate explicit
Cortex-A53 compilation and independently trained portable PGO against the same
frozen release baseline over the saved Proton endpoint. They are separate WAN
cohorts, not a PGO-versus-CPU head-to-head. IPv4 HTTPS and a laptop load are used;
device IPv6/multiple evaluation flows remain unmeasured. See absolute resource
tables, failures, warmups, binary/profile identities and independent restoration
there. Saved settings, packages, key/account and normal Direct path are preserved.

## Earlier, failed and superseded runs

- Initial PGO native focused cohorts (`6cbbc06`) and CPU focused cohorts
  (`3a4e735`) are preliminary, with earlier erasure gate coverage; their 32
  observations and absolute values are retained below, never pooled with full
  results. Initial CPU ARM64 adds eight valid observations with the original
  positive-only gate; it is also preliminary.
- [Initial CPU run](https://github.com/riKirsti/ProtunNext/actions/runs/43231795812)
  rejects x64 before timing because linked recycle clearing was unrecognized.
  No x64 speed measurement or lost-erasure conclusion follows. The follow-up
  retains optimized disassembly/binaries and passes; all later optimized builds
  require the stronger positive/negative gate. The initial failure remains.
- [Gate-2 CPU run](https://github.com/riKirsti/ProtunNext/actions/runs/43235480532)
  rejects ARM64 before timing: merely removing the optimization barrier still
  leaves an actual final zero memset on that compiler/CPU. That mutation was not
  a missing-wipe negative. The corrected gate removes both fill and barrier in
  an isolated control, and final CPU/PGO builds reject it. This was not production
  erasure loss. No insecure control is deployed or timed.
- That run and [gate-2 full PGO](https://github.com/riKirsti/ProtunNext/actions/runs/43235476400)
  were superseded/cancelled by the final gate-3 runs. Partial artifacts/logs remain
  on GitHub; they are not complete matrix cohorts or included in the 472 timed
  observations tabulated here. Count them separately from accepted results.
- Router SFTP negotiation initially fails before a training case; owned cleanup
  and twelve independent checks pass. The supported SSH transfer retry completes
  six verified cases; its failed UDP warmup remains in evidence. No instrumented
  timing is used for selection.

## Reproduction and next boundary

Dispatch `native-download-validation` with frozen baseline, `build_mode=pgo` or
`cpu`, `run_downloads=true`, `tun_mode=vnet`, `client_cores=1`,
`rate_bps=200000000`, empty block, `parallel_receive=false`, `tls_records=false`
and `screen_crypto=false`. Use the supported-LTS workflow in `pgo` mode for
guest compatibility; never substitute the host-native CPU binary. Artifacts
retain raw samples, environments, builds, logs, profiles and linked inspections.
The [result index](stage6-data/index.json) identifies 15 complete cohorts and 472
timed observations, with full values below. Raw GitHub artifacts are authoritative
for individual observations; the JSON files here retain analysis/provenance.

These results justify continued target-specific evaluation without making a
universal/default build change. No fixed CPU/RAM ceiling or invented minimum
improvement selects a winner. Review Stage 7 after Stage 6 merges: combine only
supported winners, compare each component and combination against the frozen
baseline, and validate sustained real-workload/platform behavior and capacity.
Higher maximum speed, production/profile reproducibility across updates, WAN
reliability and unmeasured platform acceptance remain work, not established facts.
Deferred Stage 5 architectures are not exhausted by these build experiments.

## Every retained workload

B = ordinary frozen release; C = the independently declared build candidate.
Numbers are observations, including preliminary and inconclusive cells. A usable
label means A/A and timing passed; auxiliary headroom is separately described
above and recorded in JSON. All LTS cells are auxiliary-limited.

### pgo-full-x64
[GitHub Actions run](https://github.com/riKirsti/ProtunNext/actions/runs/43238236344); 96/96 observations, 0 analyzer errors.

| Workload | CPU % B → C | RSS MiB B → C | Mbps B → C | CPU ns/byte B → C (change) | Validity |
|---|---:|---:|---:|---:|---|
| udp-v4-http-1 | 6.366 → 6.063 | 7.782 → 7.749 | 200.074 → 200.075 | 2.545 → 2.424 (-4.76%) | usable |
| udp-v4-http-8 | 7.387 → 7.024 | 9.164 → 8.597 | 200.694 → 200.699 | 2.944 → 2.800 (-4.91%) | usable |
| udp-v6-https-1 | 6.710 → 6.376 | 7.730 → 7.670 | 200.073 → 200.035 | 2.683 → 2.550 (-4.96%) | usable |
| udp-v6-https-8 | 7.824 → 7.508 | 8.373 → 8.221 | 200.688 → 200.688 | 3.119 → 2.993 (-4.04%) | usable |
| tcp-v4-http-1 | 6.436 → 5.882 | 7.111 → 6.978 | 200.072 → 200.076 | 2.574 → 2.352 (-8.61%) | usable |
| tcp-v4-http-8 | 7.345 → 7.044 | 7.805 → 7.781 | 200.697 → 200.697 | 2.928 → 2.808 (-4.10%) | usable |
| tcp-v6-https-1 | 6.947 → 6.632 | 7.309 → 7.127 | 200.069 → 200.072 | 2.778 → 2.652 (-4.53%) | usable |
| tcp-v6-https-8 | 7.815 → 7.643 | 7.568 → 7.610 | 200.686 → 200.688 | 3.115 → 3.047 (-2.19%) | usable |
| stealth-v4-http-1 | 7.336 → 7.077 | 4.894 → 4.785 | 200.072 → 200.072 | 2.933 → 2.830 (-3.53%) | usable |
| stealth-v4-http-8 | 7.930 → 7.825 | 5.704 → 5.527 | 200.693 → 200.698 | 3.161 → 3.119 (-1.32%) | usable |
| stealth-v6-https-1 | 7.694 → 7.197 | 4.900 → 4.663 | 200.068 → 200.072 | 3.077 → 2.878 (-6.46%) | usable |
| stealth-v6-https-8 | 8.430 → 8.270 | 5.237 → 4.954 | 200.688 → 200.680 | 3.360 → 3.297 (-1.90%) | usable |

### pgo-full-arm
[GitHub Actions run](https://github.com/riKirsti/ProtunNext/actions/runs/43238236344); 96/96 observations, 0 analyzer errors.

| Workload | CPU % B → C | RSS MiB B → C | Mbps B → C | CPU ns/byte B → C (change) | Validity |
|---|---:|---:|---:|---:|---|
| udp-v4-http-1 | 5.784 → 5.580 | 6.923 → 6.673 | 200.079 → 200.080 | 2.313 → 2.231 (-3.53%) | usable |
| udp-v4-http-8 | 5.589 → 5.541 | 7.253 → 6.928 | 200.720 → 200.719 | 2.228 → 2.209 (-0.85%) | usable |
| udp-v6-https-1 | 6.048 → 5.954 | 7.274 → 7.099 | 200.078 → 200.074 | 2.418 → 2.381 (-1.54%) | usable |
| udp-v6-https-8 | 5.763 → 5.730 | 7.654 → 7.528 | 200.716 → 200.716 | 2.297 → 2.284 (-0.58%) | usable |
| tcp-v4-http-1 | 5.480 → 5.331 | 6.595 → 6.284 | 200.079 → 200.078 | 2.191 → 2.131 (-2.73%) | usable |
| tcp-v4-http-8 | 5.367 → 5.273 | 7.591 → 7.278 | 200.723 → 200.725 | 2.139 → 2.102 (-1.75%) | usable |
| tcp-v6-https-1 | 6.053 → 5.868 | 5.901 → 6.722 | 200.078 → 200.078 | 2.420 → 2.346 (-3.07%) | usable |
| tcp-v6-https-8 | 5.628 → 5.454 | 7.160 → 7.215 | 200.717 → 200.716 | 2.243 → 2.174 (-3.08%) | usable |
| stealth-v4-http-1 | 6.594 → 6.246 | 4.379 → 4.347 | 200.077 → 200.077 | 2.637 → 2.497 (-5.28%) | usable |
| stealth-v4-http-8 | 6.520 → 6.283 | 5.093 → 5.132 | 200.713 → 200.714 | 2.599 → 2.504 (-3.63%) | usable |
| stealth-v6-https-1 | 7.307 → 6.713 | 4.406 → 4.160 | 200.073 → 200.076 | 2.922 → 2.684 (-8.14%) | usable |
| stealth-v6-https-8 | 6.771 → 6.541 | 4.785 → 4.472 | 200.709 → 200.707 | 2.699 → 2.607 (-3.39%) | usable |

### cpu-full-x64
[GitHub Actions run](https://github.com/riKirsti/ProtunNext/actions/runs/43238240552); 96/96 observations, 0 analyzer errors.

| Workload | CPU % B → C | RSS MiB B → C | Mbps B → C | CPU ns/byte B → C (change) | Validity |
|---|---:|---:|---:|---:|---|
| udp-v4-http-1 | 6.194 → 5.934 | 7.207 → 7.594 | 200.074 → 200.074 | 2.477 → 2.373 (-4.19%) | usable |
| udp-v4-http-8 | 6.926 → 6.669 | 8.107 → 7.776 | 200.702 → 200.705 | 2.761 → 2.658 (-3.71%) | usable |
| udp-v6-https-1 | 6.500 → 6.245 | 7.880 → 8.088 | 200.072 → 200.073 | 2.599 → 2.497 (-3.91%) | usable |
| udp-v6-https-8 | 7.434 → 7.143 | 8.273 → 8.423 | 200.696 → 200.695 | 2.963 → 2.847 (-3.91%) | usable |
| tcp-v4-http-1 | 6.324 → 5.936 | 7.179 → 7.102 | 200.073 → 200.076 | 2.529 → 2.374 (-6.13%) | usable |
| tcp-v4-http-8 | 6.801 → 6.582 | 7.694 → 7.654 | 200.704 → 200.703 | 2.711 → 2.624 (-3.23%) | usable |
| tcp-v6-https-1 | 6.668 → 6.302 | 7.230 → 7.356 | 200.075 → 200.073 | 2.666 → 2.520 (-5.50%) | usable |
| tcp-v6-https-8 | 7.224 → 7.085 | 7.592 → 7.621 | 200.701 → 200.694 | 2.879 → 2.824 (-1.92%) | usable |
| stealth-v4-http-1 | 6.807 → 6.604 | 4.952 → 4.929 | 200.074 → 200.073 | 2.722 → 2.641 (-2.98%) | usable |
| stealth-v4-http-8 | 7.414 → 7.245 | 5.577 → 5.725 | 200.700 → 200.701 | 2.955 → 2.888 (-2.28%) | usable |
| stealth-v6-https-1 | 7.303 → 6.986 | 4.973 → 4.964 | 200.072 → 200.071 | 2.920 → 2.793 (-4.35%) | usable |
| stealth-v6-https-8 | 8.003 → 7.783 | 5.242 → 5.115 | 200.694 → 200.690 | 3.190 → 3.103 (-2.74%) | usable |

### cpu-full-arm
[GitHub Actions run](https://github.com/riKirsti/ProtunNext/actions/runs/43238240552); 96/96 observations, 0 analyzer errors.

| Workload | CPU % B → C | RSS MiB B → C | Mbps B → C | CPU ns/byte B → C (change) | Validity |
|---|---:|---:|---:|---:|---|
| udp-v4-http-1 | 5.838 → 5.718 | 7.191 → 6.909 | 200.079 → 200.079 | 2.334 → 2.286 (-2.06%) | usable |
| udp-v4-http-8 | 5.614 → 5.599 | 7.595 → 7.462 | 200.720 → 200.721 | 2.238 → 2.232 (-0.27%) | usable |
| udp-v6-https-1 | 6.345 → 6.241 | 7.192 → 7.172 | 200.078 → 200.078 | 2.537 → 2.496 (-1.63%) | usable |
| udp-v6-https-8 | 5.847 → 5.785 | 7.969 → 7.967 | 200.717 → 200.715 | 2.331 → 2.306 (-1.07%) | usable |
| tcp-v4-http-1 | 5.476 → 5.528 | 6.340 → 6.720 | 200.079 → 200.078 | 2.190 → 2.210 (+0.95%) | usable |
| tcp-v4-http-8 | 5.419 → 5.338 | 7.285 → 6.591 | 200.713 → 200.721 | 2.160 → 2.128 (-1.50%) | usable |
| tcp-v6-https-1 | 6.178 → 5.828 | 5.966 → 6.653 | 200.078 → 200.078 | 2.470 → 2.330 (-5.67%) | usable |
| tcp-v6-https-8 | 5.624 → 5.512 | 7.280 → 6.961 | 200.717 → 200.717 | 2.242 → 2.197 (-1.99%) | usable |
| stealth-v4-http-1 | 6.626 → 6.592 | 4.406 → 4.271 | 200.077 → 200.078 | 2.649 → 2.636 (-0.51%) | usable |
| stealth-v4-http-8 | 6.551 → 6.402 | 5.090 → 5.097 | 200.712 → 200.714 | 2.611 → 2.552 (-2.27%) | usable |
| stealth-v6-https-1 | 7.276 → 6.999 | 4.472 → 4.283 | 200.076 → 200.076 | 2.909 → 2.798 (-3.81%) | usable |
| stealth-v6-https-8 | 6.812 → 6.756 | 4.660 → 4.597 | 200.707 → 200.707 | 2.715 → 2.693 (-0.83%) | usable |

### pgo-focused-x64
[GitHub Actions run](https://github.com/riKirsti/ProtunNext/actions/runs/43231791802); 8/8 observations, 0 analyzer errors.

| Workload | CPU % B → C | RSS MiB B → C | Mbps B → C | CPU ns/byte B → C (change) | Validity |
|---|---:|---:|---:|---:|---|
| udp-v4-http-1 | 4.824 → 4.711 | 8.015 → 7.574 | 200.077 → 200.078 | 1.929 → 1.884 (-2.34%) | usable |

### pgo-focused-arm
[GitHub Actions run](https://github.com/riKirsti/ProtunNext/actions/runs/43231791802); 8/8 observations, 0 analyzer errors.

| Workload | CPU % B → C | RSS MiB B → C | Mbps B → C | CPU ns/byte B → C (change) | Validity |
|---|---:|---:|---:|---:|---|
| udp-v4-http-1 | 5.869 → 5.770 | 7.469 → 7.156 | 200.079 → 200.079 | 2.347 → 2.307 (-1.69%) | usable |

### cpu-capture-x64
[GitHub Actions run](https://github.com/riKirsti/ProtunNext/actions/runs/43232918817); 8/8 observations, 0 analyzer errors.

| Workload | CPU % B → C | RSS MiB B → C | Mbps B → C | CPU ns/byte B → C (change) | Validity |
|---|---:|---:|---:|---:|---|
| udp-v4-http-1 | 4.550 → 4.326 | 7.857 → 8.116 | 200.079 → 200.080 | 1.819 → 1.730 (-4.94%) | usable |

### cpu-capture-arm
[GitHub Actions run](https://github.com/riKirsti/ProtunNext/actions/runs/43232918817); 8/8 observations, 0 analyzer errors.

| Workload | CPU % B → C | RSS MiB B → C | Mbps B → C | CPU ns/byte B → C (change) | Validity |
|---|---:|---:|---:|---:|---|
| udp-v4-http-1 | 5.764 → 5.647 | 7.232 → 7.369 | 200.078 → 200.079 | 2.305 → 2.258 (-2.04%) | usable |

### cpu-initial-arm
[GitHub Actions run](https://github.com/riKirsti/ProtunNext/actions/runs/43231795812); 8/8 observations, 0 analyzer errors.

| Workload | CPU % B → C | RSS MiB B → C | Mbps B → C | CPU ns/byte B → C (change) | Validity |
|---|---:|---:|---:|---:|---|
| udp-v4-http-1 | 5.828 → 5.616 | 7.459 → 7.599 | 200.079 → 200.080 | 2.330 → 2.246 (-3.63%) | usable |

### lts-gate2-612
[GitHub Actions run](https://github.com/riKirsti/ProtunNext/actions/runs/43235487120); 8/8 observations, 0 analyzer errors.

| Workload | CPU % B → C | RSS MiB B → C | Mbps B → C | CPU ns/byte B → C (change) | Validity |
|---|---:|---:|---:|---:|---|
| udp-v4-http-1 | 92.117 → 87.629 | 4.226 → 4.269 | 4371.345 → 4261.293 | 1.686 → 1.645 (-2.42%) | usable |

### lts-gate2-618
[GitHub Actions run](https://github.com/riKirsti/ProtunNext/actions/runs/43235487120); 8/8 observations, 0 analyzer errors.

| Workload | CPU % B → C | RSS MiB B → C | Mbps B → C | CPU ns/byte B → C (change) | Validity |
|---|---:|---:|---:|---:|---|
| udp-v4-http-1 | 83.439 → 79.409 | 4.006 → 4.002 | 4149.402 → 4088.489 | 1.609 → 1.554 (-3.41%) | usable |

### lts-current-612
[GitHub Actions run](https://github.com/riKirsti/ProtunNext/actions/runs/43238244283); 8/8 observations, 0 analyzer errors.

| Workload | CPU % B → C | RSS MiB B → C | Mbps B → C | CPU ns/byte B → C (change) | Validity |
|---|---:|---:|---:|---:|---|
| udp-v4-http-1 | 66.939 → 65.177 | 3.951 → 3.918 | 2278.124 → 2315.786 | 2.351 → 2.252 (-4.22%) | usable |

### lts-current-618
[GitHub Actions run](https://github.com/riKirsti/ProtunNext/actions/runs/43238244283); 8/8 observations, 0 analyzer errors.

| Workload | CPU % B → C | RSS MiB B → C | Mbps B → C | CPU ns/byte B → C (change) | Validity |
|---|---:|---:|---:|---:|---|
| udp-v4-http-1 | 67.404 → 63.501 | 3.941 → 4.039 | 2419.887 → 2406.929 | 2.228 → 2.111 (-5.28%) | usable |

### lts-final-612
[GitHub Actions run](https://github.com/riKirsti/ProtunNext/actions/runs/43241232273); 8/8 observations, 0 analyzer errors.

| Workload | CPU % B → C | RSS MiB B → C | Mbps B → C | CPU ns/byte B → C (change) | Validity |
|---|---:|---:|---:|---:|---|
| udp-v4-http-1 | 92.255 → 88.080 | 4.168 → 4.173 | 4056.463 → 3910.564 | 1.819 → 1.802 (-0.96%) | usable |

### lts-final-618
[GitHub Actions run](https://github.com/riKirsti/ProtunNext/actions/runs/43241232273); 8/8 observations, 0 analyzer errors.

| Workload | CPU % B → C | RSS MiB B → C | Mbps B → C | CPU ns/byte B → C (change) | Validity |
|---|---:|---:|---:|---:|---|
| udp-v4-http-1 | 88.897 → 84.808 | 4.257 → 4.037 | 4807.273 → 4792.860 | 1.479 → 1.416 (-4.31%) | inconclusive-control/timing |
