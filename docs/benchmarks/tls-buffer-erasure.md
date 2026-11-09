# TLS buffer erasure

This follow-up to PR #3 reuses its existing internal slice-erasure helper at
eight `UnbufferedTlsIo` wipe sites. It adds no dependency or TLS state-machine
change. Consumed incoming/pending spans, the used transmit gather span, and all
four owned byte buffers at Drop retain exactly their previous wipe ranges and
timing. Rustls-owned keys and structured secret types retain their own erasure.

The [zeroize 1.9.1 API contract and example](https://github.com/RustCrypto/utils/blob/zeroize-v1.9.1/zeroize/src/barrier.rs)
support zero stores followed by `optimization_barrier` on the slice contents.
The helper never passes the Box descriptor in place of its allocation. This is
the same primitive already covered by the normal thin-LTO linked-code check and
its real barrier-free negative control on x64 and ARM64.

New tests exercise partial `Read` and borrowed pending consumption, preserving
unconsumed bytes and adjacent canaries, and gather erasure when a vectored write
returns WouldBlock. Existing TLS/framing tests cover actual encrypted traffic,
partial I/O, key updates and closure. Both platform TLS adapters still build;
production selects unbuffered TLS on Linux/Android and buffered TLS on
Windows/Apple. No Windows/Apple transport improvement is inferred from this
change.

## Measurement

The existing two-variant audit runner accepts `--baseline` (new default:
`f7bf03f5b4dc3981eaadedbf3a7841c5ed5ee5c1`, merged PR #4). Candidate defaults to
the checked-out commit. The
PR4 results themselves used explicit merged-PR3 baseline `ae9c9e8`. The legacy
internal candidate label is `erase`; report
columns identify baseline and candidate. The peer and generator are separately
identified baseline binaries, fixed for both sides. No PR #3 gain is credited
to this change.

Trigger `performance-audit-lts.yml` at the candidate commit. It runs the existing
72-sample plan on three allocations per actual x64 Linux 6.12.115/6.18.60 kernel,
plus three native ARM64 diagnostic allocations. The latter record the real host
kernel; they are not ARM64 Linux 6.12/6.18 acceptance. Each cohort retains A/A
controls, ABBA/BAAB, unlimited burst/sliding and fixed-rate load, payload checks,
CPU/byte, CPU percentage, peak RSS, auxiliary saturation and source/binary hashes.

These remain local UDP echo worker measurements through UDP/TCP/Stealth outer
transports. They do not measure inner TCP/HTTPS or Proton WAN downloads. Findings
and raw artifact links belong in the PR and separate research evidence, keeping
measurement archives outside the production diff. Framing-copy changes and new
backends require separate comparisons.
