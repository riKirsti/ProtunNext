Performance paths and verification
==================================

The existing transports remain the default. These changes implement the November
1, 2026 proposal's stages 1–6. Kernel TLS is deliberately excluded. New paths are
independently selectable and require target benchmarks before promotion.

Use `Transport::with_performance(performance::Options { ... })` in the native Rust
API. The high-level/UniFFI connection exposes `update_performance_options`:
valid options queue a controlled worker reconnect while retaining account,
certificate, Local Agent, capture and private-network services. Invalid options
return false without changing state. A true return means queued, not connected.
Existing configuration records and constructors retain their fields/signatures.

The Linux CLI also accepts one additive `--performance=` argument with its run
commands. For example, `--performance=batch,offload,compact,in-place` or
`--performance=unbuffered,compact`. Positional arguments retain their previous
meaning. Build `--features io-uring,in-place-crypto` to include both optional
engines; compiling them does not change the default options.
`batch-size=8`, `batch-size=16` and `batch-size=32` select the syscall budget.

| Option | Default | Implementation / fallback |
|---|---|---|
| udp_backend | Portable | Batch uses Linux recvmmsg/sendmmsg; other platforms use portable sockets |
| udp_batch_size | 16 | Maximum 1–32 available messages; never waits for a full batch |
| udp_offload | false | UDP_SEGMENT sends, UDP_GRO receives; failures count and retain ordinary batch I/O |
| unbuffered_tls | false | Selects caller-managed rustls for Stealth; custom TLS uses Transport::UnbufferedTls |
| compact_pool | false | Small reusable buffers plus full-size read/oversized buffers; ownership remains bounded |
| in_place_crypto | false | Requires in-place-crypto build feature; narrow reviewed proton-boringtun extension |

`UdpBackend::IoUring` additionally requires the `io-uring` build feature. Linux
setup/register failures fall back to batching. A terminal unsupported multishot
completion also falls back. If UDP offload is requested with io_uring, the current
implementation selects batching so GRO metadata cannot be lost. There is no
claim that io_uring is faster; sends still use the batched socket implementation.

The io_uring path uses multishot receive, registered buffer rings and an eventfd
for completion readiness in the existing worker. Received buffers change owners
only after a completion returns them. An empty replacement buffer is published
with release ordering before reuse. SQPOLL and busy polling are absent. Setup
requires bounded synchronous cancellation support. Shutdown cancels outstanding
operations before freeing buffers; a kernel refusing cancellation quarantines
the bounded payload/ring mapping rather than risking use-after-free. Such a
failure is a reason not to promote the experimental backend.
After such a failure, new rings are disabled for the process to prevent repeated
quarantines; subsequent requested connections use the batched fallback.

TUN offloads are selected by the embedding host supplying an `IFF_VNET_HDR`
descriptor. The engine reads its metadata, software-segments TCPv4/TCPv6 and UDP
L4 GSO packets, and completes partial checksums. Raw TUN and Apple utun retain
their existing formats. The library does not call TUNSETIFF or TUNSETOFFLOAD and
does not change MTU, routes, DNS, packages or sysctls. The host controls offload
negotiation. Supported vnet header sizes are 10 and 12 bytes, using native byte
order. Legacy UFO, IPv6 routing/fragment GSO and oversized (>65535-byte) TUN
records are outside this new descriptor contract and fail explicitly. Ordinary
raw-TUN packet processing continues to support its existing inputs.

TUN segmentation is outbound. Inbound IP packets are injected individually with
zero vnet offload metadata; this implementation does not coalesce inner packets
for TUN injection. Outer UDP GRO is split into separate WireGuard datagrams before
authentication. Outer UDP GSO groups already encrypted independent datagrams;
every datagram retains its own counter and authentication tag. No peer protocol
change is introduced. GRO aggregates must fit the bounded receive allocation;
truncation is counted, and truncated data is never passed to WireGuard.

The unbuffered TLS adapter covers partial socket I/O, bounded output records,
fragmented input, accepted-byte ownership, close_notify/half-close, tickets and
TLS 1.3 key updates through rustls. It feeds record spans directly to the framing
decoder. The pinned rustls implementation still allocates received-record chunks
internally: `src/conn/unbuffered.rs::ReadTraffic::next_record`. This is not a
zero-copy or allocation-free TLS claim. Standard certificate/name verification
and Proton's explicit inner-WireGuard Stealth policy remain separate.
The borrowed-record framing path also omits the unused generic read scratch
allocation. Consumed scratch/plaintext regions are erased before reuse.
Readable TLS continues progressing when socket writes are blocked; protocol
records append behind already accepted ciphertext. Protocol output can grow
from 64 to at most 128 KiB under backpressure. A duplex/key-update regression
checks that this preserves record ordering and accepted application bytes.

Instrumentation stays local to the packet worker and is published with existing
coalesced snapshots. Counters include syscall/message batching, fallback,
truncation, GSO/GRO, io_uring operations, queue high-water marks, reserved free
pool storage at startup, authentication failures and outer I/O/EOF failures.
TCP/TLS counters are collected at the socket boundary, including ciphertext and
handshake I/O; they are distinct from delivered inner/application byte counts.
Pool capacity is not RSS or total process memory. io_uring buffers and vnet
scratch storage add memory beyond that pool statistic. No packet contents or
per-packet timestamp logging is added.

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
buffer aliasing. Disabled builds retain the copy API. Root tests cover mixed
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
- [WireGuard TUN offload implementation](https://github.com/WireGuard/wireguard-go/blob/master/tun/offload_linux.go) and [kernel segmentation documentation](https://docs.kernel.org/networking/segmentation-offloads.html).
- [Rustls unbuffered API](https://docs.rs/rustls/latest/rustls/unbuffered/index.html), checked against cached 0.23.46 source.
- [Maintainer UDP multishot example](https://github.com/axboe/liburing/blob/master/examples/io_uring-udp.c) and [io-uring 0.7.15 API](https://docs.rs/io-uring/0.7.15/io_uring/).
- Proton registry-distributed proton-boringtun 4.9.0, specifically noise/session.rs, noise/mod.rs, noise/rate_limiter.rs and their tests; archive checksum above.

Acceptance remains separate from implementation: compare portable/current and
each candidate on the target using matched direct/kernel controls, total CPU,
RSS, latency and all failed attempts. No new Travel throughput figure follows
from host/CI tests. Paid-account/Plus live validation remains a separate gate.
