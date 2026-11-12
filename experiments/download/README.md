# Download experiments

These fixtures are injected into isolated archives by `experiments/audit/prepare.py`.
They are not production modules. Never install a diagnostic fixture as the client.
The normal fixture retains its legacy peer by default.

## Stage 8 reproduction

Use the existing `native-download-validation` workflow, enable `run_downloads`,
set baseline to merged PR #12
`64caaa24004db4f9cc901cc76d62d51bbc61d983`, and select:

- `helper-screen`: independently screen in-place and bounded-reader peers against
  legacy, with byte-identical portable client/helper executables, one client CPU,
  all three transports, IPv4 HTTP/single flow and 2 GiB payloads. Four A/A samples
  precede each ABBA comparison. Short smokes include IPv6 HTTPS/eight flows.
- `capacity-diag`: twelve frozen-baseline TCP/IPv6 HTTPS/single-flow cases per
  architecture, 128 MiB at requested 200 Mbps with 5 ms delay. TCP/qdisc/counter
  sampling is diagnostic and must not be used for normal-release speed ranking.

Both modes run on native Linux x64 and ARM64. Owned disposable namespaces contain
all interfaces and qdiscs; no host or persistent network setting is modified.
Artifacts retain binaries, source/build identities, logs, plans and analyses.
Reproduce the original evidence using its linked immutable source commit, not
an unspecified future branch tip.

The reader reserves exactly 64 reusable 65,000-byte buffers, preserves FIFO
ownership and clears plaintext before reuse/destruction. It moves TUN reads to
CPU3, shares CPU1 between load/server, leaves wire on CPU0 and client on CPU2.
Metrics include wire/reader/server/load times, shared-core sums and actual host
CPU ticks. More helper CPU is not an improvement to the production client.

Stage 8 rejects these modes as default replacements: in-place processing does
not create headroom; reader mode raises x64 fixture throughput but regresses
ARM64, while the peer wire thread still saturates. Do not compare tuned clients
with different helper configurations or use these results to rank maximum client
capacity. `NEXT_PEER_HELPER_SCREEN` and `NEXT_PEER_HELPER_FIXED` are mutually
exclusive, explicit diagnostic controls, not supported production settings.

See [the measured decision and resource tables](../../docs/benchmarks/stage8-capacity.md)
and [the sampled diagnosis](../../docs/benchmarks/stage8-diagnostics.md). Other OS,
account-enabled Local Agent, NIC, thermal and WAN qualifications are separate.
