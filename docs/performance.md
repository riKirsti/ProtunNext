Production packet path and verification
=======================================

This revision replaces the unreleased tuning matrix with one production policy:
compact bounded packets and in-place WireGuard data crypto. TLS selects buffered
rustls on Windows/Apple and unbuffered rustls on Linux/Android at compile time.
There is no runtime adapter switch or backend tuning matrix.
Linux/Android use recvmmsg/sendmmsg; Windows/Apple use their native Mio sockets.
UDP offload is negotiated inside the batch adapter and preserves datagram
boundaries. The maximum available batch is 16; the worker never waits to fill it.
Partial sends retain every unaccepted packet exactly once.

The [latest platform decision](tls-selection-20261105.md) combines actual x64
LTS worker tests, native ARM64 and macOS components, Windows repetitions and a
fresh Travel rerun. Unbuffered is supported for Linux x64 and Travel bulk traffic;
macOS components instead favor buffered, and Windows remains inconclusive.
Android/iOS device acceptance is still pending. The older November 3 evidence
remains historical rather than a claim that every platform benefits.

Linux keeps mmsg with outer UDP GSO/GRO. The expanded experimental io_uring
receiver had a reproduced completion-overflow stall; a temporary corrected
build passed the targeted burst regression and eight WAN downloads. That
receiver, its fix and any liburing package are not dependencies of this PR.
Complete-worker speed/RSS measurements do not justify adopting it as default.
Software TUN aggregation, production parallel crypto and mandatory PGO remain
excluded. See the linked reports for retained failures and workload limits.

`Options`, `UdpBackend`, `Transport::Tuned`, `Transport::UnbufferedTls`,
`update_performance_options`, `run_with_options`, the CLI `--performance` switch
and the `io-uring` / `in-place-crypto` build features were unreleased experiments
and are removed. Existing v3.0.5 transport/account/control functions remain;
ordinary `Transport::Tls` and `tls::client*` now use the selected adapter.
Callers supplying a custom TLS connection use `tls::ClientConnection`, a native
`rustls::ClientConnection` on Windows/Apple and `UnbufferedClientConnection`
elsewhere. `tls::client*` returns the matching type; downstream CI compiles this
contract from a separate application. `run_transport_fd` accepts the chosen transport with
the production policy. Build with ordinary default or existing product features.

Raw TUN is the descriptor contract on Linux/Android; vnet-header software
segmentation was an unmeasured new contract and is omitted. UDP GSO/GRO operates
on already encrypted independent WireGuard datagrams, preserving counters/tags;
receive aggregates are split before authentication. Oversized/truncated datagrams
are counted and never passed to WireGuard. The library changes no host sysctl,
route, DNS or package settings in this path. See [platform support](supported-platforms.md).

The Windows/Apple buffered adapter bounds rustls application output to 64 KiB,
retains accepted-byte ownership across partial writes and keeps independent read
progress under write backpressure. It uses the ordinary framed-read scratch
buffer. Standard TLS verification and Stealth use the same platform adapter;
their distinct trust policies are unchanged.

The unbuffered TLS adapter covers partial socket I/O, bounded output records,
fragmented input, accepted-byte ownership, close_notify/half-close, tickets and
TLS 1.3 key updates through rustls. It feeds record spans directly to the framing
decoder. The pinned rustls implementation still allocates received-record chunks
internally: `src/conn/unbuffered.rs::ReadTraffic::next_record`. This is not a
zero-copy or allocation-free TLS claim. Standard certificate/name verification
and Proton's explicit inner-WireGuard Stealth policy remain separate.
On Linux/Android, the borrowed-record framing path also omits the unused generic read scratch
allocation. Consumed scratch/plaintext regions are erased before reuse.
Readable TLS continues progressing when socket writes are blocked; protocol
records append behind already accepted ciphertext. Protocol output can grow
from 64 to at most 128 KiB under backpressure. A duplex/key-update regression
checks that this preserves record ordering and accepted application bytes.

Established writes process an empty rustls input span, leaving incoming records
for the read direction. ReadTraffic has priority over WriteTraffic in the pinned
rustls state machine. Consuming incoming application data during writes and then
refusing writes until it is read creates a cycle with the worker's bounded ACK
queue: a full output queue pauses reads while its TLS writer waits for a read.
A real loopback TLS/framing regression reproduces this at 63 queued ACKs before
the fix, and transfers 4096 1280-byte packets after it. A separate TLS1.2/1.3
regression checks writes preserve an already partially read plaintext record.

The unbuffered writer also retains already available encrypted application records
in its bounded output buffer instead of flushing between every record. Explicit
flush, reads, protocol transitions and insufficient capacity still drive output;
there is no timer or wait for future plaintext. Four maximum-sized records plus
framing fit in the normal output allocation. A real TLS1.2/TLS1.3 regression
checks exact plaintext and sparse flush delivery: 16 available records require
four socket writes rather than the previous sixteen. This is a syscall-count
result, not a router-throughput forecast. A single nonempty vectored slice can
also be borrowed directly, avoiding the gather copy and its erasure.

Instrumentation stays local to the packet worker and is published with existing
coalesced snapshots. Counters include syscall/message batching, fallback,
truncation, GSO/GRO, queue high-water marks, reserved free
pool storage at startup, authentication failures and outer I/O/EOF failures.
TCP/TLS counters are collected at the socket boundary, including ciphertext and
handshake I/O; they are distinct from delivered inner/application byte counts.
Pool capacity is not RSS or total process memory. No packet contents or
per-packet timestamp logging is added.

The optional `debug-counters` build feature adds packet-queue byte high-water
marks, pool recycle wipe bytes and successful data-message crypto copy bytes.
Without it these fields remain zero and their accounting is compiled out.
Copy accounting covers the original AEAD boundary on successful nonempty data
messages (receive includes the tag); it excludes failed opens, handshake queues,
TLS/framing copies and kernel copies. Wipe accounting covers pool recycling,
including tracked stale tails, rather than every destructor or dependency wipe.
Queue byte marks cover the engine's queues before flushing, excluding storage
already accepted into stream/TLS internals. These are scoped counters, not total
memory bandwidth. Collect them separately from an uninstrumented timing run.

The initial Travel startup/TLS EOF failures have not been attributed to a cause.
New deterministic regressions exercise the surrounding ownership/lifecycle paths;
passing them does not prove a fix for those unexplained live failures.

Cryptographic extension provenance
----------------------------------

`vendor/proton-boringtun` originates from the exact Proton registry 4.9.0 archive
whose SHA256 was checked against the pre-change Cargo.lock:
`0c10f0be52e40d51a36de1aa072bbd417b59042432c852fc3657a8772d678b34`.
`UPSTREAM.json` records hashes of the original distributed files. Only Cargo.toml,
noise/mod.rs and noise/session.rs are intentionally modified. LICENSE.md and
upstream copyright notices are preserved. No other dependency is vendored.
The dependency uses a direct path into this repository, so applications consuming
ProtunNext inherit the extension without needing their own Cargo patch table.
CI compiles a separate consumer to check this integration, in addition to the
root lockfile builds. [Cargo's patch scope](https://doc.rust-lang.org/cargo/reference/overriding-dependencies.html#the-patch-section).

The extension reuses the existing ring AEAD operations, counter allocation,
pre-authentication replay check and post-authentication replay commit. Handshake,
cookie, rate-limiting, timers, session selection, queued-packet and IP validation
semantics stay in the original implementation. There is no mutable/immutable
buffer aliasing. The upstream copy API remains available for reference interoperability and handshake/control processing. Root tests cover mixed
copy/in-place interoperability, bad tags, replay and large packets. These tests
do not constitute an independent cryptographic audit. `changes.patch` contains
the exact three-file extension diff against the checksum-verified archive.
The standalone vendored unit suite additionally checks byte-identical output
between copy/in-place APIs and retains the original timer/rekey regressions.

Run formatting with `cargo fmt --package protun-next --check`: preserving the
upstream vendor text avoids unrelated formatter changes in the provenance diff.

Primary references checked for this implementation
-------------------------------------------------

- [Linux sendmmsg contract](https://man7.org/linux/man-pages/man2/sendmmsg.2.html): partial results consume only the accepted prefix.
- [Linux UDP options](https://man7.org/linux/man-pages/man7/udp.7.html): UDP_SEGMENT and UDP_GRO semantics and PMTU constraints.
- [WireGuard socket batching](https://github.com/WireGuard/wireguard-go/blob/master/conn/bind_std.go) and [GSO metadata](https://github.com/WireGuard/wireguard-go/blob/master/conn/gso_linux.go).
- [Rustls buffered connection contract](https://docs.rs/rustls/0.23.46/rustls/struct.ConnectionCommon.html): bounded output, plaintext acceptance before handshake completion, and nonblocking read/write progress.
- [Rustls unbuffered API](https://docs.rs/rustls/latest/rustls/unbuffered/index.html), checked against cached 0.23.46 source.
- Proton registry-distributed proton-boringtun 4.9.0, specifically noise/session.rs, noise/mod.rs, noise/rate_limiter.rs and their tests; archive checksum above.

Acceptance remains separate from implementation. Host/CI checks establish
specific functional behavior, not universal VPN speeds or device acceptance.
The [TLS comparison](tls-selection-20261103.md) records the real router evidence,
including failures and workload-specific regressions. Device/Plus acceptance
remains governed by the existing parity release gate.
