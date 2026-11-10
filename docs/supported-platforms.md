Production targets for this revision
====================================

| Platform | Supported runtime | CPU / packet path |
|---|---|---|
| Windows | Windows 11 25H2 (26200.x), 26H2 (26300.x) | Native x64, Wintun + Mio sockets |
| Linux / OpenWrt | 6.12.x and 6.18.x LTS | x64 / ARM64, raw TUN + recvmmsg/sendmmsg |
| Android | 16 / 17, API 36 / 37 | ARM64 primary, public VpnService + Bionic batching |
| macOS / iOS / iPadOS | 26 / 27 | ARM64 only, public NetworkExtension packet flow |

Use current security updates within each family. This explicit window advances
through reviewed releases; a larger version number is not an automatic support
claim. Windows 26H1, ARM emulation and Intel Macs are excluded. Android 16 KB
deployment is supplied by the application; no page-size probe, rejection or
separate compatibility audit is added. Other Android ABI helper targets remain
available for consumers, without implying device acceptance for those targets.

Windows checks the two build families and native CPU before creating Wintun.
Linux callers create their own interface/routing; deploy only on the two tested
kernel families, and supply raw TUN without packet-info headers, or explicitly
prepare the Linux-only [receive-coalescing descriptor contract](benchmarks/stage3-tun.md).
Android keeps its raw TUN contract. Apple
artifacts use deployment floor 26 and ARM64 slices, including simulators. Android
uses min/NDK API 36 and compile/target SDK 37; no maxSdkVersion cap is imposed.

All active workers use compact bounded packet storage and in-place WireGuard
data crypto. TLS is selected at compile time: buffered rustls on Windows and all
Apple targets, unbuffered on Linux/Android. Both ordinary TLS and Stealth use
this selection. Linux x64/Travel have supporting worker/WAN data; other ARM64
workloads differ. Buffered is supported by macOS component results, provisional
on Windows and iOS/iPadOS; Android's native performance choice remains unverified.
See [the platform evidence and limits](tls-selection-20261105.md).

The Linux backend remains mmsg batching. The experimental io_uring receiver and
its overflow correction are absent from production. No liburing package is
required. GSO/GRO capabilities are negotiated once inside the batch adapter;
unsupported offload retains the same batch backend.
The old removed vnet-header experiment was not a Protun v3.0.5 requirement.
Stage 3 introduces a separate explicit receive-only contract, preserving original
raw-TUN behavior and disabling kernel-to-userspace segmentation offloads.

The `unix`, `linux`, `android`, `apple` and `windows` Cargo features remain small
convenience bundles for existing application build scripts. They do not select
different packet algorithms or enable old-OS compatibility branches. Local Agent,
account-free builds, generated bindings, development helpers and diagnostics are
genuine build products and remain separate. No account, Plus, failover, capture,
DNS, local HTTP service, always-on, lockdown or per-app routing function is removed.

Host CI proves source behavior on its recorded host; it is not Windows 11 or
mobile VPN acceptance. The separate Linux VM job boots each supported LTS
kernel. Apple and Android runtime acceptance still requires the corresponding
device/provider and account tests. The existing release gate remains closed.

Sources verified November 3, 2026: [Microsoft release table](https://learn.microsoft.com/en-us/windows/release-health/windows11-release-information),
[native architecture API](https://learn.microsoft.com/en-us/windows/win32/api/wow64apiset/nf-wow64apiset-iswow64process2),
[kernel release feed](https://www.kernel.org/releases.json),
[Android API 37 toolchain](https://developer.android.com/build/releases/agp-9-1-0-release-notes),
[built-in Kotlin migration](https://developer.android.com/build/migrate-to-built-in-kotlin),
[Apple public packet flow](https://developer.apple.com/documentation/networkextension/nepackettunnelflow),
[Apple DTS guidance](https://developer.apple.com/forums/thread/756852),
[GitHub runner architectures](https://docs.github.com/en/actions/reference/runners/github-hosted-runners).
