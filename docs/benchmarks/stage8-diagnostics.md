# Stage 8: frozen-peer and delayed-TCP diagnosis

Baseline: merged PR #12 `64caaa24004db4f9cc901cc76d62d51bbc61d983`.
These observations diagnose the test system; sampled profiles are not normal
release speed measurements or evidence of a production speedup.

## Saturated auxiliary

Stage 7's unpaced baseline peer wire thread consumed approximately 0.998–0.999
core on both native architectures while the client used about 0.71–0.88 core.
The HTTP server and load roles individually had spare capacity. Consequently,
those results did not establish maximum client download capacity.

[Separate frozen-source profiles](https://github.com/riKirsti/ProtunNext/actions/runs/43320038702)
use source `bdcb077`, frame pointers and release debug information. Both native
Linux x64 and ARM64 jobs pass. System captures cover all four test CPUs; peer
CPU0 contributes approximately one quarter of total samples. Report percentages
below remain percentages of the complete capture, not normalized peer CPU time.

| Native platform | Transport | CPU0 total samples % | CPU0 kernel samples % | ring sealing samples % |
|---|---|---:|---:|---:|
| x64 | UDP | 25.14 | 17.22 | 5.94 |
| x64 | TCP | 24.81 | 16.42 | 6.07 |
| x64 | Stealth | 25.14 | 15.93 | 5.51 |
| ARM64 | UDP | 25.06 | 13.24 | 9.94 |
| ARM64 | TCP | 25.22 | 12.94 | 10.58 |
| ARM64 | Stealth | 25.68 | 11.44 | 8.97 |

Kernel totals include idle samples (approximately 0.61–0.80 percentage points
on x64, 1.23–1.55 on ARM64). System calls, copies, TUN and allocation work are
visible alongside encryption; Stealth also includes TLS AES-GCM work. These
samples justify screening an in-place test peer and a bounded TUN-read handoff.
They do not justify production parallel cryptography or changing ring.

The helper screen keeps the client byte-identical, gives it the same single
CPU and compares the legacy helper with each prototype separately. Reader mode
uses 64 reusable 65,000-byte buffers (4,160,000 payload-storage bytes), clears
initialized plaintext before reuse and clears entire buffers on destruction.
The reader runs on the former server CPU; server and load then share one CPU.
All thread CPU times and their shared-core sum are recorded. Actual per-core
busy time additionally catches asynchronous kernel work. Existing auxiliary
headroom screening remains in force; this is not a client resource ceiling.

## Delayed TCP

[Diagnostic run](https://github.com/riKirsti/ProtunNext/actions/runs/43320691239),
source `0e65d76`, runs twelve frozen-baseline TCP/IPv6/HTTPS/single-flow cases
per architecture: 128 MiB verified payload, aggregate 200 Mbps request, vnet,
one client CPU and 5 ms netem delay inside the owned disposable namespace.
Monitoring samples TCP state, queue statistics and protocol counters separately
in both inner namespaces. Final client-namespace counters are collected after
the fixture exits and before its namespace disappears. No host qdisc changes.

All **24/24** payloads verify. Observed diagnostic rates are approximately
**199.68–199.70 Mbps**. Every final client netem counter reports zero drops.
The severe Stage 7 delayed-TCP misses were **not reproduced**. This does not
identify their cause or establish that they are fixed: the old failures remain
in the Stage 7 report, and monitoring changes execution timing. Receive-window
and retransmission state in the retained traces merits inspection if the miss
recurs. These sampled diagnostic rates are excluded from acceptance comparisons.

Final client-namespace TCP MIB `RetransSegs` is zero in every case. Separately,
sampled sockets in the peer namespace do show retransmissions: the maximum
observed cumulative socket counter is 775 packets on x64 and 743 on ARM64.
Receive-window-limited episodes are also present (maximum observed cumulative
socket time 538 / 445 ms). These are maxima across snapshots and sockets, not
summed transfer totals. Zero client netem drops therefore does not establish
absence of transport loss, reordering or receive-window pressure. The retained
counter summary distinguishes each namespace and makes no causal claim.

## Scope

Both experiments use the existing production runtime, fingerprint
`5d448b4aabf1041e3a13920294314097f88a30eeb9a2d4c799724c2c04112fc7`,
with test and diagnostic overlays injected into isolated archives. Production
defaults, dependencies and security behavior are unchanged. Native loopback
results do not qualify physical NIC, WAN, thermal, account-enabled Local Agent
or other operating-system performance. Client comparisons and temporary Travel
tests must be reported separately before this stage is complete.
