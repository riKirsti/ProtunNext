package io.github.rikirsti.protunnext.android

import android.content.Intent
import android.content.pm.ServiceInfo
import android.net.*
import android.os.*
import io.github.rikirsti.protunnext.generated.*
import java.util.concurrent.atomic.AtomicLong

/** Same-process service. Tokens/configs never travel in Intent extras. */
class ProtunVpnService : VpnService() {
    internal class LocalBinder(val service: ProtunVpnService) : Binder()
    private val thread = HandlerThread("protun-next-android-control")
    private lateinit var control: Handler
    private lateinit var core: ProtunNextCore
    private lateinit var connectivity: ConnectivityManager
    private var current: Connection? = null // control-thread only
    private var config: AndroidConnectionConfig? = null
    private val generation = AtomicLong()
    @Volatile private var destroyed = false
    @Volatile private var underlying: Network? = null
    private var frameworkBest: Network? = null
    private val candidates = LinkedHashMap<Network, NetworkCapabilities>()
    private val links = HashMap<Network, LinkProperties>()
    private val callback = object : ConnectivityManager.NetworkCallback() {
        override fun onAvailable(network: Network) {
            submit {
                if (Build.VERSION.SDK_INT >= 31) frameworkBest = network
                connectivity.getNetworkCapabilities(network)?.let { candidates[network] = it }
                chooseNetwork()
            }
        }
        override fun onCapabilitiesChanged(network: Network, capabilities: NetworkCapabilities) {
            submit { candidates[network] = capabilities; chooseNetwork() }
        }
        override fun onLost(network: Network) {
            submit {
                candidates.remove(network)
                links.remove(network)
                if (frameworkBest == network) frameworkBest = null
                chooseNetwork()
            }
        }
        override fun onLinkPropertiesChanged(network: Network, properties: LinkProperties) {
            submit {
                val old = links.put(network, properties)
                if (network == underlying && old != null && old != properties)
                    current?.onConnectivityChange(ConnectivityEvent.NETWORK_SWITCH)
            }
        }
    }
    override fun onCreate() {
        super.onCreate()
        val initialized = ProtunNextCore.instance
        if (initialized == null) { stopSelf(); return }
        core = initialized
        thread.start()
        control = Handler(thread.looper)
        connectivity = getSystemService(ConnectivityManager::class.java)
        val request = NetworkRequest.Builder()
            .addCapability(NetworkCapabilities.NET_CAPABILITY_INTERNET)
            .addCapability(NetworkCapabilities.NET_CAPABILITY_NOT_VPN).build()
        if (Build.VERSION.SDK_INT >= 31)
            connectivity.registerBestMatchingNetworkCallback(request, callback, control)
        else connectivity.registerNetworkCallback(request, callback)
        submit {
            connectivity.allNetworks.forEach { network ->
                connectivity.getNetworkCapabilities(network)?.let { candidates[network] = it }
            }
            chooseNetwork()
        }
        control.postDelayed(object : Runnable {
            private var ticks = 0
            override fun run() {
                if (destroyed) return
                if (core.connectionManager.state.value.vpn?.connectionState is ConnectionState.Connected) {
                    current?.requestStats()
                    if (++ticks % 10 == 0) current?.requestLocalAgentStats()
                }
                control.postDelayed(this, 1000)
            }
        }, 1000)
    }
    override fun onBind(intent: Intent): IBinder? =
        if (!::core.isInitialized) null else if (intent.action == SERVICE_INTERFACE) super.onBind(intent)
        else LocalBinder(this)
    override fun onStartCommand(intent: Intent?, flags: Int, startId: Int): Int {
        if (!::core.isInitialized) return START_NOT_STICKY
        val notification = core.dependencies.notification(this, core.connectionManager.state.value)
        if (Build.VERSION.SDK_INT >= 34)
            startForeground(core.dependencies.notificationId, notification, ServiceInfo.FOREGROUND_SERVICE_TYPE_SYSTEM_EXEMPTED)
        else startForeground(core.dependencies.notificationId, notification)
        if (Build.VERSION.SDK_INT >= 29)
            core.connectionManager.environment(isAlwaysOn, isLockdownEnabled)
        when {
            intent == null -> core.dependencies.systemEvents.onProcessRestored()
            intent.action == SERVICE_INTERFACE -> core.dependencies.systemEvents.onAlwaysOnEnabled()
        }
        return START_STICKY
    }
    private fun chooseNetwork() {
        val usable = candidates.filterValues {
            it.hasCapability(NetworkCapabilities.NET_CAPABILITY_NOT_VPN) &&
                it.hasCapability(NetworkCapabilities.NET_CAPABILITY_INTERNET) &&
                it.hasCapability(NetworkCapabilities.NET_CAPABILITY_VALIDATED)
        }
        val next = if (Build.VERSION.SDK_INT >= 31)
            frameworkBest?.takeIf { usable.containsKey(it) }
        else connectivity.activeNetwork?.takeIf { usable.containsKey(it) }
            ?: underlying?.takeIf { usable.containsKey(it) } ?: usable.keys.firstOrNull()
        val old = underlying
        underlying = next
        setUnderlyingNetworks(next?.let { arrayOf(it) } ?: emptyArray())
        val change = when {
            old == null && next != null -> ConnectivityEvent.UP
            old != null && next == null -> ConnectivityEvent.DOWN
            old != next -> ConnectivityEvent.NETWORK_SWITCH
            else -> null
        }
        if (change != null) current?.onConnectivityChange(change)
    }
    internal fun native(action: Connection.() -> Unit) = submit { current?.action() }
    private fun submit(action: () -> Unit) {
        if (destroyed || !::control.isInitialized) return
        control.post {
            if (!destroyed) try { action() } catch (_: Exception) {
                // Do not include exceptions containing credentials/configuration.
                core.connectionManager.failure(FailureKind.Interface, "VPN operation failed")
                clearNative()
            }
        }
    }
    private fun protectDescriptor(fd: Int): Boolean {
        if (!super.protect(fd)) return false
        val network = underlying ?: return false
        return try {
            // fromFd duplicates rather than taking ownership of Rust's socket.
            ParcelFileDescriptor.fromFd(fd).use { network.bindSocket(it.fileDescriptor) }
            true
        } catch (_: Exception) { false }
    }
    private fun establish(network: InterfaceConfig): ParcelFileDescriptor {
        val builder = Builder().setSession("ProtunNext").setMtu(network.mtu)
            .setBlocking(false).addAddress("10.2.0.2", 32).addRoute("10.2.0.1", 32)
            .setUnderlyingNetworks(underlying?.let { arrayOf(it) } ?: emptyArray())
        if (network.ipv6) builder.addAddress("2a07:b944::2:2", 128).addRoute("2a07:b944::2:1", 128)
        network.dns.forEach { builder.addDnsServer(it) }
        builder.addDnsServer("10.2.0.1")
        if (network.ipv6) builder.addDnsServer("2a07:b944::2:1")
        network.routes.forEach { builder.addRoute(it.address, it.length) }
        network.apps?.let { apps ->
            apps.packages.forEach {
                when (apps.mode) {
                    AppMode.Include -> builder.addAllowedApplication(it)
                    AppMode.Exclude -> builder.addDisallowedApplication(it)
                }
            }
        }
        if (Build.VERSION.SDK_INT >= 29) builder.setMetered(false)
        return builder.establish() ?: throw SecurityException("VPN permission missing")
    }
    internal fun connect(next: AndroidConnectionConfig, invalidate: InvalidateRequest?) = submit {
        if (canUpdate(config, next, invalidate)) {
            val tun = if (config!!.network != next.network) establish(next.network).use { it.detachFd() } else null
            val mode = next.tunnel.connectionMode as ConnectionMode.LocalAgent
            current!!.update(ConfigUpdate(next.tunnel.peers, mode.settings, tun?.let { TunStreamInfo.TunFd(it) }))
            if (config!!.tunnel.pcapFile != next.tunnel.pcapFile)
                next.tunnel.pcapFile?.let { current!!.startPacketCapture(it) } ?: current!!.stopPacketCapture()
            config = next
            return@submit
        }
        clearNative()
        val cache = core.cache.forUser(next.userId)
        when (invalidate) {
            InvalidateRequest.All -> cache.clearAll()
            InvalidateRequest.Certificate -> cache.remove(CacheKey.CERTIFICATE)
            null -> {}
        }
        val fd = establish(next.network)
        val epoch = generation.incrementAndGet()
        fd.use {
            current = Connection.unixConnect(
                next.tunnel.copy(networkAvailable = underlying != null), it.detachFd(),
                object : StateChangedCallback {
                    override fun onStateChanged(state: VpnState) {
                        if (generation.get() == epoch) core.connectionManager.nativeState(state)
                    }
                }, object : EventCallback {
                    override fun onEvent(event: Event) {
                        if (generation.get() == epoch) core.connectionManager.event(event)
                    }
                }, object : SocketProtector {
                    override fun protectSocket(socketFd: Int): Boolean = protectDescriptor(socketFd)
                }, cache)
        }
        config = next
    }
    internal fun updateInterface(next: InterfaceConfig) = submit {
        val active = current ?: return@submit
        establish(next).use { active.updateUnixTun(TunStreamInfo.TunFd(it.detachFd())) }
        config = config?.copy(network = next)
    }
    internal fun invalidateCertificate(userId: String) = submit {
        if (config?.userId == userId) current?.invalidateCertificate()
    }
    private fun clearNative() {
        val old = current
        current = null
        config = null
        old?.disconnectAndWait()
        old?.destroy()
        generation.incrementAndGet()
        core.connectionManager.disconnected()
    }
    internal fun disconnect() = submit {
        clearNative()
        stopForeground(STOP_FOREGROUND_REMOVE)
        stopSelf()
    }
    override fun onRevoke() {
        if (::core.isInitialized) core.connectionManager.failure(FailureKind.Revoked)
        disconnect()
    }
    override fun onDestroy() {
        destroyed = true
        if (::connectivity.isInitialized) connectivity.unregisterNetworkCallback(callback)
        if (::control.isInitialized) control.post {
            clearNative()
            thread.quitSafely()
        }
        super.onDestroy()
    }
    companion object { internal const val START = "io.github.rikirsti.protunnext.START" }
}
