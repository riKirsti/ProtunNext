Apple integration
=================

Build only ARM64 for macOS, iOS/iPadOS and simulators, with deployment floor 26.
The host application owns NetworkExtension entitlements, provider lifecycle and
NEPacketTunnelNetworkSettings. Its routes, DNS, IPv6 and split-tunnel settings
must be applied before starting the native connection, and restored by the host
when stopping. ProtunNext's existing account, Plus, capture, callback, failover
and cache functions remain on the generated `Connection` interface.

`PacketFlowBridge` adapts public `NEPacketTunnelFlow.readPacketObjects` and
`writePacketObjects` to an application-owned Unix datagram pair. Pass its
`takeNativeDescriptor()` result to generated `Connection.unixConnect` (which
consumes it), retain the bridge for the connection lifetime, then call `start()`.
Its four-byte internal AF prefix matches the native Unix packet adapter. This
is not a private utun FD extracted through KVC or an undocumented property.
Apple explicitly does not guarantee access to that FD.

The bridge retains one packet-flow read and one bounded batch at a time. It
pauses reads under socket backpressure, uses writev without concatenating packet
payloads, and delivers native replies to the public API in batches of at most
64. Overflow, malformed packets and failed injection stop the bridge and invoke
the supplied failure callback; the host must disconnect/recover, not report
success. Stop/join the native connection before `bridge.stop()`. Creation itself
does not change system network settings. The socket-pair hop is a real cost;
performance on an entitled provider remains to be measured.

Build the Swift package with `swift build --package-path apple`. Use
`scripts/build-apple.sh` under the current Xcode 27 SDK to build the three Rust
ARM64 slices and generate Swift bindings. No Intel or Rosetta artifact is made.
CI compilation is separate from acceptance on an entitled VPN provider. Neither
the bridge nor typechecking establishes an actual Proton connection.

Verified API declarations: [readPacketObjects](https://developer.apple.com/documentation/networkextension/nepackettunnelflow/readpacketobjects(completionhandler:)),
[writePacketObjects](https://developer.apple.com/documentation/networkextension/nepackettunnelflow/writepacketobjects(_:)),
[NEPacket](https://developer.apple.com/documentation/networkextension/nepacket),
[DTS explanation of the public contract](https://developer.apple.com/forums/thread/756852).
