# Apple integration

Build ARM64 for macOS, iOS/iPadOS and simulators with deployment floor 26.
See the [build guide](../docs/building.md#apple) for Xcode and native slices.
The host owns NetworkExtension entitlements, provider lifecycle and
`NEPacketTunnelNetworkSettings`, including restoration of owned settings.

`PacketFlowBridge` adapts public `NEPacketTunnelFlow.readPacketObjects` and
`writePacketObjects` through an application-owned Unix datagram pair. Pass
`takeNativeDescriptor()` to generated `Connection.unixConnect` (which consumes
it), retain the bridge for the connection lifetime, then call `start()`.
Its internal four-byte address-family prefix matches the native Unix adapter.
Never extract a private utun descriptor using KVC or undocumented properties.

The bridge retains one read and a bounded batch, pauses under backpressure and
delivers replies in batches of at most 64. Overflow, malformed input or injection
failure stops the bridge and calls the failure callback; the host must recover
or disconnect. Stop/join the native connection before calling `bridge.stop()`.
Bridge construction itself changes no network settings.

The generated connection API retains account/settings, captures, statistics,
callbacks, recovery and cache functions. SDK compilation/typechecking does not
establish actual VPN-provider or live account acceptance. Validate entitlements,
real routing/DNS, lifecycle, secure persistence and teardown on the target.
