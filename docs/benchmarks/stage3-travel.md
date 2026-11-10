# Stage 3: physical Travel router test

Recorded November 9, 2026 New York / November 10 UTC. Temporary session 02:44:56–02:48:49 UTC; independent read-only postflight 02:50:07 UTC. Duration 232.70 seconds.

**Result:** all three transports complete real forwarded WAN downloads with the latest PR code and actually exercise coalescing. Of 24 planned measured sessions, 23 deliver the complete verified 100 MB body; one baseline UDP A/A session fails during warmup. All twelve balanced comparison sessions succeed. Cleanup and independent postflight pass. WAN variability prevents a repeatable maximum-speed claim. The observed client CPU/byte reduction comes with higher peak client RSS on this router, unlike the GitHub Actions memory results.

## Device, source and method

- Cudy TR3000 v1, two-core ARM64, Linux 6.12.112, 496,724 KiB physical RAM; MTU 1280. Saved Plus endpoint US-NY#544.
- Baseline merged PR #7: `f26fd3d741d125037797ec9953de1d50ebd93c8a`; candidate latest PR #8 head at testing: `6a754df933d7f77e5dd767c1a7f2380551f6f6f5`. Its production code matches the already verified `5dba6015353d20127c772ce80cd90063e6d5c6b0`; later changes in this supplement are reporting/agreement documents only.
- Identical temporary Rust driver in isolated source archives; normal release, Rust 1.99.1, ARM64-musl, `--no-default-features --features unix`. Baseline raw TUN, candidate newly negotiated receive-only vnet TUN. Actual production `Connection` and selected WireGuard UDP/TCP/Stealth paths; no installed runtime replacement.
- One laptop generates certificate-verified HTTPS through narrowly scoped Travel forwarding/NAT rules to one fixed CDN IPv4. Each measured request requires HTTP 206, exact Content-Range and exactly 100,000,000 bytes; all successful bodies share SHA-256 `2aa720bbba4fe59c980d62a1763575f43ebf214f28a5af2beadb970865d001b0`. Origin: [linux-6.12.112.tar.xz](https://cdn.kernel.org/pub/linux/kernel/v6.x/linux-6.12.112.tar.xz). Firewall byte counters verify forwarding through the owned tunnel path.
- Four baseline A/A controls, then balanced baseline/candidate/candidate/baseline order per transport. Warmup is separate; failures remain recorded. CPU uses process CPU time over the transfer interval; 100% means one core, not the whole two-core router. Memory is lifetime peak process RSS including warmup, excluding kernel/other processes. CPU/byte supplements CPU % because traffic rates differ. WAN Mbps times the complete HTTPS request, including TCP/TLS setup; it is not directly comparable to the persistent-connection GitHub fixture.

## Observed balanced comparisons

Geometric means of two successful baseline and two successful candidate measurements per transport. These are descriptive WAN observations, not controlled maximum-capacity estimates.

| Transport | CPU core % baseline / PR | Peak RSS MiB baseline / PR | Download Mbps baseline / PR | CPU ns/B baseline / PR | Quality |
|---|---:|---:|---:|---:|---|
| WireGuard UDP | 69.41 / 56.98 | 6.67 / 8.64 | 172.60 / 217.35 | 32.18 / 20.98 | incomplete A/A (one warmup failure) |
| WireGuard TCP | 75.38 / 55.58 | 2.45 / 2.51 | 201.42 / 228.52 | 29.95 / 19.48 | A/A speed -5.30%, CPU/byte +1.77% |
| Stealth | 66.60 / 71.28 | 3.18 / 3.28 | 136.44 / 217.89 | 39.06 / 26.18 | A/A speed +5.04%, CPU/byte -5.17% |

CPU/byte changes are UDP -34.80%, TCP -34.97% and Stealth -32.97%; peak RSS changes are UDP +29.64%, TCP +2.37% and Stealth +3.13% (rounded descriptive comparisons). Stealth uses more CPU per second while delivering more data, despite lower CPU per byte. No fixed memory ceiling is imposed; the measured tradeoff is retained.

UDP lacks a complete A/A cohort. TCP A/A speed changes -5.30%, exceeding the 5% comparison screen; Stealth A/A speed +5.04% and CPU/byte -5.17% also exceed it. Therefore none of these three WAN cells supports a clean causal speed ranking. Same-size Direct-path checks change from 486.93 to 382.75 Mbps, further demonstrating WAN variability. Do not advertise the observed VPN Mbps ratios as repeatable speedups.

All six candidate comparison sessions use vnet, coalesce real incoming packets, reach maximum group 32 and record zero offload fallback. Their accepted original-packet coalescing ratios span 99.25–99.67%, including warmup. All successful sessions finish TunnelEstablished with zero rejected packets, outer EOF or outer I/O errors at measurement.

## Every planned measured session

| Transport | Role/slot | Variant | Valid | Mbps | CPU core % | RSS MiB | CPU ns/B |
|---|---|---|---|---:|---:|---:|---:|
| udp | aa/0 | main | no: warmup failed | n/a | n/a | n/a | n/a |
| udp | aa/1 | main | yes | 127.02 | 51.74 | 7.26 | 32.60 |
| udp | aa/2 | main | yes | 195.10 | 75.42 | 6.79 | 30.93 |
| udp | aa/3 | main | yes | 193.99 | 75.12 | 6.73 | 30.98 |
| udp | compare/0 | main | yes | 181.31 | 72.63 | 6.57 | 32.06 |
| udp | compare/1 | erase | yes | 252.95 | 62.21 | 9.10 | 19.69 |
| udp | compare/2 | erase | yes | 186.76 | 52.19 | 8.21 | 22.36 |
| udp | compare/3 | main | yes | 164.30 | 66.33 | 6.76 | 32.30 |
| tcp | aa/0 | main | yes | 186.10 | 69.27 | 2.59 | 29.79 |
| tcp | aa/1 | main | yes | 199.62 | 75.21 | 2.58 | 30.16 |
| tcp | aa/2 | main | yes | 167.41 | 64.05 | 2.59 | 30.62 |
| tcp | aa/3 | main | yes | 200.24 | 74.88 | 2.54 | 29.93 |
| tcp | compare/0 | main | yes | 202.70 | 75.56 | 2.40 | 29.84 |
| tcp | compare/1 | erase | yes | 244.93 | 59.51 | 2.44 | 19.45 |
| tcp | compare/2 | erase | yes | 213.20 | 51.90 | 2.59 | 19.51 |
| tcp | compare/3 | main | yes | 200.15 | 75.20 | 2.51 | 30.07 |
| stealth | aa/0 | main | yes | 164.40 | 78.87 | 3.20 | 38.39 |
| stealth | aa/1 | main | yes | 146.07 | 69.09 | 2.99 | 37.85 |
| stealth | aa/2 | main | yes | 172.99 | 81.41 | 2.91 | 37.66 |
| stealth | aa/3 | main | yes | 139.29 | 71.85 | 3.21 | 41.28 |
| stealth | compare/0 | main | yes | 146.33 | 72.11 | 3.29 | 39.43 |
| stealth | compare/1 | erase | yes | 218.36 | 72.80 | 3.39 | 26.68 |
| stealth | compare/2 | erase | yes | 217.42 | 69.80 | 3.17 | 25.69 |
| stealth | compare/3 | main | yes | 127.22 | 61.51 | 3.07 | 38.70 |

## Restoration and scope

The 600-second cleanup watchdog was installed before tests. Only uniquely owned temporary executables/config copy, interface, policy rule, NAT table and marked forwarding rules were added. Cleanup removed them and the watchdog. No packages, saved settings, account/key, default route or normal Direct traffic were changed. A new SSH session independently verifies unchanged mode, boot/netifd identity, interfaces, policy rules, default route, package inventory, socket limits and saved-configuration hashes, plus absent test directory/interface/processes/guard and empty test route table. All twelve postflight checks pass. Home was not accessed.

This is one short, one-device, one-server NoLocalAgent transport/data-path test. It does not establish account/Plus control-plane parity, long-term stability, thermal behavior, loaded latency, other servers, ARM64 Linux 6.18, other devices or total-machine energy. Retain the explicit negotiated capability based on the controlled GitHub evidence and verified physical interoperability; keep existing raw caller compatibility and report the Travel memory increase.

Raw sanitized metrics, failures, source/binary fingerprints, independent postflight and the identical temporary Rust driver are retained in [Travel raw evidence](stage3-travel-evidence.md).
