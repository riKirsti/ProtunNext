# Stage 4: physical Travel router test

Recorded November 11, 2026 New York / November 12 UTC. Temporary session started
03:47:42 UTC, lasting 179.61 seconds; independent read-only postflight at 03:56:14 UTC.

**Selection:** retain the serial receiver. All twelve balanced WAN comparison
downloads verify for WireGuard UDP, WireGuard TCP and Stealth. The prototype has
lower observed speed and higher CPU per byte in each transport. Of 24 planned
measured sessions, 23 verify; the first baseline UDP A/A session fails at warmup.
Keep that failure and the WAN limitations. Cleanup and all twelve independent
restoration checks pass.

## Device, sources and measurement

- Cudy TR3000 v1, two-core ARM64 Cortex-A53, Linux 6.12.112, 496,724 KiB physical RAM, MTU 1280, saved server US-NY#544.
- Frozen serial baseline: merged PR #8 `a44d520c337611f59d4c7186971919d57de36df8`. Rejected prototype: `543675241b948a6d3b8c656bb39feb7450b9470a`. The final PR production source matches this measured serial baseline exactly; subsequent changes preserve the prototype outside production and update tooling/reports.
- Identical temporary Rust driver, isolated source archives, normal ARM64-musl release, Rust 1.99.1, `--no-default-features --features unix`. Both variants negotiate vnet, so this compares parallel receive rather than re-testing Stage 3 coalescing. Actual production Connection and transport paths are used without replacing installed software.
- A laptop makes certificate-verified HTTPS requests through narrowly scoped Travel forwarding/NAT rules to one fixed CDN IPv4:443. Each transfer requires HTTP 206, exact Content-Range, 100,000,000 bytes and SHA-256 `2aa720bbba4fe59c980d62a1763575f43ebf214f28a5af2beadb970865d001b0` from [linux-6.12.112.tar.xz](https://cdn.kernel.org/pub/linux/kernel/v6.x/linux-6.12.112.tar.xz). Forward counters prove use of the owned tunnel.
- Four serial A/A controls followed by balanced serial/prototype/prototype/serial order per transport. Warmups are separate; failures remain. CPU is whole-process CPU time over the measured interval, including owned threads; 100% means one core. RSS is peak process resident memory including setup/warmup, excluding kernel/other processes. Mbps includes TCP/TLS setup, unlike the persistent-connection GitHub fixture. CPU ns/B supplements CPU % when speeds differ.

## Observed balanced comparisons

Geometric means of two successful serial and two successful prototype samples
per transport. These short WAN observations do not establish maximum capacity.

| Transport | CPU core % serial / prototype | Peak RSS MiB serial / prototype | Mbps serial / prototype | CPU ns/B serial / prototype |
|---|---:|---:|---:|---:|
| udp | 60.65 / 65.78 | 9.37 / 9.16 | 237.68 / 231.06 | 20.42 / 22.78 |
| tcp | 63.48 / 60.63 | 2.43 / 2.44 | 270.95 / 226.89 | 18.76 / 21.39 |
| stealth | 72.79 / 72.37 | 3.14 / 3.17 | 226.69 / 206.57 | 25.70 / 28.04 |

udp: speed -2.79%, CPU/byte +11.55%, RSS -2.24%; tcp: speed -16.26%, CPU/byte +14.04%, RSS +0.51%; stealth: speed -8.87%, CPU/byte +9.12%, RSS +0.93%.

udp: incomplete A/A (first session failed at warmup). tcp: A/A speed +7.01%, CPU/byte -3.99%, RSS -2.90%. stealth: A/A speed -39.30%, CPU/byte +24.55%, RSS -1.21%.

Same-size Direct checks are 448.39 / 449.49 Mbps
(+0.25%). Stable Direct endpoints do not remove VPN-server/WAN variability.
A/A compares outer slots 0/3 with inner slots 1/2, matching the balanced
comparison order. UDP lacks a complete A/A cohort. TCP speed drift (+7.01%)
and Stealth speed/CPU-cost drift (-39.30% / +24.55%) fail the existing 5%
screen. None of these WAN cells supports a clean causal speed ranking. Do not promote
these descriptive WAN ratios into universal speed claims.

All six prototype comparisons record real owner/helper authentication, distinct
CPUs 0/1, bounded batches up to 32, negotiated vnet and no offload fallback.
All twelve comparison sessions end TunnelEstablished with zero invalid packets,
outer EOF or outer I/O errors. The serial source is the latest selected PR
production code, so the final retained path is physically tested on all three
transports as well as the rejected experiment.

## Every planned measured session

`main` means retained serial; `erase` means rejected prototype (historical builder labels).

| Transport | Role/slot | Build | Valid | Mbps | CPU core % | RSS MiB | CPU ns/B |
|---|---|---|---|---:|---:|---:|---:|
| udp | aa/0 | main | no: warmup failed | n/a | n/a | n/a | n/a |
| udp | aa/1 | main | yes | 233.20 | 58.05 | 9.50 | 19.92 |
| udp | aa/2 | main | yes | 256.31 | 63.07 | 8.93 | 19.69 |
| udp | aa/3 | main | yes | 255.28 | 62.61 | 9.27 | 19.63 |
| udp | compare/0 | main | yes | 226.21 | 59.87 | 9.55 | 21.18 |
| udp | compare/1 | erase | yes | 225.94 | 64.00 | 9.39 | 22.67 |
| udp | compare/2 | erase | yes | 236.30 | 67.60 | 8.93 | 22.89 |
| udp | compare/3 | main | yes | 249.74 | 61.44 | 9.20 | 19.69 |
| tcp | aa/0 | main | yes | 272.48 | 66.90 | 2.55 | 19.65 |
| tcp | aa/1 | main | yes | 293.94 | 66.79 | 2.57 | 18.18 |
| tcp | aa/2 | main | yes | 256.88 | 62.40 | 2.39 | 19.45 |
| tcp | aa/3 | main | yes | 241.98 | 59.03 | 2.55 | 19.52 |
| tcp | compare/0 | main | yes | 255.18 | 61.69 | 2.39 | 19.35 |
| tcp | compare/1 | erase | yes | 201.08 | 53.40 | 2.54 | 21.25 |
| tcp | compare/2 | erase | yes | 256.02 | 68.84 | 2.36 | 21.52 |
| tcp | compare/3 | main | yes | 287.69 | 65.33 | 2.47 | 18.18 |
| stealth | aa/0 | main | yes | 227.10 | 72.77 | 3.21 | 25.64 |
| stealth | aa/1 | main | yes | 137.83 | 55.30 | 3.28 | 32.10 |
| stealth | aa/2 | main | yes | 137.31 | 56.48 | 3.23 | 32.91 |
| stealth | aa/3 | main | yes | 226.19 | 75.08 | 3.38 | 26.56 |
| stealth | compare/0 | main | yes | 228.26 | 73.44 | 3.10 | 25.75 |
| stealth | compare/1 | erase | yes | 242.35 | 81.03 | 3.05 | 26.76 |
| stealth | compare/2 | erase | yes | 176.08 | 64.64 | 3.30 | 29.38 |
| stealth | compare/3 | main | yes | 225.13 | 72.14 | 3.19 | 25.64 |

## Restoration and limits

A 600-second watchdog guarded uniquely owned temporary executables/config copy,
interface, policy route and marked forwarding/NAT rules. Cleanup removes these
and its guard. Saved settings, key/account, packages, normal Direct mode and
default routing remain unchanged. A fresh SSH session independently verifies
unchanged mode, boot/netifd identity, interfaces, rules, default route, packages,
socket limits and configuration hashes; owned directory/interface/processes and
guard are absent and the temporary route table is empty. All twelve checks pass.
Home was not accessed. No release or persistent installation was made.

This is a short single-device, single-server NoLocalAgent transport/data-path
test, not full account/Plus parity, sustained thermal/loaded-latency validation,
other devices/servers, Linux 6.18 ARM64 or a total-router energy measurement.
The controlled fixed-rate GitHub CPU regressions, not WAN ratios alone, justify
rejecting this prototype. Other multicore designs remain open for later review.

[Sanitized raw evidence](stage4-travel-evidence.md) retains every session,
source/archive/effective/binary hash, Direct check, failure, cleanup result,
independent postflight and the identical temporary driver. Router credentials,
configuration contents, saved hashes and local connection addresses are omitted.
