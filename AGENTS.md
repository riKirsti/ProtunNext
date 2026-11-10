# Performance work agreement

- Follow the agreed roadmap in `docs/performance-stages.md`.
- One stage belongs in one PR. Keep independent experiments and their results
  within that stage PR; include only justified changes in the final proposal.
- Before starting each next stage, review the agreed plan with the user: its
  goal, baseline, current evidence, scope, validation and completion criteria.
  This review does not itself require asking for fresh permission to do work
  already authorized by the user.
- Stay focused on sustained download performance for UDP, TCP and Stealth.
  Testing infrastructure supports implementation; it is not a speedup.
- No invented CPU/RAM rejection thresholds or resource-exhaustion targets.
  Measure resource tradeoffs, keep ownership/queues bounded and preserve features.
- Keep historical results tied to their actual source identities. No speedup
  claim from build success, fewer syscalls or helper-limited throughput alone.
- Review router-testing authorization before any live device work; preserve
  configurations, packages, keys and Direct mode. This work creates no release.
- Starting with Stage 3, each stage PR must include absolute CPU usage (100% =
  one core), process memory MiB and verified download Mbps, baseline/candidate,
  by measured platform and WireGuard UDP/TCP/Stealth. Keep per-workload results,
  source identities, GitHub Actions evidence and measurement limits. Mark build-only
  platforms as not performance-measured instead of extrapolating Linux results.
- Each stage also includes a real temporary Travel router test of the latest PR
  production code for all three transports. Use isolated owned test artifacts,
  narrow test-only routing, a cleanup watchdog and before/after verification.
  Preserve saved settings, installed software, account/key and normal Direct path;
  remove only owned temporary artifacts. Report failures and cleanup explicitly.
