# Download performance research

The objective is sustained download throughput for WireGuard UDP, WireGuard TCP
and Stealth. Merged PR4 `f7bf03f5b4dc3981eaadedbf3a7841c5ed5ee5c1` is the
baseline. Experimental evaluation ceilings are now mandatory:

- Candidate CPU time per verified downloaded byte must be <=1.25 times baseline.
  This is a relative CPU ns/B limit, not a CPU-utilization target or allocation.
- Additional observed peak userspace memory must be <=8 MiB per tunnel on Travel,
  or <=32 MiB per tunnel on desktop/server targets. These are increments over
  baseline, not total-memory limits or automatic allocations.
- Measure kernel socket buffers separately. Do not charge them to userspace RSS,
  report missing measurements as zero, or infer an additional kernel-memory cap.

`campaign.py` applies the CPU limit to the geometric means of matched comparison
samples. Its conservative observed userspace increment is the largest candidate
per-tunnel peak minus the smallest baseline per-tunnel peak. The fixture has one
tunnel in the client process; process VmHWM includes setup and fixture overhead,
while peer, HTTP server and load generation are separate processes. This is a
resident-memory measure, not allocated heap bytes or an attribution of individual
allocations. The recorded tunnel count, measurement method and total process HWM
must agree. Multiple-tunnel division is not inferred.

Set `NEXT_DOWNLOAD_TARGET_CLASS=travel` or `desktop_server` explicitly; unknown
targets do not inherit the larger limit. The CI workflow records desktop/server.
Missing or invalid per-tunnel memory, target or kernel measurement means
**unverified**, while a measured budget breach means **fail**. Capacity screens
and budget compliance are separate. Equal runtime sources cannot establish an
optimization; passing resource ceilings does not establish a throughput gain.

The Linux fixture records read-only `SO_MEMINFO` snapshots of its own INET
sockets after warmup and after the measured transfer, outside the timed CPU
interval. Allocation counters, configured buffer limits and packet drops remain
separate, with no invented sum. These boundary snapshots exclude transient peaks,
TUN allocations, auxiliary sockets and other kernel memory. They are not a claim
to measure total kernel memory or its high-water mark. See Linux's
[socket memory UAPI](https://github.com/torvalds/linux/blob/v6.12/include/uapi/linux/sock_diag.h)
and [socket implementation](https://github.com/torvalds/linux/blob/v6.12/net/core/sock.c).

Historical archives lack the new measurement fields: their original numbers
remain unchanged, but they do not pass this resource-budget evaluation.
The [offline budget recheck](benchmarks/download-budget-check.md) retains each
cohort separately and records the missing evidence explicitly.

The [initial download baseline and profiles](benchmarks/download-baseline.md)
retain the original failures, corrected cohorts, per-target speed/CPU/RSS and
capacity limitations. This measurement-only change claims no production gain.

## First implementation: measurement

`experiments/download/probe.rs` is injected identically into isolated archives by
the existing audit builder. It does not enter the shipped crate. It runs actual
kernel TCP endpoints through the production `Connection` and raw TUN worker.
HTTP uses two requests over a persistent connection: verified warmup first,
then the measured request. HTTPS additionally uses certificate-verified Rustls
with the public test CA. Every 64 KiB body block validates flow, sequence and all
payload bytes. The measured response cannot start before the measurement gate.

The peer creates its outer sockets before unsharing a second network namespace.
Those sockets retain the original network stack while the peer TUN and inner
TCP listener use the second stack. This permits real TCP without a veth driver,
IP forwarding, host routes, persistent namespaces or sysctl changes. Namespace
identities, tunnel counters and successful cryptographic handshakes are checked.
Always start the client using `unshare -n`; it refuses the initial namespace.
See [Linux network namespaces](https://man7.org/linux/man-pages/man7/network_namespaces.7.html)
and [TCP semantics](https://man7.org/linux/man-pages/man7/tcp.7.html).

Each outer protocol has IPv4 HTTP and IPv6 HTTPS, each with 1/8 inner flows.
Each sample transfers 2 GiB total, plus 4 MiB warmup per flow. Each cell retains
four same-binary controls followed by ABBA/BAAB, for 96 samples per allocation.
This is an initial baseline campaign, not the complete IPv4/IPv6 x HTTP/HTTPS
acceptance matrix. Minimum measured duration is 2 seconds; interval skew above
2%, A/A throughput or CPU/byte differences above 5%, or a saturated auxiliary
preclude a capacity conclusion. These are engineering screens, not statistical
significance. Failed samples and incomplete cells stay in the report.

CPU is process time, including synchronous kernel calls; 100% means one core.
Peak RSS includes lifetime setup but excludes kernel and auxiliary memory.
The initial fixture has loopback outer traffic, one allocation per target, no
injected RTT/loss, no physical NIC and no thermal/Proton-WAN acceptance. Its
serial peer can limit throughput: >=0.85 core in the peer wire thread, peer
server/control work or load process disqualifies capacity ranking. Wire worker,
load, client and inner HTTP server use separate logical CPUs 0/1/2/3. Process
and main-thread clocks separate peer costs. A lower average does not prove the
absence of transient bottlenecks. This is one current production packet worker,
not a many-core scaling test.

Early fixture-development runs remain invalid/inconclusive evidence. BusyBox
does not accept `nodad`, and IPv6 address activation needed a readiness check.
Initial 512 MiB samples often finished before the 2s screen. Background multicast
from the peer TUN also bypassed the missing peer AllowedIPs filter and could
initiate crossed handshakes, producing `UnexpectedPacket` before warmup finished.
The real-session regression includes the unfiltered negative control. Delayed
Stealth startup is exercised before the measurement matrix. No throughput loss
or crypto-authentication defect is attributed to production from these failures.
One subsequent Linux 6.18 sample exposed zero-byte plaintext acceptance in the
buffered Rustls test peer under backpressure. Its adapter now reports WouldBlock
for nonempty input accepting zero bytes, as the production buffered adapter
already does. A real TLS regression forces the zero-capacity condition, then
restores capacity and verifies the exact unconsumed payload. The affected cell
is retested separately; it is not silently substituted into the original cohort.

Normal-release comparisons and profiling are separate. `profile.py` attaches
`perf` to the warmed client process, including worker threads, in a separate
frame-pointer/debug-info build. It retains perf data, reports and the actual
native host kernel. These profiles are diagnostics, not supported-LTS kernel
acceptance or normal-release speed measurements. Native ARM64 results never
substitute for ARM64 Linux 6.12/6.18 device evidence.
An additional `--system` capture records all four test CPUs, retaining a separate
system-wide report and a PID-filtered client report. This exposes deferred kernel
work, but system totals include peer/load/background work and are not attributable
client energy costs. The workflow's `profiles_only` input avoids repeating the
normal-release download matrices when refining these diagnostics.

Run `python3 -m unittest discover -s experiments/download -p 'test_*.py' -v`.
Build with `BASELINE=<immutable commit> bash experiments/audit/build.sh <target>`.
Run `sudo python3 experiments/audit/lts.py 6.12.115 --download` or `6.18.60` after
building/restoring the matching existing test kernel. For native diagnostics:
`sudo python3 experiments/audit/native.py --download`. The manual
`download-performance.yml` workflow provides both, plus separate native profiles.
Its kernel selector and optional workload block permit explicit focused retests;
unknown blocks fail rather than producing an empty successful campaign.
The `checks_only` input validates analyzers and injected Linux probe regressions
without starting any download performance campaign.

Runtime fingerprints exclude injected probes, so a harness-only PR is explicitly
reported as baseline characterization. Effective source/binary identities still
identify the entire executable and fixed baseline auxiliaries. Source archives,
compiler information, raw samples and invalidity reasons are retained.
The present diagnostic overlay also logs rejection classes and public control
headers identically in both archives; the effective source digest includes it.
No keys or application bodies are logged. Production source files stay unchanged.

## Next experiments and promotion gates

1. Profile copies, framing, allocation/erasure, crypto, TUN writes, socket work,
   scheduling and backpressure. Extend to upload/duplex, latency, real NICs,
   controlled RTT/loss and longer transfers before general capacity decisions.
2. Independently test complete-frame extraction, processing budgets, batch sizes
   and bounded pool/queue capacity. Preserve ACK/control/timer fairness.
3. Prototype genuine TUN receive coalescing/offload metadata, with correct
   checksums, IPv4/IPv6, segmentation and ordinary-packet handling.
4. Research asynchronous parallel RX, SIMD/multi-buffer crypto, shared packet
   storage and native WireGuard integration. Preserve replay acceptance, key
   epochs, ordered delivery, cancellation and full applicable feature parity.
5. Evaluate whole-pipeline io_uring, suitable zero-copy receive, software/hardware
   kTLS, CPU/NIC queue placement, NUMA and measured busy polling. Research AF_XDP
   and kernel-bypass approaches without assuming they integrate with TCP/TLS or
   the existing routing/control functions.
6. Evaluate client-only representative PGO with fixed uninstrumented auxiliaries
   and held-out workloads after promising paths stabilize.

An additional architectural hypothesis is kernel WireGuard plus a local UDP
bridge implementing Proton's TCP/Stealth framing. This could move inner crypto
and packet delivery into the kernel for all three transports, at the cost of
local socket work and integration complexity. It is unimplemented and unmeasured.
WireGuard's [namespace/socket ownership documentation](https://www.wireguard.com/netns/)
and [wstunnel's WireGuard use case](https://github.com/erebe/wstunnel) establish
underlying building blocks and a generic wrapping use case, not compatibility
with Proton or a ProtunNext speed gain. Preserve Proton wire format, capture,
private services, routing and lifecycle before considering promotion.

Official capability references (none constitutes a ProtunNext speed claim):

- [WireGuard integration](https://www.wireguard.com/embedding/).
- [Multi-buffer crypto](https://github.com/intel/intel-ipsec-mb).
- [Linux 6.18 zero-copy RX](https://github.com/torvalds/linux/blob/v6.18/Documentation/networking/iou-zcrx.rst): requires NIC header/data split and steering.
- [Linux TUN implementation](https://github.com/torvalds/linux/blob/v6.12/drivers/net/tun.c).
- [kTLS offload](https://docs.kernel.org/6.18/networking/tls-offload.html).
- [Network scaling](https://docs.kernel.org/6.18/networking/scaling.html) and [NAPI](https://docs.kernel.org/6.18/networking/napi.html).
- [AF_XDP](https://docs.kernel.org/6.18/networking/af_xdp.html).
- [Windows registered I/O](https://learn.microsoft.com/en-us/windows/win32/api/mswsock/ns-mswsock-rio_extension_function_table).
- [Rust PGO](https://doc.rust-lang.org/rustc/profile-guided-optimization.html).

Research is open-ended; this list is not a boundary. A prototype ships only
after a repeatable whole-download improvement justifies its resource/maintenance
cost and account/certificate, Plus, routing/DNS, failover, rekey and platform
regressions pass. Test Linux 6.12/6.18 x64/ARM64, Windows 25H2/26H2 x64, Android
16/17 16 KiB systems and Apple 26/27 ARM64 with explicit runtime evidence gaps.
Router testing preserves Direct mode, installed software, configuration and keys;
only owned temporary test artifacts are removed. This scope creates no release.
