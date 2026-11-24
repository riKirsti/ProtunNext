# Repository guidance

- Read `docs/development.md`, `docs/building.md` and `docs/deployment.md`.
- Verify findings against repository code, primary sources or reproducible
  checks. Cite a file and symbol, an official source, or the command and
  observed result. Label inferences and unverified claims.
- Before choosing an approach and again when reviewing the diff, check the
  latest official documentation, upstream source and maintained examples for
  the relevant use case. Check the actual dependency version, upstream commit,
  feature flags and target in this repository before applying that guidance.
- Trace affected callers before editing. In the final review, match each
  changed behavior to its evidence and report which checks ran, passed,
  failed or remain unverified.

## Find the code and its checks

- Account and connection lifecycle starts in `src/api/` and `src/connection.rs`.
  For transport and buffering, trace `src/transport.rs`, `src/framing.rs`,
  `src/packet.rs` and `src/stream.rs`. Use tests and the architecture map in
  [the development guide](docs/development.md) to follow the affected path.
- Host adapters live in `src/linux.rs`, `src/windows/`, `android/` and `apple/`.
  Check the supported feature builds and downstream bindings when touching a
  host contract. The compatibility fixtures define the public surface.

## Preserve contracts and verify changes

- Preserve public APIs, platform contracts, account features and verification gates.
  Source releases use the publishing workflow. Report live deployment evidence separately.
- Preserve bounded ownership, authentication/replay checks, erasure, descriptor
  contracts and platform-specific I/O behavior.
- Run `python scripts/verify-vendor.py`. Keep the complete pinned vendor snapshot,
  licenses and attribution; do not casually reformat or prune upstream files.
- Keep personal paths, device identifiers, private configurations, credentials
  and captures out of tracked files, issues and test artifacts. Public test
  credentials belong only in clearly marked synthetic fixtures.
- Run formatting, relevant tests, Clippy and repository checks. Preserve linked
  erasure verification and its isolated insecure negative control.
- For a Rust edit, use `cargo fmt --package protun-next --check`,
  `cargo test --locked --all-targets --all-features`, and
  `cargo clippy --locked --all-targets --all-features -- -D warnings`.
- For public or platform APIs, also run `python scripts/check-downstream.py`
  and `cargo run --locked -- verify-code-parity`. Keep the platform and
  linked-erasure workflows as the merge gate.
- For docs and cleanup, run `python scripts/check_repository.py` and confirm
  that removed files have no build, packaging or test consumers.
- For performance, name the platform, workload, metric and hypothesis first. Keep raw
  paired measurements and runner decisions in Actions artifacts. Hosted agents
  propose changes. The runner executes and reports checks independently.
- Ordinary builds/tests must not change real device settings, install a VPN or
  publish a release. Deployment requires explicit host integration.
