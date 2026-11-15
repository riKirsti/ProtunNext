package io.github.rikirsti.protunnext.android

import android.app.Notification
import android.content.Context
import android.net.VpnService
import io.github.rikirsti.protunnext.generated.*

data class AndroidStatus(val vpn: VpnState? = null, val alwaysOn: Boolean? = null,
    val lockdown: Boolean? = null)
enum class FailureKind { PermissionMissing, Interface, Service, Revoked, QueueFull }
data class AndroidFailure(val kind: FailureKind, val message: String? = null)
interface SystemEventHandler {
    fun onProcessRestored()
    fun onAlwaysOnEnabled()
}
data class Dependencies(
    val notificationId: Int,
    val notification: (VpnService, AndroidStatus) -> Notification,
    val systemEvents: SystemEventHandler,
    val cacheCipher: PersistentCacheCipher? = null,
    val logger: ClientLogger? = null,
    val logLevel: LogLevel = LogLevel.INFO,
)

/** Initialize once from Application.onCreate, including a system-restored process. */
class ProtunNextCore private constructor(context: Context, internal val dependencies: Dependencies) {
    private val context = context.applicationContext
    internal val cache = CacheStore(this.context, dependencies.cacheCipher)
    val connectionManager = ConnectionManager(this.context)
    fun clearCache(userId: String) = cache.forUser(userId).clearAll()
    fun clearCertificate(userId: String) = cache.forUser(userId).remove(CacheKey.CERTIFICATE)
    companion object {
        @Volatile internal var instance: ProtunNextCore? = null
            private set
        @Synchronized fun create(context: Context, dependencies: Dependencies): ProtunNextCore {
            check(instance == null) { "ProtunNextCore is already initialized" }
            require(dependencies.notificationId > 0)
            dependencies.logger?.let { initLogger(dependencies.logLevel, it) }
            return ProtunNextCore(context, dependencies).also { instance = it }
        }
    }
}
