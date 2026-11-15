package io.github.rikirsti.protunnext.android

import android.net.VpnService
import android.os.ParcelFileDescriptor

/** Complete the host-owned builder; descriptor transfer remains in the service. */
internal fun establishInterface(builder: VpnService.Builder, network: InterfaceConfig): ParcelFileDescriptor {
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
    builder.setMetered(false)
    return builder.establish() ?: throw SecurityException("VPN permission missing")
}
