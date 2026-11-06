# Download performance research

The objective is sustained download throughput for WireGuard UDP, WireGuard TCP
and Stealth. Merged PR4 `f7bf03f5b4dc3981eaadedbf3a7841c5ed5ee5c1` is the
baseline. No fixed percentage CPU or 8/32 MiB memory threshold rejects a
candidate. Compare throughput against total CPU, bounded memory, loaded latency,
reliability and sustained thermal behavior on each target.

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
Each sample transfers 512 MiB total, plus 4 MiB warmup per flow. Each cell retains
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
serial peer can limit throughput: >=0.85 auxiliary core disqualifies capacity
ranking. A lower average does not prove the absence of transient bottlenecks.

Normal-release comparisons and profiling are separate. `profile.py` attaches
`perf` to the warmed client process, including worker threads, in a separate
frame-pointer/debug-info build. It retains perf data, reports and the actual
native host kernel. These profiles are diagnostics, not supported-LTS kernel
acceptance or normal-release speed measurements. Native ARM64 results never
substitute for ARM64 Linux 6.12/6.18 device evidence.

Run `python3 -m unittest discover -s experiments/download -p 'test_*.py' -v`.
Build with `BASELINE=<immutable commit> bash experiments/audit/build.sh <target>`.
Run `sudo python3 experiments/audit/lts.py 6.12.115 --download` or `6.18.60` after
building/restoring the matching existing test kernel. For native diagnostics:
`sudo python3 experiments/audit/native.py --download`. The manual
`download-performance.yml` workflow provides both, plus separate native profiles.

Runtime fingerprints exclude injected probes, so a harness-only PR is explicitly
reported as baseline characterization. Effective source/binary identities still
identify the entire executable and fixed baseline auxiliaries. Source archives,
compiler information, raw samples and invalidity reasons are retained.

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
