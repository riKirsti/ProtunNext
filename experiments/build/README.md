# Stage 6 build experiments

No production source, dependency, crypto or portable release-default change.
Frozen baseline: merged PR #10 `d6c070a0f9e211f5a5b14b7e8c8bc0f995d7b7a7`.
Select `build_mode=pgo` or `cpu` in the native download workflow, with that
baseline, `tun_mode=vnet`, one client CPU and both archived prototypes disabled.
The PGO mode is also available for portable x64-musl LTS evaluation. Host-native
CPU mode is deliberately excluded from virtual-guest/cross-target comparisons.

The ordinary build path remains unchanged. Tuned builds first test the candidate
fixture and build the uninstrumented release baseline once, using it for peer,
server and load. The candidate has identical runtime source/features; its
different build flags and binary hashes are recorded in `builds.json` and checked
by the download analyzer. PGO and CPU tuning are independent; no combined mode.

PGO follows the [rustc workflow](https://doc.rust-lang.org/rustc/profile-guided-optimization.html):
instrument a target build, exercise only the client, merge fresh counters using
the compiler's matching LLVM tools, and rebuild with the resulting profile.
Six verified kernel TCP/TUN training downloads cover UDP/TCP/Stealth × IPv4 HTTP
with two flows / IPv6 HTTPS with four flows. Evaluation uses different one/eight
flows. Training is unpaced and isolated in disposable network namespaces; fixed
baseline auxiliaries are not instrumented. Logs/counters are training evidence,
never speed estimates. Profile generation/use flags otherwise match.

Each build uses a fresh owned profile directory. Compiler/target/flags, CPU,
kernel, training cases, raw/merged profile and instrumented/final binary hashes
are retained. The final timing binary must consume the recorded profile, omit
generation instrumentation and differ from the baseline binary. Profile warnings
remain in the build log. No profile is silently reused across versions/targets.

CPU mode changes only the declared compiler target to `native` on the actual
native runner. Static CPU model/feature identity is verified again at evaluation.
This binary is not portable and cannot be copied to Travel or another CPU family.
A device comparison must compile explicitly for that device and record its
requirements. See [rustc target options](https://doc.rust-lang.org/rustc/codegen-options/index.html#target-cpu).
Neither mode changes CPU allowances, NIC/router settings, crypto or worker bounds.

Optimized speed/resource comparisons remain separate from instrumentation.
Frame-pointer diagnostics in the LTS workflow are currently limited to ordinary
release mode; the workflow does not mislabel unoptimized diagnostics as PGO.
Native matrix/control/identity/headroom checks and guarded physical-device tests
must justify any retained recipe. There is no automatic promotion or release.
