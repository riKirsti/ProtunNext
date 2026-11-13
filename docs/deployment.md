# Deploy and integrate

ProtunNext is a host-integrated runtime, not a standalone VPN installer. The
release gate remains closed pending complete live service/device acceptance.
Build/package success is not a production readiness claim. No release or real
network change is made by ordinary builds/tests.

## Shared host responsibilities

Package the matching native library and regenerated language bindings. The host
owns lifecycle, permissions, secure persistence, interface/uplink choice, routes,
addresses and DNS. Preserve the previous network state and restore only owned
resources on failure, disconnect, provider shutdown and process lifecycle events.
Use the managed connection API for peer updates/recovery, callbacks, statistics
and account settings. Applied values/refusals must come from the service.

Keep private configurations and keys outside source control with restricted
access. Never ship public test credentials or trust the fixture CA. Avoid logging
tokens, selectors, keys, captures or personal device/network identifiers. Captures
are opt-in sensitive data; keep them disabled for ordinary operation and timing.
The host must supply an appropriate secure cache policy for credentials.

## Linux

The caller creates/configures a TUN and its addresses/routes/DNS, then passes
the descriptor and an explicit physical uplink. Raw descriptors use
`IFF_TUN | IFF_NO_PI`. Attachment duplicates ownership and makes the shared file
description nonblocking. Do not close a descriptor already consumed by an API.

For receive coalescing, create a **new, exclusively owned** Linux TUN with
`IFF_TUN | IFF_NO_PI | IFF_VNET_HDR`, call
`linux::prepare_receive_coalescing(fd)` before attachment, then attach normally.
Discard the new descriptor if preparation fails; setup ioctls are not
transactional. Never silently convert a raw caller descriptor. Negotiation uses
ten-byte little-endian headers and disables kernel-to-user offloads. Unsupported
packet groups retain ordinary delivery; aggregate rejection disables coalescing
and retries originals. Do not alter global NIC/sysctl settings to enable this.

The low-level CLI accepts an already-open inherited descriptor:

```sh
protun-next-cli check-config /path/to/private-config.conf
protun-next-cli run-udp /path/to/private-config.conf TUN_FD UPLINK_INTERFACE
protun-next-cli run-tcp /path/to/private-config.conf TUN_FD UPLINK_INTERFACE
protun-next-cli run-tls /path/to/private-config.conf TUN_FD UPLINK_INTERFACE SERVER_NAME
protun-next-cli run-stealth /path/to/private-config.conf TUN_FD UPLINK_INTERFACE random
```

Replace descriptor/interface placeholders with host-owned values. `run-tls` uses
ordinary certificate/name verification. Stealth also accepts `top` or an explicit
DNS SNI strategy; it retains its distinct policy plus inner WireGuard identity.
The CLI does not implement the managed account/application lifecycle and should
not be treated as an unattended production service or automatic router setup.
TCP/TLS MTU must not exceed 16,351 bytes under normal framing.

## Windows

Package the matching signed `wintun.dll` beside the host executable, along with
its license. The runtime loads only its absolute application-package location;
it does not search the current directory/PATH or download a driver. Adapter
creation and native address/route/DNS management need an elevated host. Creating
an adapter must remain an explicit application action. The runtime tracks owned
resources; do not remove other applications' routes or adapters during cleanup.
See [packaging commands](building.md#windows).

## Android and Apple

Android applications supply VpnService consent, notifications, network selection,
cache policy and lifecycle handling. Do not pass account configurations/tokens
through Intents. Test handover, always-on, revocation and process restoration on
real devices. See [Android integration](../android/README.md).

Apple applications supply NetworkExtension entitlements, provider lifecycle and
packet tunnel settings before connecting. Use the public packet-flow bridge;
do not extract a private utun FD through undocumented APIs. Stop/join native
work before releasing the bridge. See [Apple integration](../apple/README.md).

## Acceptance and updates

Validate the actual target OS/CPU, transport, account feature set, persistence,
disconnect/reconnect, network changes and teardown before deployment. Preserve
portable compiler defaults; CPU-native/PGO artifacts require separate exact-target
qualification and fresh representative training. No historical benchmark profile
is a general deployment artifact. Keep dependencies locked and verify vendor
provenance when updating. Broader live/device acceptance remains explicit through
the runtime requirements ledger; do not bypass its gate to publish a release.
