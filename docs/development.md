# Development

ProtunNext is a Rust library and low-level CLI targeting the public functionality
of [Protun v3.0.5](https://github.com/ProtonVPN/protun/tree/a498c38ae6ffb0e68e7190b2831ef985d73fa6da),
commit a498c38ae6ffb0e68e7190b2831ef985d73fa6da. This pinned release is the
authoritative upstream baseline; the upstream default branch is not a substitute.
Implementation coverage and live/device acceptance are separate.
The machine-readable ledger in `src/requirements.json` powers `status`,
`verify-code-parity` for implementation coverage and `verify-parity` for live
deployment qualification. Update live acceptance statuses only when evidence
supports them.

## Compatibility contract

The [coverage index](../tests/fixtures/compatibility.json) maps every ledger
requirement to implementation and verification locations, including ProtunNext's
existing extensions. It is an inventory, not evidence that a test or live service
was exercised. The ledger retains the actual acceptance status.

[Frozen external-consumer contracts](../tests/contracts/api.rs) typecheck common
connection methods, callback traits, 37 record/enum shapes, Unix ownership entry
points and Windows constructors/settings. Exhaustive matches and explicit field
destructuring catch accidental changes even when internal code still compiles.
The downstream checker builds them outside this package with default features,
no default features, and the no-account unix bundle. These checks complement the
executable lifecycle, account, transport and FFI tests; compilation alone does
not prove wire compatibility or live account/device acceptance.

The shared compatibility validator also checks the contract's canonical UTF-8/LF
SHA-256 from the coverage index before any external-consumer build. Preserve this
reviewed fixture; a deliberate contract change requires explicit compatibility
review and a matching checksum update. Line-ending differences between hosts do
not change its identity.

## Upstream release updates

The **Upstream release monitor** runs daily and checks ProtonVPN five days
after the last successful check. A closed workflow-owned issue records that
date. Manual dispatch checks immediately and defaults to a dry run. Each new
stable upstream commit gets one tracking issue, even when the check is retried.

The monitor can prepare a draft on GitHub-hosted runners. Enable **Allow GitHub
Actions to create and approve pull requests** under repository Actions settings
and add an [OpenCode Console API key](https://opencode.ai/docs/console/api#api-keys)
as the repository Actions secret `OPENCODE_API_KEY`. The
workflow uses its repository token to open drafts. GitHub may require a
maintainer to approve running the draft's CI. With the secret unset, the
tracking issue still records the update.

The proposal job pins `opencode/deepseek-v4.1-flash-free` through OpenCode's
built-in provider and allows only that model. If the free model is unavailable
or time runs out, any usable edits become a draft patch. The next daily run
can resume from that branch, even between release checks. Closing the tracking
issue stops retries. The proposal and draft-writing jobs have separate
credentials. Review code, dependencies, upstream coverage and CI before
merging. The source publisher is a separate workflow.

Review the upstream-to-upstream diff and map each change against ProtunNext.
Record implemented changes, behavior already covered, platform-specific
differences and outstanding work in the update PR. Update the adopted tag,
commit and compatibility contracts together after reviewing the applicable
changes. Run the platform and erasure checks and use paired measurements for
packet-path changes. See the [release procedure](deployment.md#acceptance-and-updates)
after merging.

Retain existing module paths, generated binding contracts, feature combinations
and host ownership rules. Review any contract fixture change explicitly; do not
regenerate it merely to make an incompatible change pass. One refactor stage
belongs in each PR, with scope and acceptance checks reviewed before starting
the next stage.

## Architecture

The packet engine transfers owned reusable buffers through bounded queues.
WireGuard session handling uses the pinned ring-backed `proton-boringtun`
dependency with a narrow in-place data-crypto extension. Authentication and replay
commit precede delivery. Plaintext and tracked stale tails are cleared on recycle;
destruction clears complete allocations. Control messages, timers, cancellation
and shutdown remain responsive under backpressure.

Inside the engine, the live reactor owns readiness, budgets, timers and delivery
order; the offline worker keeps local services alive without an uplink. Packet
ownership helpers retain buffers through crypto and recycle queue rejections.
A private packet policy reports forward/consumed/filtered decisions without
owning queues. Authentication/replay commit and path confirmation precede inbound
policy; outbound capture remains before customer translation. Consumed service
or probe traffic is distinct from filtered traffic in counters.

The production receiver is serial. Complete TCP frames are extracted directly;
borrowed TLS input uses MTU-sized bounded prefetch. Linux/Android use UDP mmsg
batching with negotiated GSO/GRO where available, retaining datagram boundaries.
UDP batch size is 16 and never waits to fill. TLS uses unbuffered rustls on
Linux/Android and buffered rustls on Windows/Apple. Ordinary TLS verifies standard
certificates; Stealth retains its explicit outer policy and authenticates inner
WireGuard before delivery. Never turn Stealth into a fallback for ordinary TLS.

`Connection` manages packet-worker lifecycle and coalesced state/statistics.
The compatibility API supervisor handles peer updates, recovery, TUN replacement,
captures and account integration. The optional account thread uses published
Muon/RCRL/Local Agent SDKs. Private API/DNS/control sockets use a separate private
stack; normal customer traffic bypasses it. Requested paid settings are distinct
from applied settings and server refusals. The host supplies secure persistence.

Inside the supervisor, the private peer plan owns sanitized candidate order and
retry backoff; notifications own state coalescing, statistics requests and the
connection's timestamp origin; account coordination owns the service, pending
fork and server-observed state. Account notices yield explicit retry/rekey/fatal
actions to the supervisor, which retains worker and teardown ordering. These
components do not run on the packet thread or change the public API.

The private network module exposes one reactor contract. Its datagram adapter
owns UDP delivery/retry, the socket adapter owns nonblocking TCP readiness, and
the stream adapter connects framed TCP/TLS delivery to that contract. Unix socket
setup owns uplink binding/protection before connect; Windows retains native
socket setup beside its interface integration. TLS policy, record buffering,
Linux/Android batching and the packet engine remain separate owners.

Windows interface settings own the exact address/route rows and DNS changes;
they are dropped before the device session whose IDs they use. Android core
initialization and its public connection manager have separate owners; interface
configuration completes the service-owned builder without taking over native
connection lifecycle or descriptor transfer. Apple packet-flow ownership remains
inside its dedicated serial bridge.

Platform adapters own only resources explicitly assigned to them. Unix attaches
to a caller-owned descriptor/uplink; Android supplies VpnService protection and
network binding; Apple bridges public packet-flow APIs; Windows uses native
Wintun/IP Helper management. Cleanup must never enumerate/delete unrelated
interfaces or routes. See [deployment](deployment.md) for the host contract.

## Local checks

```sh
python scripts/check_repository.py
python -m unittest discover -s scripts -p 'test_*.py'
python -m unittest discover -s scripts/security -p 'test_*.py'
python scripts/verify-vendor.py
cargo fmt --package protun-next --check
cargo test --locked --all-targets --all-features
cargo clippy --locked --all-targets --all-features -- -D warnings
cargo clippy --locked --all-targets --no-default-features --features unix -- -D warnings
python scripts/check-downstream.py
cargo run --locked -- verify-code-parity
cargo run --locked -- verify-parity
```

The last command checks live deployment qualification and continues to fail while
requirements are open. Source release qualification uses the workflow documented
in the [deployment guide](deployment.md#acceptance-and-updates).
Default tests use synthetic data and do not configure host networking. Real TUN
tests are opt-in; CI runs them in disposable namespaces. Account test helpers are
opt-in and do not log in merely because a feature is enabled.

CI checks Linux hosts, actual supported LTS kernels, ARM64-musl, Windows native
bindings, Apple SDK slices and Android packaging. Release security CI checks
linked erasure code on x64/ARM64 and rejects an isolated wipe-free control. That
control is never deployed or used as a tunnel. The microbenchmark under
`benches/` measures a local packet path and does not establish VPN throughput.

## Repeatable measurements

Record CPU, memory and framing measurements from an already-built portable
benchmark:

    python scripts/measure-baseline.py --build --output target/baseline/framing.json

The build option uses the ordinary default-feature release library and links
the unchanged framing harness with matching portable optimization/panic settings.
Build time is excluded. This avoids mixing Cargo bench/test panic profiles with
the package's native library outputs. The runner discards a separate warmup
process, then retains seven individual
samples, dispersion, executable hash/size, compiler, lockfile hash and source
commit/dirty state. CPU includes all process threads (100% means one core);
memory is lifetime peak process RSS/working set, including setup. This measures
framing and buffer recycling without encryption, sockets or a TUN. Its Mbps are
not VPN download speed. Native host CI retains reports separately from platform
build checks. The ARM64 measurement is native Linux, not execution of the
musl/router artifact. Reports belong in ignored build output and CI artifacts.

For runtime-changing PRs, also compare verified end-to-end downloads for UDP,
TCP and Stealth, using matched toolchains/features, network conditions, repeated
A/A controls and balanced baseline/candidate order. Include CPU, peak memory,
download speed and test/build identities in the PR. Run temporary travel-router
checks against that PR's code and restore prior state. Keep private configuration
and device identifiers out of public reports. Establish workload-specific
regression limits from measured variability; framing noise is not a VPN limit.
Unavailable live/device evidence remains unverified.

## Changes and privacy

Keep APIs, wire formats, key/session/replay ownership, partial-write ownership,
bounded memory and descriptor ABIs intact. Add meaningful regressions when
changing behavior. See [contribution guidance](../CONTRIBUTING.md).

Do not commit local machine paths, personal emails, real device/account data,
private configurations, captures or build/profiling output. Use documentation
address ranges and clearly marked synthetic fixtures. Keep legitimate project
identifiers and required third-party copyright/license notices intact.
Repository hygiene checks inspect project-owned text and local Markdown links;
they are not a complete secret scanner or a purge of Git/GitHub history.

The vendored snapshot is checksum-pinned by `UPSTREAM.json`; only its three
declared extension files differ from the registry archive. Keep upstream files,
licenses, notices and the patch together. Performance research has concluded;
new experiments require an explicit request rather than a maintenance cleanup.
