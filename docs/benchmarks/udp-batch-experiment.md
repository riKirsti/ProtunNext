# UDP syscall batch experiment

**Decision: do not promote 32-packet batching from this campaign.** All 192/192
downloads are valid, but all 24 cells are auxiliary-limited. Throughput changes
are small and inconsistent across workloads and architectures. Retain 16 as
the production batch limit. PR #6 restores that limit and retains the stronger
socket regression, reusable validation workflow and historical findings.
No production speedup is established.

PR #5's screenshot-era tree (`4c1abe8`) was the immutable experimental baseline.
The historical candidate increased the UDP syscall batch from 16 to 32, using the
existing maximum descriptor count. The packet-worker budget, queue limits,
wire format and platform selection remain unchanged. TCP and Stealth are
negative controls because that change only affected the Linux/Android UDP path.

The hypothesis is that fewer socket syscalls per downloaded byte can reduce
the kernel-boundary cost observed in the receive profiles. Larger bursts can
also increase latency or resource contention; throughput improvement is
unproven. The historical candidate must not be promoted based on
successful functional tests alone.

The completed experiment built isolated normal-release archives,
kept peer/load binaries fixed to the baseline, and ran all 12 download cells
on native x64 and ARM64 CI. Each cell retains A/A controls and ABBA/BAAB order,
verified payloads, CPU per byte, peak RSS, source/binary identities and failed
samples. Auxiliary saturation, short samples and noisy controls keep their
existing disqualification rules. Native CI results are not supported-LTS,
router, physical NIC, Android or Proton WAN acceptance.

The retained UDP regression crosses several full batches and includes a short tail
after several GSO groups, with IPv4/IPv6 and offload enabled/disabled.
Existing partial-send ownership tests remain.

The earlier [Travel screening](travel-decision-20261105.md) found no convincing
case for changing the batch limit in its UDP request/reply workload. This
experiment evaluated sustained inner TCP downloads instead. Native CI cannot
overrule that device-specific recommendation without a fresh device comparison.

Before promotion, repeat any clean gain on Linux 6.12/6.18 and actual target
devices; measure loaded latency, upload/duplex fairness, kernel buffer usage
and longer reliability runs. No release or device change is part of this
experiment. Historical results below retain their actual source identities.

## Reusable validation workflow

`udp-batch-experiment.yml` now runs analyzer and injected Rust regressions on
native x64 and ARM64 for relevant pull requests. Full normal-release download
campaigns are an explicit manual option (`run_downloads=true`), so test-only
changes do not automatically repeat the historical performance campaign.
The default immutable baseline is merged PR #5 (`f3e7502`); a manual campaign
can select another immutable commit. Both client variants use the same fixture,
and baseline peer/load binaries remain fixed. Changing the baseline does not
rewrite any historical results below.

## November 9 native ARM64 results

Candidate `bd56faebe4b090ac1dac4bf07ab634a666553075` versus baseline
`4c1abe891ba094e8209d9cfc382471489e710dfc`, Linux `7.0.0-1017-azure`.
[Raw ARM64 evidence](https://github.com/riKirsti/ProtunNext/actions/runs/42701875883/artifacts/13713785081).
All 96/96 planned downloads are valid, with zero invalid-data errors. All
12 cells pass the A/A and timing screens, but every cell is auxiliary-limited
and therefore ineligible for maximum-capacity ranking.

| UDP workload | Baseline / candidate Mbps | Throughput change | CPU ns/B change |
|---|---:|---:|---:|
| IPv4 HTTP, 1 flow | 2318.71 / 2415.48 | +4.17% | -3.95% |
| IPv4 HTTP, 8 flows | 2378.92 / 2424.72 | +1.93% | -1.90% |
| IPv6 HTTPS, 1 flow | 2590.78 / 2584.59 | -0.24% | +0.25% |
| IPv6 HTTPS, 8 flows | 2517.70 / 2553.45 | +1.42% | -1.40% |

Unchanged TCP/Stealth controls range from -1.03% to +1.27% throughput change.
The mean cumulative receive-call count across UDP comparison samples falls
from 8074 to 4504.875, and maximum observed receive batch grows from 16 to 32.
These counters include startup/warmup; they are not an isolated timed-interval
syscall profile. They confirm exercise of the candidate but do not establish
a repeatable capacity gain. Keep the Travel production recommendation at 16.

## November 9 native x64 results

The same immutable candidate and baseline on Linux `7.0.0-1017-azure`.
[Raw x64 evidence](https://github.com/riKirsti/ProtunNext/actions/runs/42701875883/artifacts/13712233676).
All 96/96 planned downloads are valid, with zero invalid-data errors. All
12 cells pass the A/A and timing screens, but every cell is auxiliary-limited
and ineligible for maximum-capacity ranking.

| UDP workload | Baseline / candidate Mbps | Throughput change | CPU ns/B change |
|---|---:|---:|---:|
| IPv4 HTTP, 1 flow | 2093.34 / 2070.74 | -1.08% | +0.78% |
| IPv4 HTTP, 8 flows | 1901.49 / 1850.52 | -2.68% | +2.09% |
| IPv6 HTTPS, 1 flow | 2077.85 / 2132.87 | +2.65% | -2.24% |
| IPv6 HTTPS, 8 flows | 1845.36 / 1794.04 | -2.78% | +2.06% |

Unchanged TCP/Stealth controls range from -3.21% to +2.34% throughput change.
The mean cumulative UDP comparison receive-call count is 105575.125 versus
105739.625; maximum receive batch is 16 versus 32. Increasing the configured
maximum therefore did not reduce this x64 campaign's receive-call count.
Peak userspace RSS is retained per workload in the raw report; it excludes
kernel socket buffers. No kernel-memory or fixed-rate latency claim is made.

## Validation and next experiment

[Candidate cross-platform checks](https://github.com/riKirsti/ProtunNext/actions/runs/42701875848)
and [normal-release download campaign](https://github.com/riKirsti/ProtunNext/actions/runs/42701875883)
identify the tested code commit `bd56faebe4b090ac1dac4bf07ab634a666553075`.
Both download jobs pass the injected Rust regressions and all 20 Python
analyzer tests. Those checks validate the historical 32-packet candidate;
PR #6's later restoration of 16 has separate checks and makes no new download
performance claim. Full sample plans, source/binary hashes, compiler
identity, CPU/byte, RSS, counters and controls remain in each raw artifact.

The next performance priority is a benchmark allocation with demonstrable
auxiliary headroom, followed by true receive-side TUN coalescing as a separate
candidate. Preserve raw-TUN caller compatibility and validate offload metadata,
checksums, segmentation and sparse/duplex traffic before promotion. The
historical outbound-segmentation screen is a different experiment. This batch
campaign neither validates nor rejects receive coalescing.
