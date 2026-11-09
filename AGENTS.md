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
