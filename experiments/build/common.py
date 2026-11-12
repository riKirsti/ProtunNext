"""Build provenance helpers; no production runtime changes."""
import hashlib
import json
import pathlib


def sha(path):
    return hashlib.sha256(pathlib.Path(path).read_bytes()).hexdigest()


def cpu_identity():
    keys = {'model name', 'vendor_id', 'cpu family', 'model', 'stepping', 'flags',
            'features', 'cpu implementer', 'cpu architecture', 'cpu part',
            'cpu variant', 'cpu revision'}
    values = set()
    for line in pathlib.Path('/proc/cpuinfo').read_text().splitlines():
        if ':' in line:
            key, value = line.split(':', 1)
            if key.strip().lower() in keys:
                values.add((key.strip().lower(), ' '.join(value.split())))
    assert values, 'missing actual CPU identity'
    details = sorted(values)
    return dict(sha256=hashlib.sha256(json.dumps(details).encode()).hexdigest(), details=details)
