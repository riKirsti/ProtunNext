# Android integration

The Android library calls the Rust compatibility runtime through generated
bindings; it has no Kotlin packet-forwarding loop. See the
[build guide](../docs/building.md#android) for the pinned toolchain and AAR output.
The package namespace is `io.github.rikirsti.protunnext.android`, with generated
records under `io.github.rikirsti.protunnext.generated`.

Create one `ProtunNextCore` in `Application.onCreate`, including restored
processes. Supply `Dependencies` with a notification factory and system event
handler. Create the notification channel before returning a notification.
Implement process-restored and always-on callbacks by choosing the appropriate
configuration and invoking the connection manager.

Call `connectionManager.prepare()` from the UI and launch a returned consent
Intent before connecting. Observe `state`, `events`, `connectionStats` and
`localAgentStats`; errors/refusals are not successful connection states. Account
configuration and tokens are never stored in Intent extras. Use connection
updates for peers, settings and interface configuration; use explicit cache and
certificate clearing for account changes.

Caches are application-private. Supply a suitable `PersistentCacheCipher` for
secure persistence. Cipher failure leaves only memory state and removes stale
persisted bytes; it does not fall back to plaintext. Without a cipher, values
are plaintext in private application preferences. Cache work stays off packet I/O.

The host app supplies notification/lifecycle integration, target SDK 37 and the
required 16 KB deployment environment. Real-device VPN consent, handover,
always-on/lockdown, revoke, process restore, secure storage and native loading
need target acceptance. CI's arm64-v8a AAR is development/build evidence, not
live account or device qualification. See [deployment](../docs/deployment.md).
