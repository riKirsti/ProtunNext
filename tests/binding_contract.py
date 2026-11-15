"""Public generated-language surface, checked before invoking native contracts."""
import inspect


def check_contract(api):
    methods = {
        "update_peers": ["self", "peers"],
        "update": ["self", "update"],
        "on_connectivity_change": ["self", "event"],
        "disconnect": ["self"],
        "disconnect_and_wait": ["self"],
        "start_packet_capture": ["self", "pcap_file"],
        "stop_packet_capture": ["self"],
        "request_stats": ["self"],
        "debug_log": ["self"],
        "update_local_agent_settings": ["self", "settings"],
        "request_local_agent_stats": ["self"],
        "provide_api_fork_selector": ["self", "fork_selector_info"],
        "invalidate_certificate": ["self"],
    }
    for name, arguments in methods.items():
        method = getattr(api.Connection, name)
        assert list(inspect.signature(method).parameters) == arguments, name
    records = {
        "InitialConnectionConfig": ["peers", "network_available", "pcap_file", "connection_mode", "sni_strategy"],
        "PeerInfo": ["peer_id", "server_ip", "server_public_key", "udp_ports", "tcp_ports",
                     "tls_ports", "priority", "exit_label", "server_name"],
        "ConfigUpdate": ["peers", "settings", "tun"],
        "LocalAgentSettings": ["split_tcp", "netshield_level", "soft_jail",
                               "port_forwarding", "random_nat", "circumvention_routing"],
        "ForkSelectorInfo": ["selector", "cookies"],
        "Cookies": ["items"],
        "Cookie": ["name", "value"],
        "PcapFileInfo": ["file", "max_bytes"],
        "VpnState": ["interface_state", "connection_state"],
    }
    for name, arguments in records.items():
        assert list(inspect.signature(getattr(api, name)).parameters) == arguments, name
    for name, methods in {
        "PersistentCache": ["put", "get", "remove", "clear_all"],
        "SocketProtector": ["protect_socket"],
        "StateChangedCallback": ["on_state_changed"],
        "EventCallback": ["on_event"],
        "ClientLogger": ["log"],
    }.items():
        for method in methods:
            assert callable(getattr(getattr(api, name), method)), f"{name}.{method}"
    print("Generated v3.0.5 connection, record and callback contracts passed")
