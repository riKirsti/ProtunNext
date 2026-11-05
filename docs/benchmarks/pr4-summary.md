# PR4 TLS buffer-erasure results

648 valid timed samples; 79/81 A/A controls passed. Baseline: merged PR3 ae9c9e8. Candidate: 0dff661. Fixed, independently identified baseline peer/generator.

The only production change is reusing the existing bulk-erasure helper for TLS byte buffers. UDP/TCP are controls; no new UDP/TCP optimization is claimed. Reported figures are local UDP echo worker measurements, not inner TCP/HTTPS or Proton WAN download results.

Rows retain all three allocations, including any inconclusive or auxiliary-limited cells. Geometric means are shown. CPU % is client process core utilization; CPU/byte measures efficiency. RSS is lifetime client peak, not total system memory. Allocations remain unchanged. Native ARM64 records the actual host kernel and is not ARM64 supported-LTS acceptance. Host CPU models differ; absolute results are not causal kernel comparisons.

| Target / actual kernel | Transport | Load | Usable | Mbps baseline → candidate | CPU ns/B baseline → candidate (delta) | CPU % baseline → candidate | Peak RSS MiB baseline → candidate | Capacity eligible |
|---|---|---|---:|---:|---:|---:|---:|---:|
| aarch64 / 7.0.0-1017-azure | stealth | burst / unlimited pps | 3/3 | 829.81 → 872.41 | 7.09 → 6.47 (-8.75%) | 73.56 → 70.57 | 5.29 → 5.44 | 3/3 |
| aarch64 / 7.0.0-1017-azure | stealth | burst / 10000 pps | 3/3 | 96.00 → 96.00 | 7.53 → 6.77 (-10.10%) | 9.04 → 8.12 | 5.12 → 5.33 | 0/3 |
| aarch64 / 7.0.0-1017-azure | stealth | sliding / unlimited pps | 2/3 | 1156.38 → 1302.67 | 6.74 → 6.05 (-10.25%) | 97.37 → 98.44 | 5.26 → 5.12 | 2/3 |
| aarch64 / 7.0.0-1017-azure | tcp | burst / unlimited pps | 3/3 | 1087.56 → 1085.47 | 5.34 → 5.40 (+0.97%) | 72.65 → 73.22 | 4.75 → 4.53 | 3/3 |
| aarch64 / 7.0.0-1017-azure | tcp | burst / 10000 pps | 3/3 | 96.00 → 96.00 | 5.79 → 5.71 (-1.31%) | 6.95 → 6.86 | 4.59 → 4.84 | 0/3 |
| aarch64 / 7.0.0-1017-azure | tcp | sliding / unlimited pps | 3/3 | 1526.61 → 1532.71 | 5.13 → 5.13 (-0.06%) | 97.93 → 98.26 | 4.75 → 4.74 | 3/3 |
| aarch64 / 7.0.0-1017-azure | udp | burst / unlimited pps | 3/3 | 1103.93 → 1106.80 | 5.39 → 5.36 (-0.48%) | 74.35 → 74.19 | 5.50 → 5.46 | 3/3 |
| aarch64 / 7.0.0-1017-azure | udp | burst / 10000 pps | 3/3 | 96.00 → 96.00 | 5.70 → 5.68 (-0.36%) | 6.84 → 6.81 | 5.34 → 5.09 | 0/3 |
| aarch64 / 7.0.0-1017-azure | udp | sliding / unlimited pps | 3/3 | 1541.31 → 1514.54 | 5.11 → 5.14 (+0.62%) | 98.51 → 97.40 | 5.63 → 5.41 | 3/3 |
| x86_64 / 6.12.115 | stealth | burst / unlimited pps | 3/3 | 681.80 → 699.98 | 8.43 → 8.05 (-4.51%) | 71.86 → 70.45 | 2.90 → 2.81 | 3/3 |
| x86_64 / 6.12.115 | stealth | burst / 10000 pps | 3/3 | 96.00 → 96.00 | 7.96 → 7.80 (-1.92%) | 9.55 → 9.36 | 2.86 → 2.86 | 0/3 |
| x86_64 / 6.12.115 | stealth | sliding / unlimited pps | 3/3 | 889.11 → 929.19 | 8.02 → 7.64 (-4.76%) | 89.14 → 88.72 | 2.77 → 2.87 | 3/3 |
| x86_64 / 6.12.115 | tcp | burst / unlimited pps | 3/3 | 914.50 → 912.10 | 7.31 → 7.26 (-0.71%) | 83.58 → 82.77 | 1.99 → 2.04 | 3/3 |
| x86_64 / 6.12.115 | tcp | burst / 10000 pps | 3/3 | 96.00 → 96.00 | 6.86 → 6.78 (-1.21%) | 8.23 → 8.13 | 2.03 → 2.07 | 0/3 |
| x86_64 / 6.12.115 | tcp | sliding / unlimited pps | 3/3 | 1154.78 → 1162.23 | 6.86 → 6.82 (-0.63%) | 99.05 → 99.06 | 2.07 → 1.99 | 3/3 |
| x86_64 / 6.12.115 | udp | burst / unlimited pps | 3/3 | 946.32 → 931.37 | 7.25 → 7.30 (+0.68%) | 85.77 → 84.99 | 2.89 → 2.87 | 3/3 |
| x86_64 / 6.12.115 | udp | burst / 10000 pps | 3/3 | 96.00 → 96.00 | 7.09 → 7.26 (+2.40%) | 8.51 → 8.72 | 2.86 → 2.94 | 0/3 |
| x86_64 / 6.12.115 | udp | sliding / unlimited pps | 3/3 | 1203.63 → 1193.37 | 6.59 → 6.60 (+0.14%) | 99.14 → 98.43 | 2.91 → 2.92 | 3/3 |
| x86_64 / 6.18.60 | stealth | burst / unlimited pps | 3/3 | 1052.17 → 1092.90 | 5.05 → 4.71 (-6.80%) | 66.46 → 64.33 | 2.90 → 2.85 | 3/3 |
| x86_64 / 6.18.60 | stealth | burst / 10000 pps | 2/3 | 96.00 → 96.00 | 5.70 → 5.46 (-4.26%) | 6.84 → 6.55 | 2.85 → 2.82 | 0/3 |
| x86_64 / 6.18.60 | stealth | sliding / unlimited pps | 3/3 | 1384.46 → 1452.27 | 5.22 → 4.97 (-4.79%) | 90.35 → 90.23 | 2.86 → 2.83 | 2/3 |
| x86_64 / 6.18.60 | tcp | burst / unlimited pps | 3/3 | 1521.29 → 1516.69 | 4.14 → 4.16 (+0.63%) | 78.63 → 78.89 | 1.99 → 2.02 | 3/3 |
| x86_64 / 6.18.60 | tcp | burst / 10000 pps | 3/3 | 96.00 → 96.00 | 5.15 → 5.03 (-2.26%) | 6.18 → 6.04 | 1.97 → 2.09 | 0/3 |
| x86_64 / 6.18.60 | tcp | sliding / unlimited pps | 3/3 | 1848.04 → 1848.75 | 4.23 → 4.23 (-0.02%) | 97.74 → 97.75 | 1.99 → 2.06 | 3/3 |
| x86_64 / 6.18.60 | udp | burst / unlimited pps | 3/3 | 1543.27 → 1542.78 | 3.96 → 3.98 (+0.52%) | 76.45 → 76.83 | 2.92 → 2.88 | 3/3 |
| x86_64 / 6.18.60 | udp | burst / 10000 pps | 3/3 | 96.00 → 96.00 | 4.72 → 4.69 (-0.50%) | 5.66 → 5.63 | 2.88 → 2.85 | 0/3 |
| x86_64 / 6.18.60 | udp | sliding / unlimited pps | 3/3 | 1895.86 → 1912.35 | 4.13 → 4.09 (-0.81%) | 97.83 → 97.88 | 2.89 → 2.85 | 3/3 |
