"""Measure verified kernel TCP downloads through the normal release Linux worker."""

from contextlib import contextmanager
import hashlib
import json
import math
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile


ROOT = Path(__file__).resolve().parent.parent
FIXTURE = ROOT / "tests/fixtures/download_probe.rs"
TRANSPORTS = ("udp", "tcp", "stealth")
METRICS = {"download_mbps": 1, "cpu_ns_per_payload_byte": -1, "peak_rss_bytes": -1}


@contextmanager
def overlay(work):
    manifest = work / "Cargo.toml"
    library = work / "src/lib.rs"
    fixture = work / "src/qualification_download_probe.rs"
    entry = work / "src/bin/qualification-download.rs"
    original = {path: path.read_bytes() for path in (manifest, library)}
    if fixture.exists() or entry.exists():
        raise ValueError("download overlay would overwrite a tracked file")
    try:
        fixture.write_bytes(FIXTURE.read_bytes())
        library.write_bytes(original[library] + b"\n#[doc(hidden)]\npub mod qualification_download_probe;\n")
        entry.write_text("fn main() { protun_next::qualification_download_probe::run(); }\n", encoding="utf-8")
        manifest.write_bytes(original[manifest] + b'\n[[bin]]\nname = "qualification-download"\n'
                             b'path = "src/bin/qualification-download.rs"\n')
        yield
    finally:
        for path, data in original.items():
            path.write_bytes(data)
        for path in (fixture, entry):
            path.unlink(missing_ok=True)


def build(work, target):
    tree = subprocess.check_output(["git", "write-tree"], cwd=work, text=True).strip()
    identity = hashlib.sha256(tree.encode() + Path(__file__).read_bytes() + FIXTURE.read_bytes()).hexdigest()
    with overlay(work):
        subprocess.run(["cargo", "build", "--release", "--locked", "--offline", "--bin",
                        "qualification-download", "--target-dir", str(target)],
                       cwd=work, env={**os.environ, "NEXT_BUILD_ID": identity}, check=True, timeout=900)
    executable = target / "release/qualification-download"
    return executable, identity, hashlib.sha256(executable.read_bytes()).hexdigest()


def positive(value):
    return type(value) in (float, int) and math.isfinite(value) and value > 0


def metrics(sample, client, baseline, protocol, family, app, flows, blocks):
    peer, load = sample["peer"], sample["load"]
    expected_bytes = 65536 * blocks * flows
    if (sample["role"] != "download-client" or sample["effective_sources_sha256"] != client
            or any(item["effective_sources_sha256"] != baseline for item in (peer, load))
            or (sample["protocol"], sample["family"], sample["application"], sample["flows"],
                sample["blocks_per_flow"]) != (protocol, family, app, flows, blocks)
            or load["verified"] is not True or load["bytes"] != expected_bytes
            or not sample["namespace"] == load["namespace"] == peer["outer_namespace"] != peer["namespace"]
            or peer["handshakes"] < 1
            or sample["before"]["state"] != "TunnelEstablished"
            or sample["after"]["state"] != "TunnelEstablished"):
        raise ValueError("download identity, byte count, routing or handshake check failed")
    counters = sample["counters"]
    if (any(counters[name] for name in ("invalid_packets", "outer_io_errors", "outer_eof"))
            or min(counters["decrypted_packets"], counters["tun_reads"]) <= 0):
        raise ValueError("invalid production tunnel counters")
    if protocol == "stealth" and peer["tls"] != "Some(TLSv1_3)":
        raise ValueError("Stealth did not negotiate TLS 1.3")
    duration, cpu, load_time = sample["seconds"], sample["client_cpu_seconds"], load["seconds"]
    if not all(positive(value) for value in (duration, cpu, load_time, load["cpu_seconds"],
               peer["seconds"], sample["client_rss_kib"])):
        raise ValueError("invalid download process measurements")
    auxiliary = max(load["cpu_seconds"] / load_time, peer["wire_cpu_seconds"] / peer["seconds"],
                    peer["server_cpu_seconds"] / peer["seconds"])
    if not positive(auxiliary):
        raise ValueError("invalid auxiliary CPU accounting")
    return {"download_mbps": expected_bytes * 8 / load_time / 1e6,
            "cpu_ns_per_payload_byte": cpu * 1e9 / expected_bytes,
            "cpu_percent_one_core": 100 * cpu / duration,
            "peak_rss_bytes": sample["client_rss_kib"] * 1024,
            "seconds": load_time, "verified_payload_bytes": expected_bytes,
            "interval_skew": abs(duration / load_time - 1), "auxiliary_core_fraction": auxiliary}


def sample(build_info, baseline, protocol, family, app, flows, blocks):
    cpus = sorted(os.sched_getaffinity(0))
    if len(cpus) < 4:
        raise RuntimeError("download qualification needs four isolated process CPU assignments")
    executable, identity, _ = build_info
    peer, fixed_identity, _ = baseline
    assignments = dict(zip(("PEER", "LOAD", "CLIENT", "SERVER"), cpus[:4]))
    if os.geteuid() == 0:
        raise RuntimeError("download qualification must start as an unprivileged user")
    affinity = [argument for name, cpu in assignments.items()
                for argument in ("--setenv", f"NEXT_{name}_CPUS", str(cpu))]
    run = subprocess.run(
        ["/usr/bin/bwrap", "--unshare-user", "--uid", "0", "--gid", "0",
         "--unshare-net", "--unshare-pid", "--unshare-ipc", "--unshare-uts",
         "--cap-add", "CAP_NET_ADMIN", "--cap-add", "CAP_SYS_ADMIN",
         "--new-session", "--die-with-parent", "--clearenv",
         "--setenv", "PATH", "/usr/bin:/usr/sbin",
         "--ro-bind", "/usr", "/usr", "--ro-bind", "/lib", "/lib",
         "--ro-bind-try", "/lib64", "/lib64",
         "--proc", "/proc", "--dev", "/dev", "--dev-bind", "/dev/net/tun", "/dev/net/tun",
         "--tmpfs", "/tmp", "--chdir", "/tmp",
         "--ro-bind", str(executable), "/candidate", "--ro-bind", str(peer), "/baseline",
         *affinity, "/usr/bin/unshare", "--net", "/candidate",
         "download-client", "/baseline", protocol, str(flows), str(blocks), family, app],
        stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True, timeout=180,
    )
    if run.returncode:
        error = run.stderr.splitlines()[:1]
        raise RuntimeError(f"isolated download process exited {run.returncode}: {error}")
    rows = [json.loads(line.removeprefix("NEXT_JSON ")) for line in run.stdout.splitlines()
            if line.startswith("NEXT_JSON ")]
    if len(rows) != 1:
        raise ValueError("missing or duplicate download result")
    return metrics(rows[0], identity, fixed_identity, protocol, family, app, flows, blocks)


def smoke():
    subprocess.run(["cargo", "fetch", "--locked"], cwd=ROOT, check=True)
    with tempfile.TemporaryDirectory(dir=os.environ.get("RUNNER_TEMP")) as directory:
        path = Path(directory)
        work = path / "source"
        subprocess.run(["git", "worktree", "add", "--detach", str(work), "HEAD"], cwd=ROOT, check=True)
        try:
            binary = build(work, path / "target")
            for protocol in TRANSPORTS:
                for family, app, flows in (("v4", "http", 1), ("v6", "https", 8)):
                    blocks = 32768 // flows if "--sustained" in sys.argv[1:] else 64
                    row = sample(binary, binary, protocol, family, app, flows, blocks)
                    print(protocol, family, app, flows, json.dumps(row), flush=True)
        finally:
            subprocess.run(["git", "worktree", "remove", "--force", str(work)], cwd=ROOT, check=True)


if __name__ == "__main__":
    smoke()
