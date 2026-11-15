package io.github.rikirsti.protunnext.android

import android.content.*
import android.net.VpnService
import android.os.Handler
import android.os.IBinder
import android.os.Looper
import io.github.rikirsti.protunnext.generated.*
import kotlinx.coroutines.channels.BufferOverflow
import kotlinx.coroutines.flow.*

/** Control-only Android wrapper; all packet I/O stays in the native worker. */
class ConnectionManager internal constructor(private val context: Context) {
    private val main = Handler(Looper.getMainLooper())
    private var bound = false
    private var service: ProtunVpnService? = null
    private val pending = ArrayDeque<(ProtunVpnService) -> Unit>()
    private val mutableState = MutableStateFlow(AndroidStatus())
    val state: StateFlow<AndroidStatus> = mutableState.asStateFlow()
    private val mutableEvents = MutableSharedFlow<Event>(extraBufferCapacity = 128,
        onBufferOverflow = BufferOverflow.DROP_OLDEST)
    val events: SharedFlow<Event> = mutableEvents.asSharedFlow()
    val connectionStats: Flow<Event.ConnectionStats> = events.filterIsInstance()
    val localAgentStats: Flow<Event.LocalAgentStats> = events.filterIsInstance()
    private val mutableFailures = MutableSharedFlow<AndroidFailure>(extraBufferCapacity = 16,
        onBufferOverflow = BufferOverflow.DROP_OLDEST)
    val failures: SharedFlow<AndroidFailure> = mutableFailures.asSharedFlow()
    internal fun nativeState(state: VpnState) { mutableState.update { it.copy(vpn = state) } }
    internal fun environment(alwaysOn: Boolean, lockdown: Boolean) {
        mutableState.update { it.copy(alwaysOn = alwaysOn, lockdown = lockdown) }
    }
    internal fun event(event: Event) { mutableEvents.tryEmit(event) }
    internal fun failure(kind: FailureKind, message: String? = null) {
        mutableFailures.tryEmit(AndroidFailure(kind, message))
    }
    internal fun disconnected() { mutableState.update { it.copy(vpn = null) } }
    private val binding = object : ServiceConnection {
        override fun onServiceConnected(name: ComponentName, binder: IBinder) {
            service = (binder as ProtunVpnService.LocalBinder).service
            while (pending.isNotEmpty()) pending.removeFirst().invoke(service!!)
        }
        override fun onServiceDisconnected(name: ComponentName) {
            service = null
            disconnected()
        }
        override fun onBindingDied(name: ComponentName) {
            unbind()
            pending.clear()
            disconnected()
            failure(FailureKind.Service, "VPN service binding died")
        }
        override fun onNullBinding(name: ComponentName) {
            unbind()
            pending.clear()
            failure(FailureKind.Service, "VPN service unavailable")
        }
    }
    fun prepare(): Intent? = VpnService.prepare(context)
    fun connect(config: AndroidConnectionConfig, invalidate: InvalidateRequest? = null) = main.post {
        if (prepare() != null) {
            failure(FailureKind.PermissionMissing)
            return@post
        }
        try {
            val intent = Intent(context, ProtunVpnService::class.java).setAction(ProtunVpnService.START)
            context.startForegroundService(intent)
            if (!bound) {
                bound = context.bindService(intent, binding, Context.BIND_AUTO_CREATE)
                if (!bound) {
                    failure(FailureKind.Service, "Cannot bind VPN service")
                    return@post
                }
            }
            dispatch { it.connect(config, invalidate) }
        } catch (_: RuntimeException) {
            pending.clear()
            unbind()
            failure(FailureKind.Service, "Cannot start VPN service")
        }
    }
    private fun dispatch(action: (ProtunVpnService) -> Unit) {
        val current = service
        if (current != null) action(current)
        else if (bound && pending.size < 64) pending.addLast(action)
        else failure(if (bound) FailureKind.QueueFull else FailureKind.Service)
    }
    fun updateInterfaceConfig(config: InterfaceConfig) = main.post { dispatch { it.updateInterface(config) } }
    fun updatePeers(peers: List<PeerInfo>) = main.post { dispatch { it.native { updatePeers(peers) } } }
    fun updateLocalAgentSettings(settings: LocalAgentSettings) = main.post {
        dispatch { it.native { updateLocalAgentSettings(settings) } }
    }
    fun updateApiSelector(selector: ForkSelectorInfo) = main.post {
        dispatch { it.native { provideApiForkSelector(selector) } }
    }
    fun setPacketCaptureEnabled(info: PcapFileInfo?) = main.post {
        dispatch { it.native { if (info == null) stopPacketCapture() else startPacketCapture(info) } }
    }
    fun invalidateCertificate(userId: String) = main.post {
        ProtunNextCore.instance!!.clearCertificate(userId)
        if (bound) dispatch { it.invalidateCertificate(userId) }
    }
    fun requestStats() = main.post { dispatch { it.native { requestStats() } } }
    fun requestLocalAgentStats() = main.post { dispatch { it.native { requestLocalAgentStats() } } }
    fun debugLog() = main.post { dispatch { it.native { debugLog() } } }
    fun disconnect() = main.post {
        pending.clear()
        service?.disconnect()
        if (service == null) context.stopService(Intent(context, ProtunVpnService::class.java))
        unbind()
        disconnected()
    }
    private fun unbind() {
        if (bound) context.unbindService(binding)
        bound = false
        service = null
    }
}
