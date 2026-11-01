"""Call Windows native metadata/initialization without activating an adapter."""
import sys
sys.path.insert(0, sys.argv[1])
import protun_next as native

class Logger:
    def log(self, level, message):
        pass

root = native.ProTun.initialize(native.LogLevel.INFO, Logger())
configuration = native.NetworkConfig(
    tun_adapter=native.AdapterConfig(
        custom_dns_server_ips=['9.9.9.9'], is_ipv6_enabled=False,
        mtu=1420, buffer_size_bytes=1048576),
    udp_socket=native.SocketConfig(send_buffer_size_bytes=0, receive_buffer_size_bytes=0))
assert configuration.tun_adapter.custom_dns_server_ips == ['9.9.9.9']
assert hasattr(native.WindowsConnection, 'connect')
assert hasattr(native.WindowsConnection, 'get_connection')
assert hasattr(native.WindowsConnection, 'set_dns')
assert hasattr(native.WindowsConnection, 'set_ipv6')
# No owned adapters/routes exist in this fresh process; this must be a no-op.
root.delete_routes()
print('Windows native initialization and generated contracts passed; no VPN adapter activated')
