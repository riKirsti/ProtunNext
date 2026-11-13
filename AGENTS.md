# Repository guidance

- Read `docs/development.md`, `docs/building.md` and `docs/deployment.md`.
- Preserve public APIs, platform contracts, account features and release gates.
  Cleanup must not remove supported functionality or fabricate release readiness.
- Preserve bounded ownership, authentication/replay checks, erasure, descriptor
  contracts and platform-specific I/O behavior.
- Run `python scripts/verify-vendor.py`. Keep the complete pinned vendor snapshot,
  licenses and attribution; do not casually reformat or prune upstream files.
- Keep personal paths, device identifiers, private configurations, credentials
  and captures out of tracked files, issues and test artifacts. Public test
  credentials belong only in clearly marked synthetic fixtures.
- Run formatting, relevant tests, Clippy and repository checks. Preserve linked
  erasure verification and its isolated insecure negative control.
- Performance research is closed. Do not add experiments or stage reports unless
  the user explicitly reopens it.
- Ordinary builds/tests must not change real device settings, install a VPN or
  publish a release. Deployment requires explicit host integration.
