# Stage 6: client PGO and platform builds

Frozen baseline: merged PR #10 `d6c070a0f9e211f5a5b14b7e8c8bc0f995d7b7a7`.
[Transition review](../performance-stages.md#stage-6-review-november-12) precedes
implementation. This stage is in progress; no optimization is selected yet.

The [independent build experiments](../../experiments/build/README.md) compare
client-only PGO and CPU-specific compilation without changing runtime source,
production dependencies, portable defaults, crypto, worker/queue bounds or TUN
mode. Fixed baseline peer/server/load binaries are uninstrumented. PGO training
uses different flow counts from evaluation and never supplies speed estimates.
Compiler/profile/CPU/binary identities distinguish a build experiment from A/A
even though runtime-source fingerprints match. Unsupported CPU portability and
missing or unconsumed profiles invalidate a comparison.

Retain absolute CPU %, CPU/byte, RSS MiB and verified download Mbps by measured
platform/transport, every control failure, source/build identity and limitations.
Build-only platforms remain explicitly performance-unmeasured. Latest selected
code/build requires temporary Travel validation for UDP/TCP/Stealth with owned
artifacts, narrow routing, watchdog and independent restoration. One Stage 6 PR;
no PGO/CPU combination, persistent NIC/router changes, release or next-stage code.
