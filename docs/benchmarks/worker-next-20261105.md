# Further worker performance experiments — 2026-11-05

Status: investigation branch; the repeated cohort is running. Do not merge based
on the pilot figures below. No release, router deployment, account access or
router package changes are part of this campaign.

## Changes and boundaries

Merged main is `c4b2554398ffcbf35cdaa9c8797504d8df146bd9`.
The shipping-source candidates are deliberately small:

* Transfer ownership of the final UDP GRO segment instead of copying that last
  segment. Earlier segments keep their existing independent buffers. This is
  not shared-buffer slicing or complete zero-copy GRO.
* Retain exactly the existing packet erasure extent and lifetime, but use a bulk
  zero fill followed by `zeroize::optimization_barrier`. The dependency floor is
  1.9.1, which provides that public API. Key erasure elsewhere is unchanged.

The latter follows the library's documented pattern of writes followed by the
[supported barrier](https://github.com/RustCrypto/utils/blob/zeroize-v1.9.1/zeroize/src/barrier.rs).
Tests cover unaligned ranges, short packets retaining a previously longer live
region, and GRO ownership/recycling. Both native release assemblies retain
`memset` calls in the candidate's recycler. This is an implementation check,
not a claim that a unit test alone proves dead-store resistance.

Parallel crypto and kTLS live under `experiments/`, outside the shipping build.
The former applies checked transformations to a disposable source archive;
production vendor files and APIs do not gain its experimental interfaces.

## Benchmark validity correction

The first comparisons used one Cargo target directory for several archived
checkouts with the same package identity. Cargo reused the baseline executable.
The binary hashes in those artifacts are identical. Runs `41969099610`,
`41970017615` and `41971733173` cannot establish any candidate speedup or loss.
Their baseline profiles still describe baseline work, and their A/A variation
can illustrate measurement noise. The first run additionally failed to upload
root-owned profiler files; its successful measurement steps do not repair that
missing artifact.

The superseded runs `41972470951`, `41972471050` and `41972532343` were canceled.
No candidate recommendation uses them. Retain the original files and failure
history instead of silently replacing the evidence.

Corrected builds use a separate Cargo target directory for each variant and for
each profiling variant. The harness hashes the actual source tree after all
transformations, embeds that identity in each executable, verifies it at runtime,
and refuses comparisons with equal executable hashes or mismatched identities.
Corrected pilot run:
[41972734503](https://github.com/riKirsti/ProtunNext/actions/runs/41972734503).

## Workload and interpretation

The real `Connection`/TUN/WireGuard worker runs in a disposable network namespace.
The peer and generator are separate processes, always invoked from the same
baseline binary for both sides of a comparison. Two CPUs are available to the
client, one to the peer and one to the generator. CPU affinity does not isolate
interrupts or asynchronous kernel processing. The peer busy-polls, so its CPU
usage alone cannot establish whether it limits throughput.

Cases are UDP, framed TCP and Stealth transports carrying synthetic IPv4 UDP
echoes: 64-byte application payload/window 1, and 1,200-byte payload/window 32
with either one or eight flows. Cold authenticated exchange is excluded from the
timing interval. Client process CPU excludes peer/generator CPU and includes
synchronous system time, but not all asynchronous kernel work. RSS is the client
process lifetime peak, not packet-pool reserved capacity.

These are not inner-TCP/HTTPS, NIC, mobile power, Proton WAN or real-account tests.
Native hosts currently run Linux `7.0.0-1017-azure`; their performance cannot be
reported as measured Linux 6.12/6.18 performance. Supported-LTS kTLS checks below
boot the named kernels separately.

The corrected pilot has two matched rounds on one allocation per architecture.
The expanded cohort uses all six orders of baseline/GRO-only/combined candidate
on three separately allocated runners per architecture. Repeated packet samples
or rounds on one runner do not become independent hosts. Keep protocol, packet
size, flow count and architecture separate when selecting a change.

## Pilot evidence — provisional

Compared with main, bulk UDP throughput changed +7.7%/+9.3% for x64 one/eight
flows and +6.4%/+5.1% for ARM64; corresponding client CPU/byte changed
-7.5%/-8.9% and -18.8%/-15.4%. Sparse and Stealth figures remain mixed. These
numbers motivate repetition; they do not establish a universal improvement.

Named flat CPU samples show the recycler shrinking from 9.5% to 1.4% in x64 UDP
and from 16.5% to 0.6% in ARM64 UDP. Percentages are compositional, profiler builds
are separate from timing builds, inlined work may be attributed to callers, and
symbols below the report threshold are omitted. Do not equate these percentages
with an exact absolute count of erasure cycles.

## Bounded parallel crypto

The first actual-worker experiment adds two persistent TX crypto workers with
at most 63 owned packets. Nonces are reserved serially before dispatch. Immutable
keys cross the boundary; tunnel state, replay commits, timers and statistics stay
with the owner. Completions verify key identity as well as the session index and
are delivered in input order. Handshake processing remains serial. Detached RX
jobs have regression coverage but are not enabled in the measured worker.

The design is informed by WireGuard's
[ordered/parallel queue discussion](https://www.wireguard.com/papers/wireguard-netdev22.pdf),
not by assuming a primitive benchmark predicts VPN throughput.
Local tests exercise sparse/full batches, ordinary peer interoperability,
duplicate and tampered data, expiry and shutdown with work in flight.

Corrected run
[41972734509](https://github.com/riKirsti/ProtunNext/actions/runs/41972734509)
has six pairs per cell on one allocation per architecture. The per-packet
dispatch/batch-barrier implementation loses substantially: bulk UDP throughput
falls about 30% on x64 and 16% on ARM64, while CPU/byte rises about 65% and 50–53%.
It must not become the production default. A refinement dispatches reusable
whole batches and bypasses worker handoffs for batches smaller than eight;
results are pending. Neither result can decide the value of every possible
asynchronous or flow-based parallel design.

## Kernel TLS

The standalone bounded functional probe uses the pinned Rustls kernel-connection
API and a fixed public test certificate. Linux 6.18.60 passed application-data,
RX/TX key-update, session-ticket and clean-close checks. Its kernel counters
reported one RX and two TX rekeys with no rekey errors. Linux 6.12.115 accepted
the initial keys but rejected replacement with `EBUSY`; that is a demonstrated
limitation, not successful long-lived kTLS acceptance. Native x64/ARM64 passed
the same bounded functional fixture. All negotiated AES-256-GCM with TLS 1.3.
TLS 1.2 remains outside the prototype; a `userspace_required` declaration is not
a TLS 1.2 kTLS test pass.

Evidence:
[41971072655](https://github.com/riKirsti/ProtunNext/actions/runs/41971072655),
[Linux 6.18 kTLS contract](https://docs.kernel.org/6.18/networking/tls.html),
[Linux getsockopt/key setup source](https://github.com/torvalds/linux/blob/v6.18/net/tls/tls_main.c),
[pinned Rustls kernel API](https://github.com/rustls/rustls/blob/v/0.23.46/rustls/src/conn/kernel.rs).
Rustls delegates record-limit handling and much of the post-handshake behavior
to the caller after extraction. The current adapter is blocking and bounded;
it does not implement a production asynchronous handoff, reconnect fallback or
long-lived cipher-limit policy. It cannot be selected by the production worker.

A separate component screen borrows decrypted records in both userspace and
kTLS, uses a fixed separate Rustls peer, excludes the warm-up handshake, and caps
each connection's time, messages and bytes. It tests potential outer-TLS savings,
not complete Stealth performance. Both its native and actual-6.18 results must be
kept separate from the worker table. Integration still requires retained trust
and SNI policies, partial I/O, fragmented controls, KeyUpdate, closure, failure
recovery and bounded resource tests before any default can change.

## Remaining selection gates

Do not add flow queues, multiqueue TUN, io_uring or PGO solely because a technology
is available. Re-profile the winning small change, then test the next measured
bottleneck. Tailscale's [article](https://tailscale.com/blog/making-tailscale-faster)
suggests inspecting buffer ownership, queue depth and flow concurrency; it does
not establish that ProtunNext should duplicate their design or reported gains.

Before promotion, retain all failures, check the full existing cross-platform
suite, and verify supported-kernel behavior. Mobile and Apple/Windows device
performance remain separate acceptance work. Router tests require their own
authorized bounded window; no router work was performed here.
