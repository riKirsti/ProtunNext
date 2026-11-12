# Stage 5 crypto preselection

This independent crate does not change production dependencies or runtime code.
It compares pinned ring 0.17.14, the existing RustCrypto ChaCha20Poly1305 0.10.1
dependency, and AWS-LC 1.19.0. All implement full IETF ChaCha20-Poly1305; no custom
or reduced-round cipher is introduced. [AWS-LC's compatibility API](https://docs.rs/aws-lc-rs/1.19.0/aws_lc_rs/aead/index.html)
and [RustCrypto's in-place API](https://docs.rs/chacha20poly1305/0.10.1/chacha20poly1305/)
support the packet construction used here. The [current AWS-LC release](https://github.com/aws/aws-lc-rs/releases/tag/v1.19.0)
is pinned independently of production's lockfile.

Run `cargo test --locked --release --manifest-path experiments/crypto/Cargo.toml`
and `cargo build --locked --release --manifest-path experiments/crypto/Cargo.toml`.
Tests cover the [RFC 8439 §2.8.2 known answer](https://www.rfc-editor.org/rfc/rfc8439#section-2.8.2),
all six directional cross-backend pairs, empty/unaligned/MTU/large payloads,
short tags, wrong nonce/AAD, and altered ciphertext/tag. These tests do not replace
production session/replay/rekey/erasure tests for an integrated candidate.

On Linux, `python3 experiments/crypto/screen.py` runs six alternating forward /
reverse orders on one allowed CPU. Each cell lasts at least 250 ms, excluding
key setup/allocation. It times detached-tag sealing and authenticated opening
of 0/64/256/1280/1420/16384-byte payloads. Restoring the public synthetic input
buffer is included equally in every operation; outputs are consumed and checked.
Sealing uses distinct counter nonces within each measured key lifetime; opening
repeats a fixed public synthetic ciphertext, with no live session/replay state.
Compiled dispatch, crypto and buffer handling costs are included. Wall ns/op is
not process CPU/byte, RSS, calibrated energy, full-session cost or download Mbps.

The workflow retains raw observations, compiler/CPU information, complete
dependency tree, exact source/manifest/lock/binary hashes and commit. The same
binary contains all three implementations. This screening only selects plausible
candidates for independent production-path download experiments, not a backend
winner. No automatic CPU-feature forcing, PGO or production selection is made.
