"""Compile frozen public contracts from independent consumers without patches.

Run after fetching/building the root lockfile: consumer resolution is offline.
"""
import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile

from compatibility import ROOT, validate_checkout

upstream = validate_checkout()
root = ROOT
output = root / 'target'
output.mkdir(exist_ok=True)
env = os.environ.copy()
env.setdefault('CARGO_TARGET_DIR', str(output))
for mode, options in [
    ('default', ''),
    ('core', 'default-features = false,'),
    ('unix', 'default-features = false, features = ["unix"],'),
]:
    with tempfile.TemporaryDirectory(prefix='downstream-', dir=output) as folder:
        consumer = Path(folder)
        (consumer / 'src').mkdir()
        (consumer / '.cargo').mkdir()
        shutil.copyfile(root / '.cargo/config.toml', consumer / '.cargo/config.toml')
        (consumer / 'Cargo.toml').write_text(
            '[package]\nname = "protun-next-consumer-check"\nversion = "0.0.0"\nedition = "2024"\n'
            '[workspace]\n[dependencies]\nprotun-next = { path = '
            + json.dumps(str(root)) + ', ' + options + ' }\n', encoding='utf-8'
        )
        shutil.copyfile(root / 'tests/contracts/api.rs', consumer / 'src/contracts.rs')
        (consumer / 'src/main.rs').write_text('mod contracts;\nfn main() {}\n', encoding='utf-8')
        subprocess.run([os.environ.get('CARGO', 'cargo'), 'check', '--offline', '--manifest-path',
                        str(consumer / 'Cargo.toml')], cwd=consumer, env=env, check=True)
        print(f"External {upstream['tag']}/public API contracts compiled: {mode}", flush=True)
