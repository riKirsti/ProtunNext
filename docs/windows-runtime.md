# Windows implementation checkpoint

The Windows library now exports `ProTun.initialize/delete_routes` and
`WindowsConnection.connect/get_connection/get_adapter_details/set_ipv6/set_dns`.
The returned compatibility connection uses the shared packet worker, account
runtime, settings, failover, cache, capture, statistics and event contracts.

This is executable platform code, not an API declaration-only layer. It remains
a development checkpoint: alternate Windows subnet fallback and full virtual
network behavior are unfinished, and no real Wintun adapter has been activated
in verification. Full v3.0.5 code parity must not yet be asserted.

## Packet and socket path

The narrow Wintun C API boundary keeps the driver library, adapter and session
under ownership. Receive copies once from the driver ring into an existing pool
packet, then releases the ring slot. Send copies once into a driver allocation
and always submits it. Neither operation constructs a packet object, allocates
memory, clones an Arc or takes a packet lock. A wait-event bridge wakes the one
Mio reactor and waits for a ring-drain acknowledgement instead of spinning.

Outer UDP/TCP/TLS sockets use Mio IOCP readiness. Route selection happens once
per new socket, chooses a connected physical interface by longest prefix and
route-plus-interface metric, and excludes the owned TUN. `IP_UNICAST_IF` or
`IPV6_UNICAST_IF` is applied before connect, using the native-width SOCKET. No
per-peer host-route insertion is needed to protect the VPN transport.

TCP connect waits with bounded `WSAPoll` calls before registration with the
packet reactor; a temporary IOCP registration would prevent associating the
same socket with the worker's completion port. The transport then shares the
same bounded framing/TLS/partial-write code as Unix.

Interface addresses, split default routes, MTU/metric and DNS use native IP Helper
calls. IP-interface and route-change callbacks only signal a coalescing control
channel, which excludes the owned TUN and recreates sockets off the packet path.
The clock uses suspend-inclusive `QueryInterruptTimePrecise`. Owned route/address
rows are retained for cleanup; no other application's routes/adapters are opened
or removed. The adapter has a fresh unique name for each explicit construction.

## Build and package

Windows10 build19041 or later is required by `SetInterfaceDnsSettings`. Creating
a real Wintun adapter requires an elevated host application. Compiling, binding
generation, initialization and the loopback checks below require no adapter.

```powershell
cargo build --locked --release --lib --bin protun-next-cli
./scripts/package-windows.ps1
```

The script downloads official Wintun0.14.1, checks archive SHA256
`07c256185d6ee3652e09fa55c0b673e2624b565e02c4b9091c79ca7d2f24ef51`,
checks a valid WireGuard LLC Authenticode signature, verifies matching PE machine
types and copies a development bundle to `target/windows-package`. It does not
load/install the DLL or activate a VPN. The archive's full binary license is
included as `wintun-LICENSE.txt`. Other native build directories/architectures
can be selected with `-BuildDirectory` and `-Architecture`.

The embedding application packages the matching signed `wintun.dll` beside its
executable. The runtime loads only that absolute application-package path, with
DLL dependency search restricted to its directory and System32. It never loads
a driver from the current working directory or PATH and never downloads one.
Driver packaging/version checks are deployment responsibilities of the host.

Additional Rust boundary dependencies are `windows-sys`0.52 (MIT/Apache2) and
`libloading`0.8.9 (ISC); socket2 and Mio are shared with Unix. No Wintun Rust
framework or copy of the old Protun Windows event loop is used.

## Executed checks and remaining work

`tests/windows_network.rs` executes real loopback Winsock, Mio and Noise exchanges
over UDP, TCP, verified TLS and Proton-style Stealth. `windows::wait` tests real
Windows events, readiness acknowledgement and shutdown. Generated Python calls
actual native initialization and checks Windows records/methods without calling
the adapter constructor. Host tests and all-target/all-feature Clippy pass, and
the Unix path also passes AArch64-musl Clippy after the shared-boundary refactor.

The code currently configures the standard `10.2.0.*`/`2a07:b944::2:*` subnet.
Upstream's Windows helper tries subsequent subnets after address setup failure;
that fallback must be wired consistently through the private stack, probes,
policy, API/DNS and reported adapter details before closing this code gap.
Native adapter/DNS/routes/network-event execution, teardown during real network
changes, packaged C# host integration and live Proton Plus acceptance remain
deferred verification. Loopback tests prove neither those behaviors nor target
VPN throughput.

## Primary sources checked

- [Pinned Windows API](https://github.com/ProtonVPN/protun/blob/a498c38ae6ffb0e68e7190b2831ef985d73fa6da/src/api/windows/connection_windows.rs)
- [Official Wintun exports](https://git.zx2c4.com/wintun/tree/api/exports.def) and [C API header](https://git.zx2c4.com/wintun/tree/api/wintun.h)
- [Wintun ownership and close behavior](https://github.com/WireGuard/wintun/blob/master/README.md)
- [IP socket options](https://learn.microsoft.com/en-us/windows/win32/winsock/ipproto-ip-socket-options)
- [Per-interface DNS API](https://learn.microsoft.com/en-us/windows/win32/api/netioapi/nf-netioapi-setinterfacednssettings) and [settings fields](https://learn.microsoft.com/en-us/windows/win32/api/netioapi/ns-netioapi-dns_interface_settings)
