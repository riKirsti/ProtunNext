package io.github.rikirsti.protunnext.android

import io.github.rikirsti.protunnext.generated.ConnectionMode
import io.github.rikirsti.protunnext.generated.InitialConnectionConfig
import java.net.InetAddress

data class IpPrefix(val address: InetAddress, val length: Int) {
    init { require(length in 0..address.address.size * 8) { "IP prefix length" } }
}
enum class AppMode { Include, Exclude }
data class AppFilter(val mode: AppMode, val packages: List<String>)
data class InterfaceConfig(
    val ipv6: Boolean,
    val dns: List<String> = emptyList(),
    val routes: List<IpPrefix> = defaultRoutes(ipv6),
    val apps: AppFilter? = null,
    val mtu: Int = 1420,
) {
    init { require(mtu in 1280..16351) { "MTU outside supported WireGuard TCP frame bounds" } }
    companion object {
        fun defaultRoutes(ipv6: Boolean): List<IpPrefix> = buildList {
            add(IpPrefix(InetAddress.getByAddress(byteArrayOf(0, 0, 0, 0)), 0))
            if (ipv6) add(IpPrefix(InetAddress.getByAddress(ByteArray(16)), 0))
        }
    }
}
/** Native records carry all account/Plus settings without a second conversion layer. */
data class AndroidConnectionConfig(
    val userId: String,
    val tunnel: InitialConnectionConfig,
    val network: InterfaceConfig,
)
enum class InvalidateRequest { Certificate, All }

internal fun canUpdate(current: AndroidConnectionConfig?, next: AndroidConnectionConfig,
    invalidate: InvalidateRequest?): Boolean = current != null && invalidate == null &&
    current.userId == next.userId &&
    current.tunnel.connectionMode is ConnectionMode.LocalAgent &&
    next.tunnel.connectionMode is ConnectionMode.LocalAgent &&
    (current.tunnel.connectionMode as ConnectionMode.LocalAgent).let { old ->
        (next.tunnel.connectionMode as ConnectionMode.LocalAgent).let { new ->
            old.appVersion == new.appVersion && old.userAgent == new.userAgent &&
                old.muonEnv == new.muonEnv && old.certificateDuration == new.certificateDuration &&
                current.tunnel.sniStrategy == next.tunnel.sniStrategy
        }
    }
