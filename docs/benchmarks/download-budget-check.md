# Resource-budget recheck of retained download archives

Current ceilings: CPU time per verified downloaded byte <=1.25x baseline; additional peak userspace memory <=8 MiB/tunnel on Travel or <=32 MiB/tunnel on desktop/server. These ceilings do not allocate resources or target CPU utilization. Kernel socket buffers are recorded separately by the updated probe.

This is offline reanalysis, not a new performance campaign. Original archive files and speed/CPU/RSS measurements remain unchanged. Historical archives lack explicit target classification, scoped per-tunnel memory and kernel socket snapshots. Every retained cohort therefore remains **UNVERIFIED** for the complete resource budget; none is eligible as a proven optimization. Missing measurements are not zero or a pass.

CPU-only results below use the existing baseline/candidate comparison cells. Matching runtime sources make these characterization data, not evidence of a production improvement. Two incomplete cells cannot receive even a CPU-only pass.

| Artifact | Kernel / architecture | Planned samples | CPU cells within 1.25x | CPU cells unverified | Full budget |
|---|---|---:|---:|---:|---|
| [13466729158](https://github.com/riKirsti/ProtunNext/actions/runs/42140088347/artifacts/13466729158) | 6.12.115 / x86_64 | 96 | 12 | 0 | UNVERIFIED |
| [13468223459](https://github.com/riKirsti/ProtunNext/actions/runs/42140088347/artifacts/13468223459) | 6.18.60 / x86_64 | 96 | 11 | 1 | UNVERIFIED |
| [13467744076](https://github.com/riKirsti/ProtunNext/actions/runs/42140088347/artifacts/13467744076) | 7.0.0-1017-azure / x86_64 | 96 | 12 | 0 | UNVERIFIED |
| [13467129755](https://github.com/riKirsti/ProtunNext/actions/runs/42140088347/artifacts/13467129755) | 7.0.0-1017-azure / aarch64 | 96 | 11 | 1 | UNVERIFIED |
| [13468807946](https://github.com/riKirsti/ProtunNext/actions/runs/42142606408/artifacts/13468807946) | 6.18.60 / x86_64 | 96 | 12 | 0 | UNVERIFIED |
| [13469601239](https://github.com/riKirsti/ProtunNext/actions/runs/42143801058/artifacts/13469601239) | 6.18.60 / x86_64 | 8 | 1 | 0 | UNVERIFIED |
| [13469172610](https://github.com/riKirsti/ProtunNext/actions/runs/42143801058/artifacts/13469172610) | 7.0.0-1017-azure / aarch64 | 8 | 1 | 0 | UNVERIFIED |
| [13469706432](https://github.com/riKirsti/ProtunNext/actions/runs/42143801058/artifacts/13469706432) | 6.12.115 / x86_64 | 8 | 1 | 0 | UNVERIFIED |
| [13468958565](https://github.com/riKirsti/ProtunNext/actions/runs/42143801058/artifacts/13468958565) | 7.0.0-1017-azure / x86_64 | 8 | 1 | 0 | UNVERIFIED |

Validation includes exact 25%, 8 MiB and 32 MiB boundaries, values just above each ceiling, unknown target classification, total RSS versus scoped per-tunnel memory, missing/invalid kernel counters, budget approval versus speed evidence and legacy reanalysis. The new SO_MEMINFO collector has a real Linux socket regression. Synthetic analyzer fixtures are unit-test inputs, not measured VPN results.

[Checks-only CI passed](https://github.com/riKirsti/ProtunNext/actions/runs/42147662111) on code commit `1442914b57568409783596451450440c8aa16d73`: 20 download analyzer tests, 10 existing audit analyzer tests and 40 injected Rust library tests. The actual Linux socket accounting regression passed. Performance download jobs were skipped; no router was involved. Later changes only add this report and its documentation link.

New kernel snapshots cover only the client process INET sockets before/after transfer. They exclude transient peaks, TUN allocations, auxiliary sockets and other kernel memory. No kernel-memory cap is inferred from the userspace limits.
