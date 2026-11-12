# Stage 4 receive prototype

`stage4-pinned.patch` preserves the complete experimental production-source delta
from merged PR #8 to `543675241b948a6d3b8c656bb39feb7450b9470a`, including the
owned authentication API, bounded worker, explicit placement and regression tests.
SHA-256: `dd5f493c21ac34fe2dddff41d117affcd0a96968ffb748fbfaba59de8a3f0d38`.
The ordinary shipped source retains the PR #8 serial receiver; this patch is not
a runtime setting, Cargo feature or dependency of production builds.

To repeat the experiment in the existing download workflows, select baseline
`a44d520c337611f59d4c7186971919d57de36df8`, `tun_mode=vnet`, `client_cores=2`
and `parallel_receive=true`. The workflow sets the **builder-only**
`NEXT_RECEIVE_PROTOTYPE=1`. The archive builder validates and applies the patch
only to the isolated candidate archive, before computing runtime/effective source
fingerprints. The baseline and working checkout are untouched. The manifest also
records the patch digest and original prototype commit. Default workflows leave
the prototype unapplied.

The analyzer requires actual helper work, distinct recorded owner/helper CPUs
matching the two allowed client CPUs, identical TUN contracts, checksum-valid
downloads and the existing A/A/order/quality controls. Shared peer/server CPU
usage is screened together. Instrumented profiles remain separate from normal
release timing. A local reconstruction check reproduces both tested normalized
production-source fingerprints exactly.

See [the stage evaluation](../../docs/benchmarks/stage4-receive.md) for measured
costs, failures, rejected placements, Travel validation and remaining gaps. This
rejects the tested batch/barrier implementation, not all multicore architectures.
