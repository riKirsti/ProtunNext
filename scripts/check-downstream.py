"""Compile the production API from a separate Cargo application, without patches.

Run after fetching/building the root lockfile: dependency resolution is offline.
The standalone lockfile is a consumer-compatibility check, not a performance build.
"""
import json
import os
from pathlib import Path
import subprocess
import tempfile

root = Path(__file__).resolve().parent.parent
output = root / "target"
output.mkdir(exist_ok=True)
consumer = Path(tempfile.mkdtemp(prefix="downstream-", dir=output))
(consumer / "src").mkdir()
(consumer / ".cargo").mkdir()
(consumer / ".cargo/config.toml").write_text(
    '[registries.proton]\nindex = "sparse+https://rust-registry.proton.me/index/"\n'
)
(consumer / "Cargo.toml").write_text(
    '[package]\nname = "protun-next-consumer-check"\nversion = "0.0.0"\nedition = "2024"\n'
    '[workspace]\n[dependencies]\nprotun-next = { path = '
    + json.dumps(str(root)) + ', features = [] }\n'
)
(consumer / "src/main.rs").write_text(
    'fn main() {\n'
    ' let _transport = protun_next::transport::Transport::Udp;\n'
    ' let connection: protun_next::tls::ClientConnection = protun_next::tls::client("example.com").unwrap();\n'
    ' let _tls = protun_next::transport::Transport::Tls(Box::new(connection));\n'
    ' let _key = protun_next::session::Session::public_key([7;32]);\n}\n'
)
env = os.environ.copy()
env.setdefault("CARGO_TARGET_DIR", str(output))
subprocess.run([os.environ.get("CARGO", "cargo"), "check", "--offline", "--manifest-path",
                str(consumer / "Cargo.toml")], cwd=consumer, env=env, check=True)
print("Downstream production API compiled without workspace patches")
