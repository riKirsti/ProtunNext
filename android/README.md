# Android development library

This independent service layer calls the actual Rust compatibility connection.
It contains no Kotlin packet-forwarding loop. Generated native records expose
every Rust account setting, certificate duration, API environment, exit label,
peer/protocol selection, state/event and Plus-policy feedback field.

The package is `io.github.rikirsti.protunnext.android`, with native records under
`io.github.rikirsti.protunnext.generated`. This is a new API, not a binary/source
drop-in replacement for `me.proton.vpn.core`. Device verification is deferred.

## Application integration

Create one `ProtunNextCore` from `Application.onCreate`, including when Android
restores the process. Supply `Dependencies` with a notification factory and a
`SystemEventHandler`. Create the notification's channel before returning the
notification. Implement `onProcessRestored` and `onAlwaysOnEnabled` by choosing
the desired account/configuration and calling `connectionManager.connect`.
The library does not persist full connection configurations or tokens in Intents.

Call `connectionManager.prepare()` from the UI and launch the returned Android
consent Intent if non-null. Connect only after consent. Permission failures and
service/interface failures are reported through `failures`; native policy/API
errors arrive in `events` and `state`, without reclassifying them as success.

Observe `state`, `events`, `connectionStats` and `localAgentStats` as Kotlin flows.
Statistics are requested every second while connected; LocalAgent requests are
every ten seconds. `requestStats` and `requestLocalAgentStats` also make explicit
requests. Their results retain the native fields and monotonic timestamps.

| v3.0.5 function | ProtunNext Android path |
| --- | --- |
| Connect/reconnect; certificate/all invalidation | `connect(AndroidConnectionConfig, InvalidateRequest?)` |
| Same-user live peers/settings/interface updates | Native atomic `ConfigUpdate`; `updatePeers`, `updateLocalAgentSettings`, `updateInterfaceConfig` |
| Account switch / changed API environment | Closes the old native connection before selecting the new per-user cache |
| TUN/DNS/routes/IPv6/per-app inclusion or exclusion | `InterfaceConfig`; gateway routes and Proton DNS fallbacks follow the pinned source |
| Socket protection and underlying network | `VpnService.protect`, duplicate-descriptor `Network.bindSocket`, network-change notifications |
| Selector/cookies and certificate refresh | `updateApiSelector`, `invalidateCertificate(userId)` |
| Capture file/descriptor, mode and limits | `setPacketCaptureEnabled(PcapFileInfo?)` |
| State, traffic, NetShield stats, errors, debug | Native callback flows, stat requests, `debugLog` |
| Cache encryption, clear account / certificate | Optional `PersistentCacheCipher`; `clearCache`, `clearCertificate` |
| Always-on, lockdown, process restore and revoke | Status fields, host system callbacks and service shutdown |

The per-user cache uses application-private preferences. A supplied cipher
failure leaves only a memory copy and removes stale persisted bytes; it never
falls back to persisting plaintext. Without a supplied cipher, storage is
plaintext in application-private preferences, as permitted by the original API.
Cache operations can block their calling account/control thread; packet I/O
does not call Android preferences or the cipher.

## Build

Use JDK17, Gradle8.14.3, SDK36 and an Android NDK. Versions of AGP, Kotlin,
coroutines, JNA and the minimum API25 follow the pinned upstream Android build.
The build also enables Java library desugaring for the generated duration API.
From the repository root on Linux or macOS:

```sh
bash scripts/build-android.sh aarch64-linux-android
# Optional complete ABI set:
bash scripts/build-android.sh aarch64-linux-android armv7-linux-androideabi x86_64-linux-android i686-linux-android
```

The helper builds native libraries, generates bindings from the first actual
Android ELF library, compiles the service/unit tests, and assembles a development
AAR under `android/build/outputs/aar`. It installs nothing. CI currently compiles
arm64-v8a; the other ABI commands are provided but remain unverified. A host app
must enable core-library desugaring when supporting API25.

## Evidence and limits

The Rust worker reads complete TUN packets, rather than truncating them to its
initial MTU. Interface MTUs from1280 to16351 fit all supported transports;
larger datagrams exceed the normal TunSafe frame limit. Native UDP's separate
wire limit remains65503. This wrapper validates the common transport bound.

API31+ uses the framework's best matching non-VPN network. Earlier APIs track
validated non-VPN networks and retain a working uplink until it becomes
unavailable or the framework exposes a different active non-VPN network.
Real-device handover, always-on/process restore, revocation, AndroidKeystore
integration and native JNA loading remain acceptance work. Compiling an AAR
does not prove those platform behaviors or live Proton Plus interoperability.

Source references:

- [Pinned v3.0.5 Android API and service](https://github.com/ProtonVPN/protun/tree/a498c38ae6ffb0e68e7190b2831ef985d73fa6da/vpn-core-android/vpn-core)
- [Android VPN lifecycle and per-app routing](https://developer.android.com/develop/connectivity/vpn)
- [VpnService protection and underlying networks](https://developer.android.com/reference/android/net/VpnService)
- [Network socket binding](https://developer.android.com/reference/android/net/Network#bindSocket(java.io.FileDescriptor))
- [Foreground service types: VPN system exemption](https://developer.android.com/develop/background-work/services/fgs/service-types#system-exempted)
