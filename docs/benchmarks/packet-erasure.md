# Packet erasure: scope and evidence

PR #3 changes packet erasure from per-byte volatile stores to `fill(0)` followed
by `zeroize::optimization_barrier` (dependency floor 1.9.1). Drop still erases the
whole allocation; recycle still erases through the previous high-water mark.
Allocation sizes, queues, TLS, transport selection and platform support are
unchanged. This is a CPU-cost optimization, not a memory-footprint reduction.

The barrier's [official contract](https://github.com/RustCrypto/utils/blob/zeroize-v1.9.1/zeroize/src/barrier.rs)
observes the contents of its argument after preceding stores. The helper passes
the slice contents. Packet tests cover unaligned regions, adjacent canaries and
shortened packets with an earlier longer live region.

## Preserved research

All earlier prototypes, reports, raw evidence, failed attempts and corrected
harness history remain available at the annotated tag
[`pr3-research-20261105`](https://github.com/riKirsti/ProtunNext/tree/pr3-research-20261105)
and branch `research/pr3-performance-20261105`. Rejected kTLS, parallel crypto and
GRO-handoff experiments are removed from this PR's final file tree. The archive
is intentional: these are investigated candidates, not shipping backends.

The [corrected historical report](https://github.com/riKirsti/ProtunNext/blob/pr3-research-20261105/docs/benchmarks/test-audit-20261105.md)
separates native hosted-runner Linux 7.0 measurements, actual Linux 6.18 kTLS
measurements and Travel's Linux 6.12.112 measurements. The native numbers must
not be described as Linux 6.12/6.18 results. Historical measurements without
binary or kernel-image hashes do not acquire those hashes retrospectively.

## Focused supported-kernel comparison

`performance-audit-lts.yml` tests x64 **actual Linux 6.12.115 and 6.18.60**, each
on three independent hosted-runner allocations. These are local controlled
worker tests, not Proton WAN downloads. Each job has 72 samples: UDP, TCP and
Stealth; burst and sliding unlimited load plus 10,000 packets/second fixed load;
four same-candidate A/A controls and four ABBA/BAAB comparison samples per cell.
Packets are 1,200 bytes, the window is 32 and each sample has one warm-up second
and five measured seconds. One fixed, separate candidate peer and generator are
used for both clients. CPU affinities isolate logical CPU sets, not physical
cores or interrupts. KVM is mandatory; emulation is never substituted.

Baseline tag `bench-pr3-main-20261105` is checked against
`c4b2554398ffcbf35cdaa9c8797504d8df146bd9`. Candidate is the workflow's exact
checked-out commit. Rust is pinned to **1.99.1**, using the ordinary thin-LTO
release profile. Each run records source commits, archive and effective-source
SHA-256 values, two distinct executable hashes, guest-verified executable
hashes, kernel-image and builder-recipe hashes, compiler details, CPU pins,
raw console output, every planned sample and automatic analysis.

An execution pass is not evidence of a performance win. `analyze.py` validates
identity, planned sample completeness, measured arithmetic, negotiated TLS and
packet/I/O counters. A/A drift above 5% or missed fixed load makes that cell
**inconclusive**. A saturated peer/generator disqualifies a tunnel-capacity
ranking. Raw inconclusive cells remain visible; they are not retried until
passing. CPU/byte and CPU percentage describe client process cost; peak RSS is
lifetime client RSS. Neither includes asynchronous kernel work or auxiliary
process memory. Small RSS differences do not establish an allocation saving.

## Normal release code generation

`performance-audit.yml` builds the production packet API fixture on native x64
and ARM64 using normal release thin LTO, one codegen unit and aborting panics.
The fixture makes live bytes observable **before** erasure and never reads them
back afterward. The recycle fixture deliberately retains its small pool until
process exit so destruction cannot substitute for recycling. It is never
packaged with the runtime.

Artifacts contain the final linked binary hash, compiler/profile metadata,
functional packet tests and disassembly. The checker looks for a zero-valued
bulk memset reachable from both Drop and recycle fixtures, distinguishing it
from their initial runtime-byte fill. Unknown code generation fails for manual
inspection. This compiler-specific regression tripwire supplements the API
contract and functional tests; it is not a formal security proof for all future
compilers or platforms.

Reproduce with `python3 -m unittest discover -s experiments/audit -p 'test_*.py'`,
then run the two workflows at the same commit. The actual-kernel workflow needs
four logical CPUs, root-accessible KVM and the existing functional-test kernel
image cache (or builds that image from the unchanged kernel recipe).

## Current validation

The focused supported-kernel comparison and final linked-code inspection are
pending for this revision. No new throughput claim is made until their raw
artifacts have been reviewed. Both routers remain unchanged by this work.
