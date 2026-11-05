# Focused packet-erasure results

432 valid samples; 54/54 A/A controls passed; zero invalid-data errors. Three independent runner allocations per actual kernel. 54/54 comparison blocks reduced measured client CPU/byte (individual changes: -12.17% to -2.20%).

Recommendation: merge the packet-erasure change. It preserves wipe coverage and lowers client CPU cost. Retain rejected backends as research; these measurements justify no new production backend.

Each row is the geometric mean across all three allocations; no noisy or limited allocation is silently filtered. CPU % is one process CPU second per wall second = 100%; CPU/byte is the efficiency measure. RSS is lifetime peak client memory, excluding peers and kernel memory. Allocations are unchanged: small RSS variations do not establish a memory gain or regression.

These are warmed local UDP echo worker tests through actual Connection/TUN/WireGuard and the selected outer transport, not Proton WAN downloads. They cover 1,200-byte packets, one flow and a 32-packet window. The fixed-rate 10,000 pps cells intentionally transfer 96 Mbps and measure cost at matched load.

32/36 unlimited comparison blocks qualify for a capacity ranking. 4 blocks had peer/generator saturation; their observed throughput is retained but is not a tunnel capacity result. Across all individual unlimited blocks, throughput changes ranged from -1.69% to +12.66%, rather than improving in every block. A/A absolute drift reached 4.81% in throughput and 4.88% in CPU/byte (predeclared limit: 5%).

Host CPUs differ (Xeon 8370C, EPYC 7763 and EPYC 9V74), so absolute figures must not be used as a causal comparison between kernel versions. The baseline and candidate executable hashes each reproduce identically across all six allocations. Compiler: Rust 1.99.1; normal thin-LTO release; baseline c4b2554, candidate 9b469d5.

| Actual x64 kernel | Transport | Load | A/A passed | Mbps main → erase | CPU ns/B main → erase (delta) | CPU % main → erase | Peak RSS MiB main → erase | Capacity-eligible blocks |
|---|---|---|---|---:|---:|---:|---:|---:|
| 6.12.115 | stealth | burst / unlimited pps | 3/3 | 624.60 → 635.21 | 8.62 → 8.22 (-4.67%) | 67.33 → 65.27 | 2.81 → 2.86 | 3/3 |
| 6.12.115 | stealth | burst / 10000 pps | 3/3 | 96.00 → 96.00 | 9.22 → 8.70 (-5.63%) | 11.06 → 10.44 | 2.87 → 2.85 | 0/3 |
| 6.12.115 | stealth | sliding / unlimited pps | 3/3 | 821.53 → 838.39 | 8.62 → 8.22 (-4.57%) | 88.49 → 86.18 | 2.81 → 2.81 | 3/3 |
| 6.12.115 | tcp | burst / unlimited pps | 3/3 | 851.40 → 863.78 | 7.23 → 6.89 (-4.63%) | 76.89 → 74.40 | 1.96 → 2.09 | 3/3 |
| 6.12.115 | tcp | burst / 10000 pps | 3/3 | 96.00 → 96.00 | 8.45 → 7.70 (-8.88%) | 10.13 → 9.23 | 2.03 → 2.01 | 0/3 |
| 6.12.115 | tcp | sliding / unlimited pps | 3/3 | 1133.78 → 1176.26 | 7.00 → 6.60 (-5.69%) | 99.13 → 97.00 | 2.01 → 2.03 | 3/3 |
| 6.12.115 | udp | burst / unlimited pps | 3/3 | 853.38 → 900.97 | 7.29 → 6.82 (-6.55%) | 77.81 → 76.77 | 2.94 → 2.98 | 3/3 |
| 6.12.115 | udp | burst / 10000 pps | 3/3 | 96.00 → 96.00 | 8.19 → 7.54 (-8.01%) | 9.83 → 9.04 | 2.97 → 2.99 | 0/3 |
| 6.12.115 | udp | sliding / unlimited pps | 3/3 | 1126.06 → 1215.19 | 7.04 → 6.42 (-8.84%) | 99.11 → 97.50 | 2.95 → 2.90 | 3/3 |
| 6.18.60 | stealth | burst / unlimited pps | 3/3 | 824.97 → 839.64 | 6.80 → 6.43 (-5.40%) | 70.07 → 67.46 | 2.85 → 2.88 | 3/3 |
| 6.18.60 | stealth | burst / 10000 pps | 3/3 | 96.00 → 96.00 | 7.29 → 6.79 (-6.88%) | 8.74 → 8.14 | 2.84 → 2.80 | 0/3 |
| 6.18.60 | stealth | sliding / unlimited pps | 3/3 | 1041.90 → 1074.26 | 6.94 → 6.65 (-4.16%) | 90.39 → 89.32 | 2.81 → 2.85 | 3/3 |
| 6.18.60 | tcp | burst / unlimited pps | 3/3 | 1266.86 → 1279.87 | 5.47 → 5.19 (-5.16%) | 86.61 → 82.99 | 1.99 → 1.99 | 3/3 |
| 6.18.60 | tcp | burst / 10000 pps | 3/3 | 96.00 → 96.00 | 6.31 → 5.84 (-7.43%) | 7.58 → 7.01 | 1.97 → 1.94 | 0/3 |
| 6.18.60 | tcp | sliding / unlimited pps | 3/3 | 1467.74 → 1567.63 | 5.43 → 5.09 (-6.26%) | 99.67 → 99.78 | 2.00 → 2.05 | 1/3 |
| 6.18.60 | udp | burst / unlimited pps | 3/3 | 1282.30 → 1324.87 | 5.27 → 4.92 (-6.69%) | 84.44 → 81.40 | 2.86 → 2.88 | 3/3 |
| 6.18.60 | udp | burst / 10000 pps | 3/3 | 96.00 → 96.00 | 6.14 → 5.67 (-7.57%) | 7.37 → 6.81 | 2.91 → 2.94 | 0/3 |
| 6.18.60 | udp | sliding / unlimited pps | 3/3 | 1483.44 → 1648.89 | 5.38 → 4.84 (-9.97%) | 99.75 → 99.82 | 2.93 → 2.89 | 1/3 |
