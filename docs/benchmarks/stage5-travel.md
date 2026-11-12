# Stage 5: physical Travel router comparison

Recorded November 12, 2026: 05:10:19 UTC, duration 189.83 seconds; independent
postflight at 05:14:08 UTC. Cudy TR3000 v1, two-core ARM64 Cortex-A53, Linux
6.12.112, 496,724 KiB physical RAM, MTU 1280, saved server US-NY#544.

All twelve balanced comparison downloads verify. Of 24 planned measured
sessions, 23 verify; the first baseline UDP A/A warmup fails and remains below.
Cleanup and all twelve independent restoration checks pass. The latest selected
production path is the measured ring baseline; the TLS prototype is archive-only
and has unresolved implicit-provider/Local Agent compatibility.

## Sources and method

- Frozen baseline: merged PR #9 `4c7cc53a0932fa31579729b603d80a9b9e9ecabf`. Candidate: Stage 5 source `905517c` with the explicit [TLS-suite patch](../../experiments/crypto/TLS_RECORDS.md). Runtime fingerprints match the GitHub comparisons exactly: baseline `5d448b4aabf1041e3a13920294314097f88a30eeb9a2d4c799724c2c04112fc7`, prototype `6d1b450fb9abe512604fff33c89583cae6afe82ea5162ce755b27712726f7341`.
- Identical temporary driver, isolated archives, normal release Rust 1.99.1 / aarch64-musl, `--no-default-features --features unix`. Both variants negotiate vnet and retain serial ring packet AEAD. Only TLS suite implementations change in the prototype. No installed program or persistent setting is replaced.
- Laptop-generated, certificate-verified HTTPS to one fixed CDN IPv4:443 through narrowly scoped Travel forwarding/NAT rules. Each measured transfer requires HTTP 206, exact Content-Range, exactly 100,000,000 bytes and SHA-256 `2aa720bbba4fe59c980d62a1763575f43ebf214f28a5af2beadb970865d001b0`, from [linux-6.12.112.tar.xz](https://cdn.kernel.org/pub/linux/kernel/v6.x/linux-6.12.112.tar.xz). Forward counters prove the tunnel path.
- Four baseline A/A sessions, then baseline/prototype/prototype/baseline order per transport; warmups are separate. CPU is whole-process CPU time over the measured interval, with 100% equal to one core. RSS is lifetime peak process resident memory including setup/warmup, excluding kernel/other processes. CPU ns/B accounts for unequal speeds. WAN Mbps includes TCP/TLS setup and differs from the persistent-connection GitHub fixture.

## Observed balanced comparisons

Geometric means of two successful samples per build/transport. These are short
WAN observations, not repeatable maximum-capacity estimates.

| Transport | CPU core % baseline / prototype | Peak RSS MiB baseline / prototype | Mbps baseline / prototype | CPU ns/B baseline / prototype |
|---|---:|---:|---:|---:|
| udp | 49.36 / 64.92 | 8.99 / 8.78 | 186.57 / 264.65 | 21.17 / 19.64 |
| tcp | 58.41 / 66.89 | 2.47 / 2.77 | 244.58 / 291.35 | 19.11 / 18.38 |
| stealth | 71.10 / 64.80 | 3.00 / 4.02 | 219.18 / 190.80 | 25.96 / 27.18 |

udp: speed +41.85%, CPU/byte -7.26%, RSS -2.34%.

tcp: speed +19.12%, CPU/byte -3.85%, RSS +11.84%.

stealth: speed -12.95%, CPU/byte +4.71%, RSS +33.74%.

A/A compares outer slots 0/3 with inner slots 1/2, matching the balanced order. udp: incomplete (first warmup failed). tcp: speed -4.68%, CPU/byte +1.83%; pass. stealth: speed +6.08%, CPU/byte -3.58%; fails 5% A/A screen. Direct checks: 475.11 / 491.58 Mbps.

UDP/TCP have no intended packet-path optimization in this experiment, yet show
large observed WAN speed differences. Those controls make it inappropriate to
attribute the ratios to the TLS backend. UDP A/A is incomplete; Stealth fails
its speed-drift screen. TCP's passing A/A screen alone does not establish a
TLS benefit on a transport that does not use TLS. Stealth has higher observed
CPU/byte and RSS. No physical speed improvement is selected from these samples.

All twelve comparison sessions use negotiated vnet without offload fallback,
end TunnelEstablished, and record zero invalid packets, outer EOF or outer I/O
errors at measurement. The normal-release static driver binary grows from
2,457,672 to 4,468,352 bytes in the prototype; this is file size, not RSS.

## Every planned measured session

`main` means baseline ring; `erase` is the historical builder label for the
archive-only TLS-suite prototype, not final production.

| Transport | Role/slot | Build | Valid | Mbps | CPU core % | RSS MiB | CPU ns/B |
|---|---|---|---|---:|---:|---:|---:|
| udp | aa/0 | main | no: warmup failed | n/a | n/a | n/a | n/a |
| udp | aa/1 | main | yes | 261.93 | 63.86 | 8.54 | 19.51 |
| udp | aa/2 | main | yes | 188.26 | 49.35 | 9.23 | 20.98 |
| udp | aa/3 | main | yes | 256.15 | 63.30 | 9.12 | 19.78 |
| udp | compare/0 | main | yes | 222.07 | 54.37 | 9.34 | 19.59 |
| udp | compare/1 | erase | yes | 262.44 | 64.75 | 8.78 | 19.75 |
| udp | compare/2 | erase | yes | 266.87 | 65.09 | 8.79 | 19.52 |
| udp | compare/3 | main | yes | 156.74 | 44.82 | 8.65 | 22.88 |
| tcp | aa/0 | main | yes | 259.89 | 63.11 | 2.49 | 19.44 |
| tcp | aa/1 | main | yes | 285.37 | 65.40 | 2.52 | 18.35 |
| tcp | aa/2 | main | yes | 241.70 | 60.85 | 2.33 | 20.15 |
| tcp | aa/3 | main | yes | 292.09 | 66.93 | 2.52 | 18.34 |
| tcp | compare/0 | main | yes | 291.24 | 66.82 | 2.52 | 18.36 |
| tcp | compare/1 | erase | yes | 291.03 | 66.87 | 2.72 | 18.39 |
| tcp | compare/2 | erase | yes | 291.66 | 66.91 | 2.82 | 18.36 |
| tcp | compare/3 | main | yes | 205.39 | 51.06 | 2.43 | 19.89 |
| stealth | aa/0 | main | yes | 201.25 | 69.49 | 3.20 | 27.64 |
| stealth | aa/1 | main | yes | 226.72 | 72.46 | 3.16 | 25.58 |
| stealth | aa/2 | main | yes | 219.66 | 73.07 | 3.25 | 26.62 |
| stealth | aa/3 | main | yes | 219.91 | 72.82 | 3.31 | 26.50 |
| stealth | compare/0 | main | yes | 220.65 | 71.45 | 2.98 | 25.92 |
| stealth | compare/1 | erase | yes | 215.29 | 70.96 | 3.89 | 26.38 |
| stealth | compare/2 | erase | yes | 169.10 | 59.17 | 4.15 | 28.01 |
| stealth | compare/3 | main | yes | 217.71 | 70.75 | 3.03 | 26.01 |

## Restoration and scope

A 600-second watchdog guards only uniquely owned temporary executables/config
copy, interface, marked forwarding/NAT rules and narrow policy route. Cleanup
removes these and its guard. A fresh SSH session independently verifies unchanged
mode, boot/netifd identity, interfaces, policy rules, default route, packages,
socket limits and saved configuration hashes; test directory/interface/processes
and guard are absent and the temporary route table is empty. All twelve checks
pass. Saved account/key and normal Direct mode remain unchanged. Home was not
accessed; no package installation, persistent configuration change or release.

This single-device, single-server NoLocalAgent test does not establish account/
Plus parity, long-term stability, thermal behavior, loaded latency, other devices,
Linux 6.18 ARM64, or total-machine energy. It cannot resolve the prototype's
separate default-provider compatibility issue. The final selected ring code is
the measured baseline, so the latest production path is physically exercised
for UDP, TCP and Stealth.

[Sanitized raw evidence](stage5-travel-evidence.md) retains every session,
failure, source/archive/runtime/effective/binary identity, Direct check, cleanup,
independent postflight and identical temporary driver. Credentials, saved hashes,
configuration contents, local addresses and disposable source paths are omitted.
