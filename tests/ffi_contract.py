"""Exercise generated bindings against the real native library, without networking."""
import importlib
import pathlib
import sys
import threading

sys.path.insert(0, str(pathlib.Path(sys.argv[1]).resolve()))
api = importlib.import_module("protun_next")


class Cache:
    def __init__(self):
        self.values = {}

    def put(self, key, data):
        self.values[key] = data

    def get(self, key):
        return self.values.get(key)

    def remove(self, key):
        self.values.pop(key, None)

    def clear_all(self):
        self.values.clear()


class States:
    def __init__(self):
        self.values = []
        self.changed = threading.Event()

    def on_state_changed(self, state):
        self.values.append(state)
        self.changed.set()


class Events:
    def on_event(self, event):
        raise AssertionError(f"Unexpected event with no network/capture: {event}")


states, cache = States(), Cache()
config = api.InitialConnectionConfig(
    peers=[], network_available=False, pcap_file=None,
    connection_mode=api.ConnectionMode.NO_LOCAL_AGENT(wg_private_key=None),
    sni_strategy=api.SniStrategy.RANDOM,
)
connection = api.Connection.unix_connect(config, None, states, Events(), None, cache)
assert states.changed.wait(3), "state callback did not cross the FFI boundary"
assert states.values[-1].connection_state.is_CONNECTING()
assert cache.values, "identity persistence callback was not invoked"
connection.update_unix_tun(api.TunStreamInfo.NO_TUN())
options = api.Options(udp_backend=api.UdpBackend.BATCH, udp_batch_size=16,
                      udp_offload=False, unbuffered_tls=True, compact_pool=True,
                      in_place_crypto=False)
assert connection.update_performance_options(options)
options.udp_batch_size = 0
assert not connection.update_performance_options(options)
connection.disconnect_and_wait()
connection.disconnect_and_wait()
assert states.values[-1].connection_state.is_DISCONNECTED()
print("Generated binding constructor, identity/cache, state callbacks and shutdown passed")
