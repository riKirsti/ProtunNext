# UDP syscall batch experiment

PR #5's screenshot-era tree (`4c1abe8`) is the immutable baseline. This
candidate increases the connected UDP syscall batch from 16 to 32, using the
existing maximum descriptor count. The packet-worker budget, queue limits,
wire format and platform selection remain unchanged. TCP and Stealth are
negative controls because this change only affects the Linux/Android UDP path.

The hypothesis is that fewer socket syscalls per downloaded byte can reduce
the kernel-boundary cost observed in the receive profiles. Larger bursts can
also increase latency or resource contention; throughput improvement is
unproven. This candidate is experimental and must not be promoted based on
successful functional tests alone.

The separate experiment workflow builds isolated normal-release archives,
keeps peer/load binaries fixed to the baseline, and runs all 12 download cells
on native x64 and ARM64 CI. Each cell retains A/A controls and ABBA/BAAB order,
verified payloads, CPU per byte, peak RSS, source/binary identities and failed
samples. Auxiliary saturation, short samples and noisy controls keep their
existing disqualification rules. Native CI results are not supported-LTS,
router, physical NIC, Android or Proton WAN acceptance.

The real UDP regression crosses two full batches, includes a short tail and
forces GSO splitting at the existing maximum payload bound, with IPv4/IPv6
and offload enabled/disabled. Existing partial-send ownership tests remain.

Before promotion, repeat any clean gain on Linux 6.12/6.18 and actual target
devices; measure loaded latency, upload/duplex fairness, kernel buffer usage
and longer reliability runs. No release or device change is part of this
experiment. Results will be recorded with their actual source identities.
