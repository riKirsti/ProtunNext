# Stage 7: same-host build and real-workload acceptance

Frozen baseline: merged PR #11 `83582248f3a8e153c6ee75d2846ef7c7064b6323`.
[Stage review](../performance-stages.md#stage-7-review-november-12) precedes
implementation. Evaluation is in progress; no combined build or default change
is selected yet.

The [acceptance runner](../../experiments/build/README.md#stage-7-same-host-acceptance)
compares a fixed portable release baseline with independently trained PGO,
CPU-specific and fresh combined PGO+CPU builds on the same host, interleaved by
workload. CPU/RSS/Mbps, source/build/profile identities, A/A/timing/headroom and
failures must accompany each cell. Matched-load, unpaced capacity and disposable
network-delay scenarios remain distinct. Build-only platforms are unmeasured.

Guarded physical Travel validation compares portable release and fresh device
PGO across UDP/TCP/Stealth, retaining settings/Direct mode and independently
checking restoration. No Cortex-A53 combination, persistent router/NIC change,
new backend, production dependency or release belongs to this stage.
