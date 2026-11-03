**TLS selection: unbuffered for the sustained-throughput production path**

Recorded November 3, 2026, before any production refactor. Reference PR head: `83f025231535a3ef0731123988e9dae8621cef4a`. Both router variants used the same frozen binary (SHA-256 `c6b1e549f6526f3061343a79f3abac98b353cf326840f1d847d96718a5a0fdea`), compiler, compact pool, in-place crypto, MTU 1280, peer keys and framing. Only the TLS adapter flag changed.

Travel was already Direct when this valid cohort began. Four counterbalanced AB/BA pairs on each of two distinct Plus peers, US-NY#601 and US-NY#505, produced 16 connected sessions and 32/32 valid, certificate-verified settled downloads. Every captured outer handshake negotiated TLS 1.3 / AES-256-GCM. This run lasted 633.953 seconds; independent cleanup verified Direct mode, unchanged saved configuration, active key, account helper, installed packages, interfaces, routing rules, default route, socket limits and router identity. All owned test processes, watchdog, files and temporary routes/interfaces were removed.

| Workload | Median buffered Mbps | Median unbuffered Mbps | Median paired throughput change | Median paired VPN CPU/byte change |
|---|---:|---:|---:|---:|
| 12 MB, 8 pairs | 81.72 | 77.65 | -8.15% | -21.16% |
| 50 MB, 8 pairs | 81.12 | 111.99 | +54.09% | -28.92% |

Paired changes are medians of within-server/round ratios, not the ratio of the two overall medians. Unbuffered won throughput in all eight 50 MB pairs and reduced process CPU per byte in all sixteen pairs. This repeatable bulk result supports retaining its extra state handling for the project's sustained-throughput goal. Median paired time-to-first-byte changes were -10 ms (12 MB) and -21 ms (50 MB); observed RSS was approximately 5.2 MiB for both.

This is **not a universal performance win**: unbuffered lost throughput in six of eight 12 MB pairs. Keep that regression visible for subsequent optimization and cross-platform validation; do not claim it was fixed. Five cold first requests failed with curl TLS EOF (buffered 3/8, unbuffered 2/8); no conclusion that the adapter fixes or causes that separate issue is justified. Cold requests were retained and were not silently counted as successful settled downloads.

The bounded follow-up local framing comparison exercised the actual borrowed-record decoder path, in addition to an earlier generic Read test. All 32 framing cases passed. Results depend strongly on packet shape: TLS 1.3 / 1280-byte frames showed a median paired throughput regression of 36.5%, while 16,352-byte frames improved 99.6%. Local loopback scheduling and the coarse Windows CPU timer limit inference. The earlier 96 generic Read cases also passed but do not exercise the production borrowed-record receive path. These measurements justify selecting unbuffered for measured router bulk traffic, **not certifying it as fastest for every platform/workload**. Release acceptance still requires the agreed supported-target validation.

The origin was kernel.org CDN, using exact 12/50 MB byte ranges, HTTP 206 and byte-count/content-range checks with certificate verification. Historical Hetzner absolute speeds are not comparable across origins. Earlier Local-mode nested-VPN samples and a Cloudflare direct HTTP 403 attempt are excluded, retained in a separate exclusion record, and supply no TLS performance/reliability evidence.

Raw results and paired statistics: [comparison JSON](benchmarks/tls-selection-20261103.json). Local framing: [framing JSON](benchmarks/tls-framing-20261103.json). Frozen tracked source and reference router executable were saved outside the production repository before changes. Production source remained unchanged throughout the comparison.
