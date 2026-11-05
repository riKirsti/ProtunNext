"""Verify the narrow patch surface against original registry archive file hashes."""
import hashlib
import json
from pathlib import Path, PurePosixPath

root = Path(__file__).resolve().parent.parent / "vendor/proton-boringtun"
manifest = json.loads((root / "UPSTREAM.json").read_text())
assert manifest["archive_sha256"] == "0c10f0be52e40d51a36de1aa072bbd417b59042432c852fc3657a8772d678b34"
allowed = {"Cargo.toml", "src/noise/mod.rs", "src/noise/session.rs"}
changed = set()
for name, expected in manifest["files"].items():
    path = PurePosixPath(name)
    assert not path.is_absolute() and ".." not in path.parts
    actual = hashlib.sha256((root / path).read_bytes()).hexdigest()
    if actual != expected:
        changed.add(name)
assert changed == allowed, f"unexpected vendor changes: {changed ^ allowed}"
print(f"Verified {len(manifest['files'])} upstream file hashes; only the three declared extension files differ")
