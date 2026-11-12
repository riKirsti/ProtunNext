# Final performance validation

The [all-stage selection](../performance-policy.md) retains the production
implementation from merged PR #13, `7c582a28a528701ef3b3874f476903ab10f205a7`.
This closing PR changes guidance and records final validation. Production
runtime, dependencies, features and portable compiler defaults are unchanged.

The existing normal-release matrix validates UDP/TCP/Stealth × IPv4 HTTP/
IPv6 HTTPS × one/eight inner flows on native Linux x64 and ARM64 at requested
200 Mbps. Each cell retains four A/A controls and a balanced comparison with
exactly 128 MiB after warmup, fixed baseline peer/load binaries, negotiated vnet
and one allowed client CPU. This is a matched-load characterization of the
selected configuration; an unchanged runtime and imposed rate cannot establish
a new speedup or maximum capacity. Original Stage 7/8 failures remain historical.

CPU 100% means one core of process time; memory is lifetime peak process RSS MiB
including setup/warmup, excluding auxiliary/kernel memory. Verified application
Mbps includes body verification. Profiles/instrumented training supply no timing
values. The controlled fixture uses unix without Local Agent; full-feature
product builds retain Local Agent and are not performance-qualified by it.

Fresh tables, identities, controls, GitHub evidence and the separate temporary
Travel validation are added after the planned runs complete. Windows/Apple/
Android platform checks are functional/build/API evidence, not download results.
No physical NIC/thermal/account-enabled or universal-best claim is made.
