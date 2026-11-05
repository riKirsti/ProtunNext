Paired median changes; percentages are not additive. Positive throughput is faster; negative CPU/RSS/latency is lower. CI is exploratory hierarchical bootstrap with few runner allocations.

| Family | Kernel / arch | Transport / bytes / window | Candidate | Pairs / runners | Throughput change | Worker CPU/B | 95% interval |
|---|---|---|---|---:|---:|---:|---:|
| PGO-whole-fixture | 7.0.0-1017-azure / aarch64 | stealth / 64 / 32 | profile-use-vs-baseline | 12 / 3 | +4.21% | -4.47% | [+2.11, +7.84]% |
| PGO-whole-fixture | 7.0.0-1017-azure / aarch64 | stealth / 1200 / 32 | profile-use-vs-baseline | 12 / 3 | +1.43% | -2.26% | [+1.03, +2.27]% |
| PGO-whole-fixture | 7.0.0-1017-azure / aarch64 | tcp / 64 / 32 | profile-use-vs-baseline | 12 / 3 | +0.87% | -0.39% | [-0.94, +2.78]% |
| PGO-whole-fixture | 7.0.0-1017-azure / aarch64 | tcp / 1200 / 32 | profile-use-vs-baseline | 12 / 3 | +2.49% | -3.07% | [+1.16, +3.73]% |
| PGO-whole-fixture | 7.0.0-1017-azure / aarch64 | udp / 64 / 32 | profile-use-vs-baseline | 12 / 3 | +1.04% | -0.94% | [+0.13, +2.63]% |
| PGO-whole-fixture | 7.0.0-1017-azure / aarch64 | udp / 1200 / 32 | profile-use-vs-baseline | 12 / 3 | +0.40% | -0.39% | [-1.53, +1.67]% |
| PGO-whole-fixture | 7.0.0-1017-azure / x86_64 | stealth / 64 / 32 | profile-use-vs-baseline | 12 / 3 | +3.92% | -3.97% | [+1.92, +9.53]% |
| PGO-whole-fixture | 7.0.0-1017-azure / x86_64 | stealth / 1200 / 32 | profile-use-vs-baseline | 12 / 3 | -2.09% | -2.02% | [-5.44, +5.63]% |
| PGO-whole-fixture | 7.0.0-1017-azure / x86_64 | tcp / 64 / 32 | profile-use-vs-baseline | 12 / 3 | +3.26% | -5.85% | [+1.52, +11.31]% |
| PGO-whole-fixture | 7.0.0-1017-azure / x86_64 | tcp / 1200 / 32 | profile-use-vs-baseline | 12 / 3 | +6.07% | -1.82% | [-2.05, +12.66]% |
| PGO-whole-fixture | 7.0.0-1017-azure / x86_64 | udp / 64 / 32 | profile-use-vs-baseline | 12 / 3 | +3.24% | -3.93% | [-2.22, +11.38]% |
| PGO-whole-fixture | 7.0.0-1017-azure / x86_64 | udp / 1200 / 32 | profile-use-vs-baseline | 12 / 3 | +6.18% | -3.08% | [-1.70, +18.24]% |
| TLS-component | 25.6.0 / aarch64 | — / 1280 / — | download:TLSv1_2 | 12 / 3 | +0.32% | — | [-10.79, +12.39]% |
| TLS-component | 25.6.0 / aarch64 | — / 1280 / — | download:TLSv1_3 | 12 / 3 | +5.57% | — | [-4.11, +13.44]% |
| TLS-component | 25.6.0 / aarch64 | — / 1280 / — | framed-download:TLSv1_2 | 12 / 3 | -26.99% | — | [-31.26, -4.61]% |
| TLS-component | 25.6.0 / aarch64 | — / 1280 / — | framed-download:TLSv1_3 | 12 / 3 | -25.77% | — | [-29.19, -15.27]% |
| TLS-component | 25.6.0 / aarch64 | — / 1280 / — | upload-burst:TLSv1_2 | 12 / 3 | -26.11% | — | [-31.73, -14.14]% |
| TLS-component | 25.6.0 / aarch64 | — / 1280 / — | upload-burst:TLSv1_3 | 12 / 3 | -28.01% | — | [-38.74, -11.25]% |
| TLS-component | 25.6.0 / aarch64 | — / 1280 / — | upload:TLSv1_2 | 12 / 3 | +1.19% | — | [-16.48, +13.96]% |
| TLS-component | 25.6.0 / aarch64 | — / 1280 / — | upload:TLSv1_3 | 12 / 3 | -5.44% | — | [-40.14, +3.59]% |
| TLS-component | 25.6.0 / aarch64 | — / 16352 / — | framed-download:TLSv1_2 | 12 / 3 | -30.83% | — | [-36.91, -27.15]% |
| TLS-component | 25.6.0 / aarch64 | — / 16352 / — | framed-download:TLSv1_3 | 12 / 3 | -29.91% | — | [-34.02, -24.95]% |
| TLS-component | 25.6.0 / aarch64 | — / 16384 / — | download:TLSv1_2 | 12 / 3 | -23.50% | — | [-35.48, -5.76]% |
| TLS-component | 25.6.0 / aarch64 | — / 16384 / — | download:TLSv1_3 | 12 / 3 | -36.14% | — | [-40.20, -14.01]% |
| TLS-component | 25.6.0 / aarch64 | — / 16384 / — | upload-burst:TLSv1_2 | 12 / 3 | -7.10% | — | [-25.90, +13.06]% |
| TLS-component | 25.6.0 / aarch64 | — / 16384 / — | upload-burst:TLSv1_3 | 12 / 3 | -1.07% | — | [-19.37, +4.27]% |
| TLS-component | 25.6.0 / aarch64 | — / 16384 / — | upload:TLSv1_2 | 12 / 3 | +0.65% | — | [-18.78, +11.27]% |
| TLS-component | 25.6.0 / aarch64 | — / 16384 / — | upload:TLSv1_3 | 12 / 3 | -3.66% | — | [-7.71, +9.53]% |
| TLS-component | 27.0.0 / aarch64 | — / 1280 / — | download:TLSv1_2 | 12 / 3 | -2.25% | — | [-6.23, +26.98]% |
| TLS-component | 27.0.0 / aarch64 | — / 1280 / — | download:TLSv1_3 | 12 / 3 | +6.43% | — | [-5.79, +34.03]% |
| TLS-component | 27.0.0 / aarch64 | — / 1280 / — | framed-download:TLSv1_2 | 12 / 3 | -10.77% | — | [-28.88, +3.27]% |
| TLS-component | 27.0.0 / aarch64 | — / 1280 / — | framed-download:TLSv1_3 | 12 / 3 | -20.43% | — | [-28.41, -1.41]% |
| TLS-component | 27.0.0 / aarch64 | — / 1280 / — | upload-burst:TLSv1_2 | 12 / 3 | -23.45% | — | [-35.10, -11.73]% |
| TLS-component | 27.0.0 / aarch64 | — / 1280 / — | upload-burst:TLSv1_3 | 12 / 3 | -24.97% | — | [-34.83, -6.77]% |
| TLS-component | 27.0.0 / aarch64 | — / 1280 / — | upload:TLSv1_2 | 12 / 3 | +2.77% | — | [-2.55, +20.17]% |
| TLS-component | 27.0.0 / aarch64 | — / 1280 / — | upload:TLSv1_3 | 12 / 3 | -0.96% | — | [-20.11, +12.34]% |
| TLS-component | 27.0.0 / aarch64 | — / 16352 / — | framed-download:TLSv1_2 | 12 / 3 | -23.57% | — | [-31.63, -11.52]% |
| TLS-component | 27.0.0 / aarch64 | — / 16352 / — | framed-download:TLSv1_3 | 12 / 3 | -24.86% | — | [-39.04, -9.42]% |
| TLS-component | 27.0.0 / aarch64 | — / 16384 / — | download:TLSv1_2 | 12 / 3 | -29.62% | — | [-36.16, -11.25]% |
| TLS-component | 27.0.0 / aarch64 | — / 16384 / — | download:TLSv1_3 | 12 / 3 | -27.55% | — | [-36.01, -19.39]% |
| TLS-component | 27.0.0 / aarch64 | — / 16384 / — | upload-burst:TLSv1_2 | 12 / 3 | -15.61% | — | [-26.60, -0.83]% |
| TLS-component | 27.0.0 / aarch64 | — / 16384 / — | upload-burst:TLSv1_3 | 12 / 3 | -1.19% | — | [-14.51, +20.83]% |
| TLS-component | 27.0.0 / aarch64 | — / 16384 / — | upload:TLSv1_2 | 12 / 3 | -1.50% | — | [-16.15, +8.01]% |
| TLS-component | 27.0.0 / aarch64 | — / 16384 / — | upload:TLSv1_3 | 12 / 3 | -1.16% | — | [-9.74, +3.15]% |
| TLS-component | 6.12.115 / x86_64 | — / 1280 / — | download:TLSv1_2 | 4 / 1 | -21.32% | — | [-42.66, -15.44]% |
| TLS-component | 6.12.115 / x86_64 | — / 1280 / — | download:TLSv1_3 | 4 / 1 | +112.79% | — | [+87.55, +524.14]% |
| TLS-component | 6.12.115 / x86_64 | — / 1280 / — | framed-download:TLSv1_2 | 4 / 1 | +57.95% | — | [-4.41, +124.70]% |
| TLS-component | 6.12.115 / x86_64 | — / 1280 / — | framed-download:TLSv1_3 | 4 / 1 | +82.63% | — | [+78.35, +92.94]% |
| TLS-component | 6.12.115 / x86_64 | — / 1280 / — | upload-burst:TLSv1_2 | 4 / 1 | +152.26% | — | [+113.78, +259.74]% |
| TLS-component | 6.12.115 / x86_64 | — / 1280 / — | upload-burst:TLSv1_3 | 4 / 1 | +243.00% | — | [+162.06, +292.22]% |
| TLS-component | 6.12.115 / x86_64 | — / 1280 / — | upload:TLSv1_2 | 4 / 1 | +8.16% | — | [+1.09, +16.05]% |
| TLS-component | 6.12.115 / x86_64 | — / 1280 / — | upload:TLSv1_3 | 4 / 1 | +100.90% | — | [-2.55, +207.14]% |
| TLS-component | 6.12.115 / x86_64 | — / 16352 / — | framed-download:TLSv1_2 | 4 / 1 | +86.66% | — | [+80.92, +102.29]% |
| TLS-component | 6.12.115 / x86_64 | — / 16352 / — | framed-download:TLSv1_3 | 4 / 1 | +81.60% | — | [+68.39, +104.97]% |
| TLS-component | 6.12.115 / x86_64 | — / 16384 / — | download:TLSv1_2 | 4 / 1 | +133.38% | — | [+95.89, +150.87]% |
| TLS-component | 6.12.115 / x86_64 | — / 16384 / — | download:TLSv1_3 | 4 / 1 | +131.47% | — | [+112.08, +152.42]% |
| TLS-component | 6.12.115 / x86_64 | — / 16384 / — | upload-burst:TLSv1_2 | 4 / 1 | +87.85% | — | [+59.95, +117.10]% |
| TLS-component | 6.12.115 / x86_64 | — / 16384 / — | upload-burst:TLSv1_3 | 4 / 1 | +86.10% | — | [+66.27, +145.61]% |
| TLS-component | 6.12.115 / x86_64 | — / 16384 / — | upload:TLSv1_2 | 4 / 1 | +146.64% | — | [+122.83, +180.37]% |
| TLS-component | 6.12.115 / x86_64 | — / 16384 / — | upload:TLSv1_3 | 4 / 1 | +147.59% | — | [+125.10, +175.88]% |
| TLS-component | 6.18.60 / x86_64 | — / 1280 / — | download:TLSv1_2 | 4 / 1 | -30.56% | — | [-54.84, -11.63]% |
| TLS-component | 6.18.60 / x86_64 | — / 1280 / — | download:TLSv1_3 | 4 / 1 | +108.59% | — | [+103.69, +509.48]% |
| TLS-component | 6.18.60 / x86_64 | — / 1280 / — | framed-download:TLSv1_2 | 4 / 1 | +14.69% | — | [+2.07, +65.58]% |
| TLS-component | 6.18.60 / x86_64 | — / 1280 / — | framed-download:TLSv1_3 | 4 / 1 | +95.42% | — | [+91.54, +129.52]% |
| TLS-component | 6.18.60 / x86_64 | — / 1280 / — | upload-burst:TLSv1_2 | 4 / 1 | +139.57% | — | [+105.31, +215.92]% |
| TLS-component | 6.18.60 / x86_64 | — / 1280 / — | upload-burst:TLSv1_3 | 4 / 1 | +228.57% | — | [+201.24, +280.90]% |
| TLS-component | 6.18.60 / x86_64 | — / 1280 / — | upload:TLSv1_2 | 4 / 1 | -0.57% | — | [-4.00, +1.76]% |
| TLS-component | 6.18.60 / x86_64 | — / 1280 / — | upload:TLSv1_3 | 4 / 1 | +6.29% | — | [-3.40, +26.91]% |
| TLS-component | 6.18.60 / x86_64 | — / 16352 / — | framed-download:TLSv1_2 | 4 / 1 | +77.54% | — | [+26.23, +122.70]% |
| TLS-component | 6.18.60 / x86_64 | — / 16352 / — | framed-download:TLSv1_3 | 4 / 1 | +69.37% | — | [+30.95, +97.50]% |
| TLS-component | 6.18.60 / x86_64 | — / 16384 / — | download:TLSv1_2 | 4 / 1 | +152.32% | — | [+137.39, +160.85]% |
| TLS-component | 6.18.60 / x86_64 | — / 16384 / — | download:TLSv1_3 | 4 / 1 | +144.72% | — | [+121.10, +151.18]% |
| TLS-component | 6.18.60 / x86_64 | — / 16384 / — | upload-burst:TLSv1_2 | 4 / 1 | +103.57% | — | [+100.89, +116.15]% |
| TLS-component | 6.18.60 / x86_64 | — / 16384 / — | upload-burst:TLSv1_3 | 4 / 1 | +108.48% | — | [+103.89, +122.88]% |
| TLS-component | 6.18.60 / x86_64 | — / 16384 / — | upload:TLSv1_2 | 4 / 1 | +167.00% | — | [+135.86, +211.27]% |
| TLS-component | 6.18.60 / x86_64 | — / 16384 / — | upload:TLSv1_3 | 4 / 1 | +191.74% | — | [+172.12, +211.37]% |
| TLS-component | Windows 11 reported 26H2, build 26300.9818 / x86_64 | — / 1280 / — | download:TLSv1_2 | 12 / 1 | -6.21% | — | [-17.24, -3.61]% |
| TLS-component | Windows 11 reported 26H2, build 26300.9818 / x86_64 | — / 1280 / — | download:TLSv1_3 | 12 / 1 | -5.69% | — | [-6.16, -4.39]% |
| TLS-component | Windows 11 reported 26H2, build 26300.9818 / x86_64 | — / 1280 / — | framed-download:TLSv1_2 | 12 / 1 | +1.93% | — | [-12.93, +25.95]% |
| TLS-component | Windows 11 reported 26H2, build 26300.9818 / x86_64 | — / 1280 / — | framed-download:TLSv1_3 | 12 / 1 | -7.15% | — | [-23.55, +3.11]% |
| TLS-component | Windows 11 reported 26H2, build 26300.9818 / x86_64 | — / 1280 / — | upload-burst:TLSv1_2 | 12 / 1 | +11.60% | — | [+3.94, +20.27]% |
| TLS-component | Windows 11 reported 26H2, build 26300.9818 / x86_64 | — / 1280 / — | upload-burst:TLSv1_3 | 12 / 1 | +8.79% | — | [+6.94, +15.85]% |
| TLS-component | Windows 11 reported 26H2, build 26300.9818 / x86_64 | — / 1280 / — | upload:TLSv1_2 | 12 / 1 | +0.07% | — | [-1.47, +0.46]% |
| TLS-component | Windows 11 reported 26H2, build 26300.9818 / x86_64 | — / 1280 / — | upload:TLSv1_3 | 12 / 1 | +0.11% | — | [-0.12, +0.60]% |
| TLS-component | Windows 11 reported 26H2, build 26300.9818 / x86_64 | — / 16352 / — | framed-download:TLSv1_2 | 12 / 1 | +2.93% | — | [-5.61, +11.97]% |
| TLS-component | Windows 11 reported 26H2, build 26300.9818 / x86_64 | — / 16352 / — | framed-download:TLSv1_3 | 12 / 1 | -4.25% | — | [-8.57, +1.78]% |
| TLS-component | Windows 11 reported 26H2, build 26300.9818 / x86_64 | — / 16384 / — | download:TLSv1_2 | 12 / 1 | -10.02% | — | [-24.76, -4.67]% |
| TLS-component | Windows 11 reported 26H2, build 26300.9818 / x86_64 | — / 16384 / — | download:TLSv1_3 | 12 / 1 | -15.61% | — | [-21.25, +1.90]% |
| TLS-component | Windows 11 reported 26H2, build 26300.9818 / x86_64 | — / 16384 / — | upload-burst:TLSv1_2 | 12 / 1 | -3.53% | — | [-10.66, +15.60]% |
| TLS-component | Windows 11 reported 26H2, build 26300.9818 / x86_64 | — / 16384 / — | upload-burst:TLSv1_3 | 12 / 1 | -4.15% | — | [-11.23, +4.23]% |
| TLS-component | Windows 11 reported 26H2, build 26300.9818 / x86_64 | — / 16384 / — | upload:TLSv1_2 | 12 / 1 | -6.37% | — | [-9.49, +0.74]% |
| TLS-component | Windows 11 reported 26H2, build 26300.9818 / x86_64 | — / 16384 / — | upload:TLSv1_3 | 12 / 1 | -2.67% | — | [-9.43, +8.49]% |
| production-bridge | 7.0.0-1017-azure / aarch64 | stealth / 1200 / 32 | PR2-f53-vs-selected-reference | 12 / 3 | -0.07% | -0.08% | [-0.68, +0.35]% |
| production-bridge | 7.0.0-1017-azure / aarch64 | tcp / 1200 / 32 | PR2-f53-vs-selected-reference | 12 / 3 | +1.92% | -2.03% | [-1.65, +3.46]% |
| production-bridge | 7.0.0-1017-azure / aarch64 | udp / 1200 / 32 | PR2-f53-vs-selected-reference | 12 / 3 | +0.28% | -0.86% | [-0.39, +2.22]% |
| production-bridge | 7.0.0-1017-azure / x86_64 | stealth / 1200 / 32 | PR2-f53-vs-selected-reference | 12 / 3 | +1.18% | -0.97% | [-1.72, +5.83]% |
| production-bridge | 7.0.0-1017-azure / x86_64 | tcp / 1200 / 32 | PR2-f53-vs-selected-reference | 12 / 3 | -2.69% | +1.67% | [-12.22, +0.31]% |
| production-bridge | 7.0.0-1017-azure / x86_64 | udp / 1200 / 32 | PR2-f53-vs-selected-reference | 12 / 3 | -3.72% | -0.53% | [-5.32, -1.09]% |
| shared-session-primitive | 6.12.115 / x86_64 | — / 64 / — | 2-workers | 6 / 1 | -25.52% | — | [-26.42, -22.29]% |
| shared-session-primitive | 6.12.115 / x86_64 | — / 64 / — | 4-workers | 6 / 1 | -74.60% | — | [-75.16, -73.12]% |
| shared-session-primitive | 6.12.115 / x86_64 | — / 1280 / — | 2-workers | 6 / 1 | +46.06% | — | [+44.61, +47.41]% |
| shared-session-primitive | 6.12.115 / x86_64 | — / 1280 / — | 4-workers | 6 / 1 | -17.98% | — | [-18.45, -17.35]% |
| shared-session-primitive | 6.18.60 / x86_64 | — / 64 / — | 2-workers | 6 / 1 | -24.15% | — | [-25.09, -23.00]% |
| shared-session-primitive | 6.18.60 / x86_64 | — / 64 / — | 4-workers | 6 / 1 | -66.12% | — | [-73.73, -44.51]% |
| shared-session-primitive | 6.18.60 / x86_64 | — / 1280 / — | 2-workers | 6 / 1 | +48.01% | — | [+46.16, +50.66]% |
| shared-session-primitive | 6.18.60 / x86_64 | — / 1280 / — | 4-workers | 6 / 1 | -17.18% | — | [-20.23, -15.86]% |
| shared-session-primitive | 7.0.0-1017-azure / aarch64 | — / 64 / — | 2-workers | 18 / 3 | +14.50% | — | [+5.88, +18.79]% |
| shared-session-primitive | 7.0.0-1017-azure / aarch64 | — / 64 / — | 4-workers | 18 / 3 | +18.36% | — | [-0.18, +23.27]% |
| shared-session-primitive | 7.0.0-1017-azure / aarch64 | — / 1280 / — | 2-workers | 18 / 3 | +77.07% | — | [+74.81, +79.46]% |
| shared-session-primitive | 7.0.0-1017-azure / aarch64 | — / 1280 / — | 4-workers | 18 / 3 | +212.82% | — | [+208.97, +220.17]% |
| shared-session-primitive | 7.0.0-1017-azure / x86_64 | — / 64 / — | 2-workers | 18 / 3 | +36.22% | — | [+14.15, +44.19]% |
| shared-session-primitive | 7.0.0-1017-azure / x86_64 | — / 64 / — | 4-workers | 18 / 3 | +31.08% | — | [+18.34, +39.73]% |
| shared-session-primitive | 7.0.0-1017-azure / x86_64 | — / 1280 / — | 2-workers | 18 / 3 | +76.49% | — | [+72.44, +79.19]% |
| shared-session-primitive | 7.0.0-1017-azure / x86_64 | — / 1280 / — | 4-workers | 18 / 3 | +98.99% | — | [+87.15, +113.27]% |
| socket | 6.12.115 / x86_64 | — / 64 / — | mmsg-vs-single | 4 / 1 | +16.15% | — | [+13.84, +17.04]% |
| socket | 6.12.115 / x86_64 | — / 64 / — | uring128-vs-batch:GSO=False,GRO=False | 4 / 1 | -13.82% | — | [-15.05, -10.09]% |
| socket | 6.12.115 / x86_64 | — / 64 / — | uring128-vs-batch:GSO=False,GRO=True | 4 / 1 | -14.04% | — | [-17.63, -13.79]% |
| socket | 6.12.115 / x86_64 | — / 64 / — | uring128-vs-batch:GSO=True,GRO=False | 4 / 1 | +16.49% | — | [+14.26, +18.15]% |
| socket | 6.12.115 / x86_64 | — / 64 / — | uring128-vs-batch:GSO=True,GRO=True | 4 / 1 | +26.89% | — | [+25.90, +31.10]% |
| socket | 6.12.115 / x86_64 | — / 1280 / — | mmsg-vs-single | 4 / 1 | +6.97% | — | [+3.96, +8.78]% |
| socket | 6.12.115 / x86_64 | — / 1280 / — | uring128-vs-batch:GSO=False,GRO=False | 4 / 1 | +1.64% | — | [+1.29, +6.11]% |
| socket | 6.12.115 / x86_64 | — / 1280 / — | uring128-vs-batch:GSO=False,GRO=True | 4 / 1 | +1.65% | — | [+1.17, +4.07]% |
| socket | 6.12.115 / x86_64 | — / 1280 / — | uring128-vs-batch:GSO=True,GRO=False | 4 / 1 | +19.96% | — | [+17.16, +20.18]% |
| socket | 6.12.115 / x86_64 | — / 1280 / — | uring128-vs-batch:GSO=True,GRO=True | 4 / 1 | +21.11% | — | [+15.77, +22.60]% |
| socket | 6.18.60 / x86_64 | — / 64 / — | mmsg-vs-single | 4 / 1 | +15.16% | — | [+13.97, +15.95]% |
| socket | 6.18.60 / x86_64 | — / 64 / — | uring128-vs-batch:GSO=False,GRO=False | 4 / 1 | -14.54% | — | [-15.37, -14.20]% |
| socket | 6.18.60 / x86_64 | — / 64 / — | uring128-vs-batch:GSO=False,GRO=True | 4 / 1 | -13.51% | — | [-13.95, -8.81]% |
| socket | 6.18.60 / x86_64 | — / 64 / — | uring128-vs-batch:GSO=True,GRO=False | 4 / 1 | +14.00% | — | [+13.57, +14.75]% |
| socket | 6.18.60 / x86_64 | — / 64 / — | uring128-vs-batch:GSO=True,GRO=True | 4 / 1 | +27.07% | — | [+22.86, +32.46]% |
| socket | 6.18.60 / x86_64 | — / 1280 / — | mmsg-vs-single | 4 / 1 | +6.34% | — | [+5.82, +8.95]% |
| socket | 6.18.60 / x86_64 | — / 1280 / — | uring128-vs-batch:GSO=False,GRO=False | 4 / 1 | +3.32% | — | [+1.56, +5.15]% |
| socket | 6.18.60 / x86_64 | — / 1280 / — | uring128-vs-batch:GSO=False,GRO=True | 4 / 1 | +2.79% | — | [+1.37, +3.76]% |
| socket | 6.18.60 / x86_64 | — / 1280 / — | uring128-vs-batch:GSO=True,GRO=False | 4 / 1 | +18.27% | — | [+17.04, +18.47]% |
| socket | 6.18.60 / x86_64 | — / 1280 / — | uring128-vs-batch:GSO=True,GRO=True | 4 / 1 | +17.87% | — | [+16.43, +18.28]% |
| worker | 6.12.115 / x86_64 | stealth / 64 / 1 | unbuffered-TLS13 | 12 / 3 | +0.61% | -0.43% | [-1.81, +3.54]% |
| worker | 6.12.115 / x86_64 | stealth / 1200 / 32 | TUN-vnet-aggregation (1s vs 1.5s samples) | 12 / 3 | -3.95% | +3.96% | [-11.11, +0.14]% |
| worker | 6.12.115 / x86_64 | stealth / 1200 / 32 | compact-buffered | 12 / 3 | +0.76% | -3.72% | [-4.50, +4.69]% |
| worker | 6.12.115 / x86_64 | stealth / 1200 / 32 | compact-inplace-buffered | 12 / 3 | +1.56% | -1.48% | [-0.68, +4.61]% |
| worker | 6.12.115 / x86_64 | stealth / 1200 / 32 | inplace-buffered | 12 / 3 | +4.72% | -5.57% | [+1.26, +10.31]% |
| worker | 6.12.115 / x86_64 | stealth / 1200 / 32 | inplace-full-buffered | 12 / 3 | +3.02% | -5.60% | [+0.56, +8.54]% |
| worker | 6.12.115 / x86_64 | stealth / 1200 / 32 | unbuffered-TLS12 | 12 / 3 | +48.42% | -33.19% | [+34.97, +79.80]% |
| worker | 6.12.115 / x86_64 | stealth / 1200 / 32 | unbuffered-TLS13 | 12 / 3 | +50.78% | -35.67% | [+44.30, +61.53]% |
| worker | 6.12.115 / x86_64 | stealth / 2048 / 16 | unbuffered-TLS13 | 12 / 3 | +2.89% | -1.24% | [-3.71, +11.82]% |
| worker | 6.12.115 / x86_64 | tcp / 1200 / 32 | compact-copy | 12 / 3 | +0.99% | +0.84% | [-10.80, +8.89]% |
| worker | 6.12.115 / x86_64 | tcp / 1200 / 32 | compact-inplace | 12 / 3 | +2.65% | -2.14% | [-3.61, +10.49]% |
| worker | 6.12.115 / x86_64 | tcp / 1200 / 32 | inplace-compact | 12 / 3 | +6.19% | -10.54% | [-2.50, +24.50]% |
| worker | 6.12.115 / x86_64 | tcp / 1200 / 32 | inplace-full | 12 / 3 | +5.78% | -9.08% | [+2.48, +9.96]% |
| worker | 6.12.115 / x86_64 | udp / 1200 / 32 | TUN-vnet-aggregation (1s vs 1.5s samples) | 12 / 3 | -13.67% | +21.68% | [-21.18, -0.64]% |
| worker | 6.12.115 / x86_64 | udp / 1200 / 32 | compact-copy | 12 / 3 | +0.64% | +2.49% | [-11.19, +2.08]% |
| worker | 6.12.115 / x86_64 | udp / 1200 / 32 | compact-inplace | 12 / 3 | +0.06% | -1.89% | [-6.61, +12.02]% |
| worker | 6.12.115 / x86_64 | udp / 1200 / 32 | inplace-compact | 12 / 3 | +7.75% | -9.86% | [+4.28, +14.79]% |
| worker | 6.12.115 / x86_64 | udp / 1200 / 32 | inplace-full | 12 / 3 | +4.79% | -8.67% | [-3.74, +15.54]% |
| worker | 6.12.115 / x86_64 | udp / 1200 / 32 | limit1-vs-limit16 | 12 / 3 | +4.56% | -12.31% | [-4.26, +14.62]% |
| worker | 6.12.115 / x86_64 | udp / 1200 / 32 | limit16-vs-limit16 | 12 / 3 | +0.00% | +0.00% | [+0.00, +0.00]% |
| worker | 6.12.115 / x86_64 | udp / 1200 / 32 | limit32-vs-limit16 | 12 / 3 | -0.22% | +0.04% | [-3.92, +7.41]% |
| worker | 6.12.115 / x86_64 | udp / 1200 / 32 | limit8-vs-limit16 | 12 / 3 | +0.98% | -2.84% | [-3.57, +15.41]% |
| worker | 6.12.115 / x86_64 | udp / 1200 / 32 | mmsg-vs-portable | 12 / 3 | -1.66% | +4.80% | [-12.15, +7.34]% |
| worker | 6.12.115 / x86_64 | udp / 1200 / 32 | outer-UDP-offloads | 12 / 3 | +2.59% | -9.32% | [-8.34, +16.47]% |
| worker | 6.12.115 / x86_64 | udp / 1200 / 32 | uring-no-offloads | 12 / 3 | -21.99% | +41.21% | [-28.46, -17.55]% |
| worker | 6.12.115 / x86_64 | udp / 1200 / 32 | uring-with-offloads | 12 / 3 | -17.30% | +37.79% | [-29.81, -14.84]% |
| worker | 6.18.60 / x86_64 | stealth / 64 / 1 | unbuffered-TLS13 | 12 / 3 | -1.13% | +4.45% | [-6.58, +2.37]% |
| worker | 6.18.60 / x86_64 | stealth / 1200 / 32 | TUN-vnet-aggregation (1s vs 1.5s samples) | 12 / 3 | -4.08% | +4.08% | [-4.85, +4.79]% |
| worker | 6.18.60 / x86_64 | stealth / 1200 / 32 | compact-buffered | 12 / 3 | -1.84% | +0.61% | [-4.60, +2.96]% |
| worker | 6.18.60 / x86_64 | stealth / 1200 / 32 | compact-inplace-buffered | 12 / 3 | +0.94% | -0.98% | [+0.43, +7.28]% |
| worker | 6.18.60 / x86_64 | stealth / 1200 / 32 | inplace-buffered | 12 / 3 | +6.22% | -6.32% | [+2.37, +9.12]% |
| worker | 6.18.60 / x86_64 | stealth / 1200 / 32 | inplace-full-buffered | 12 / 3 | +1.24% | -3.32% | [-0.35, +4.02]% |
| worker | 6.18.60 / x86_64 | stealth / 1200 / 32 | unbuffered-TLS12 | 12 / 3 | +45.58% | -34.20% | [+37.42, +69.49]% |
| worker | 6.18.60 / x86_64 | stealth / 1200 / 32 | unbuffered-TLS13 | 12 / 3 | +48.36% | -33.66% | [+39.47, +56.25]% |
| worker | 6.18.60 / x86_64 | stealth / 2048 / 16 | unbuffered-TLS13 | 12 / 3 | +10.82% | -16.14% | [+4.28, +24.43]% |
| worker | 6.18.60 / x86_64 | tcp / 1200 / 32 | compact-copy | 12 / 3 | +0.55% | -0.03% | [-15.55, +4.43]% |
| worker | 6.18.60 / x86_64 | tcp / 1200 / 32 | compact-inplace | 12 / 3 | +2.58% | -6.43% | [-14.33, +10.39]% |
| worker | 6.18.60 / x86_64 | tcp / 1200 / 32 | inplace-compact | 12 / 3 | +5.04% | -9.40% | [+2.77, +12.83]% |
| worker | 6.18.60 / x86_64 | tcp / 1200 / 32 | inplace-full | 12 / 3 | +1.92% | -6.66% | [-5.60, +24.60]% |
| worker | 6.18.60 / x86_64 | udp / 1200 / 32 | TUN-vnet-aggregation (1s vs 1.5s samples) | 12 / 3 | -12.46% | +21.88% | [-16.08, -9.76]% |
| worker | 6.18.60 / x86_64 | udp / 1200 / 32 | compact-copy | 12 / 3 | +0.80% | -6.07% | [-7.01, +10.80]% |
| worker | 6.18.60 / x86_64 | udp / 1200 / 32 | compact-inplace | 12 / 3 | -1.86% | +2.54% | [-7.70, +1.62]% |
| worker | 6.18.60 / x86_64 | udp / 1200 / 32 | inplace-compact | 12 / 3 | -0.09% | -1.38% | [-5.45, +7.63]% |
| worker | 6.18.60 / x86_64 | udp / 1200 / 32 | inplace-full | 12 / 3 | +2.15% | -0.60% | [+0.07, +6.54]% |
| worker | 6.18.60 / x86_64 | udp / 1200 / 32 | limit1-vs-limit16 | 12 / 3 | -0.96% | +13.79% | [-10.81, +4.34]% |
| worker | 6.18.60 / x86_64 | udp / 1200 / 32 | limit16-vs-limit16 | 12 / 3 | +0.00% | +0.00% | [+0.00, +0.00]% |
| worker | 6.18.60 / x86_64 | udp / 1200 / 32 | limit32-vs-limit16 | 12 / 3 | -0.50% | +4.66% | [-7.78, +1.02]% |
| worker | 6.18.60 / x86_64 | udp / 1200 / 32 | limit8-vs-limit16 | 12 / 3 | -1.43% | +0.30% | [-11.94, +0.63]% |
| worker | 6.18.60 / x86_64 | udp / 1200 / 32 | mmsg-vs-portable | 12 / 3 | +1.16% | -4.36% | [-6.33, +7.53]% |
| worker | 6.18.60 / x86_64 | udp / 1200 / 32 | outer-UDP-offloads | 12 / 3 | +10.61% | -16.74% | [+4.98, +23.10]% |
| worker | 6.18.60 / x86_64 | udp / 1200 / 32 | uring-no-offloads | 12 / 3 | -16.01% | +20.01% | [-20.02, -6.49]% |
| worker | 6.18.60 / x86_64 | udp / 1200 / 32 | uring-with-offloads | 12 / 3 | -13.55% | +30.58% | [-19.74, -11.94]% |
| worker | 7.0.0-1017-azure / aarch64 | stealth / 64 / 1 | unbuffered-TLS13 | 12 / 3 | -0.24% | +0.24% | [-2.66, +1.11]% |
| worker | 7.0.0-1017-azure / aarch64 | stealth / 1200 / 32 | TUN-vnet-aggregation (1s vs 1.5s samples) | 12 / 3 | -5.89% | +6.39% | [-6.40, -5.40]% |
| worker | 7.0.0-1017-azure / aarch64 | stealth / 1200 / 32 | compact-buffered | 12 / 3 | +0.04% | -1.02% | [-0.59, +0.98]% |
| worker | 7.0.0-1017-azure / aarch64 | stealth / 1200 / 32 | compact-inplace-buffered | 12 / 3 | +0.22% | +0.13% | [-0.06, +1.34]% |
| worker | 7.0.0-1017-azure / aarch64 | stealth / 1200 / 32 | inplace-buffered | 12 / 3 | +4.35% | -6.31% | [+3.38, +4.99]% |
| worker | 7.0.0-1017-azure / aarch64 | stealth / 1200 / 32 | inplace-full-buffered | 12 / 3 | +3.96% | -7.30% | [+2.93, +4.65]% |
| worker | 7.0.0-1017-azure / aarch64 | stealth / 1200 / 32 | unbuffered-TLS12 | 12 / 3 | -3.17% | +6.07% | [-3.74, -2.63]% |
| worker | 7.0.0-1017-azure / aarch64 | stealth / 1200 / 32 | unbuffered-TLS13 | 12 / 3 | -3.56% | +6.68% | [-4.61, -2.99]% |
| worker | 7.0.0-1017-azure / aarch64 | stealth / 2048 / 16 | unbuffered-TLS13 | 12 / 3 | -4.68% | +7.33% | [-5.69, -3.43]% |
| worker | 7.0.0-1017-azure / aarch64 | tcp / 1200 / 32 | compact-copy | 12 / 3 | +0.38% | -1.54% | [-0.18, +1.86]% |
| worker | 7.0.0-1017-azure / aarch64 | tcp / 1200 / 32 | compact-inplace | 12 / 3 | +0.65% | -0.64% | [-2.68, +4.22]% |
| worker | 7.0.0-1017-azure / aarch64 | tcp / 1200 / 32 | inplace-compact | 12 / 3 | +9.82% | -10.68% | [+5.25, +11.48]% |
| worker | 7.0.0-1017-azure / aarch64 | tcp / 1200 / 32 | inplace-full | 12 / 3 | +9.90% | -10.95% | [+7.59, +11.04]% |
| worker | 7.0.0-1017-azure / aarch64 | udp / 1200 / 32 | TUN-vnet-aggregation (1s vs 1.5s samples) | 12 / 3 | -8.87% | +10.02% | [-9.98, -8.52]% |
| worker | 7.0.0-1017-azure / aarch64 | udp / 1200 / 32 | compact-copy | 12 / 3 | -0.44% | -0.01% | [-0.78, +0.60]% |
| worker | 7.0.0-1017-azure / aarch64 | udp / 1200 / 32 | compact-inplace | 12 / 3 | -0.12% | +0.56% | [-1.50, +1.00]% |
| worker | 7.0.0-1017-azure / aarch64 | udp / 1200 / 32 | inplace-compact | 12 / 3 | +6.61% | -8.90% | [+5.31, +7.34]% |
| worker | 7.0.0-1017-azure / aarch64 | udp / 1200 / 32 | inplace-full | 12 / 3 | +7.21% | -9.80% | [+5.59, +7.74]% |
| worker | 7.0.0-1017-azure / aarch64 | udp / 1200 / 32 | limit1-vs-limit16 | 12 / 3 | -1.00% | +4.47% | [-1.86, +0.22]% |
| worker | 7.0.0-1017-azure / aarch64 | udp / 1200 / 32 | limit16-vs-limit16 | 12 / 3 | +0.00% | +0.00% | [+0.00, +0.00]% |
| worker | 7.0.0-1017-azure / aarch64 | udp / 1200 / 32 | limit32-vs-limit16 | 12 / 3 | -0.63% | +1.40% | [-1.33, +0.37]% |
| worker | 7.0.0-1017-azure / aarch64 | udp / 1200 / 32 | limit8-vs-limit16 | 12 / 3 | +0.43% | -0.42% | [-0.45, +2.22]% |
| worker | 7.0.0-1017-azure / aarch64 | udp / 1200 / 32 | mmsg-vs-portable | 12 / 3 | -0.22% | -2.12% | [-2.10, +1.02]% |
| worker | 7.0.0-1017-azure / aarch64 | udp / 1200 / 32 | outer-UDP-offloads | 12 / 3 | +7.96% | -13.48% | [+7.59, +10.16]% |
| worker | 7.0.0-1017-azure / aarch64 | udp / 1200 / 32 | uring-no-offloads | 12 / 3 | -2.57% | +3.81% | [-3.14, -1.24]% |
| worker | 7.0.0-1017-azure / aarch64 | udp / 1200 / 32 | uring-with-offloads | 12 / 3 | -4.56% | +5.61% | [-5.09, -2.04]% |
