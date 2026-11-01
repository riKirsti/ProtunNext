package io.github.rikirsti.protunnext.android

import io.github.rikirsti.protunnext.generated.*
import org.junit.Assert.*
import org.junit.Test
import java.net.InetAddress

class ConfigurationTest {
    private fun config(user: String = "test-user", environment: MuonEnv = MuonEnv.Prod): AndroidConnectionConfig =
        AndroidConnectionConfig(user, InitialConnectionConfig(emptyList(), false, null,
            ConnectionMode.LocalAgent("public test agent", "windows-vpn@4.1.0",
                LocalAgentSettings(null, null, null, null, null, null), environment, null),
            SniStrategy.RANDOM), InterfaceConfig(ipv6 = true))
    @Test fun accountBoundaryAndEnvironmentChangesForceNewNativeSession() {
        val current = config()
        assertTrue(canUpdate(current, config(), null))
        assertFalse(canUpdate(current, config("other-user"), null))
        assertFalse(canUpdate(current, config(environment = MuonEnv.Atlas("public-test")), null))
        assertFalse(canUpdate(current, config(), InvalidateRequest.Certificate))
        assertFalse(canUpdate(current, config(), InvalidateRequest.All))
        assertFalse(canUpdate(null, config(), null))
    }
    @Test fun keyOnlyModeAndChangedNativeIdentityConfigurationForceRestart() {
        val current = config()
        val next = config()
        next.tunnel.connectionMode = ConnectionMode.NoLocalAgent(ByteArray(32) { 7 })
        assertFalse(canUpdate(current, next, null))
        next.tunnel.connectionMode = (current.tunnel.connectionMode as ConnectionMode.LocalAgent)
            .copy(certificateDuration = CertificateDuration.Minutes(90u))
        assertFalse(canUpdate(current, next, null))
        next.tunnel.connectionMode = current.tunnel.connectionMode
        next.tunnel.sniStrategy = SniStrategy.TOP
        assertFalse(canUpdate(current, next, null))
    }
    @Test fun interfaceValidationRejectsInvalidFamilyPrefixesAndMtu() {
        assertThrows(IllegalArgumentException::class.java) {
            IpPrefix(InetAddress.getByAddress(byteArrayOf(10, 2, 0, 1)), 33)
        }
        assertThrows(IllegalArgumentException::class.java) { InterfaceConfig(false, mtu = 0) }
        assertEquals(listOf(4, 16), InterfaceConfig.defaultRoutes(true).map { it.address.address.size })
        assertEquals(listOf(0, 0), InterfaceConfig.defaultRoutes(true).map { it.length })
    }
}
