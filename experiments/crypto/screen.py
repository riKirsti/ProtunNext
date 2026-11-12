"""Isolated one-CPU primitive preselection, never a download-performance claim."""
import hashlib
import json
import os
import pathlib
import platform
import subprocess

ROOT = pathlib.Path(__file__).resolve().parents[2]
OUT = ROOT / "target/crypto-screen"
CRATE = ROOT / "experiments/crypto"
BIN = CRATE / "target/release/stage5-crypto-screen"


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    OUT.mkdir(parents=True, exist_ok=True)
    cpus = sorted(os.sched_getaffinity(0))
    cpu = cpus[min(2, len(cpus) - 1)]
    os.sched_setaffinity(0, {cpu})
    env = dict(kernel=platform.release(), machine=platform.machine(),
               original_allowed_cpus=cpus, benchmark_cpu=cpu,
               compiler=subprocess.check_output(["rustc", "-vV"], text=True),
               commit=subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=ROOT, text=True).strip(),
               binary_sha256=sha(BIN), manifest_sha256=sha(CRATE / "Cargo.toml"),
               lock_sha256=sha(CRATE / "Cargo.lock"), source_sha256=sha(CRATE / "src/main.rs"),
               scope="primitive-only preselection; not production/session/download performance",
               rustflags=os.environ.get("RUSTFLAGS", ""),
               order="six rounds of ring/rustcrypto/rustcrypto11/aws-lc, alternating forward/reverse",
               milliseconds_per_cell=250)
    (OUT / "environment.json").write_text(json.dumps(env, indent=2))
    (OUT / "hardware.txt").write_text(subprocess.check_output(["lscpu"], text=True))
    (OUT / "dependencies.txt").write_text(subprocess.check_output(
        ["cargo", "tree", "--locked", "--manifest-path", str(CRATE / "Cargo.toml")], text=True))
    rows = []
    with (OUT / "samples.jsonl").open("w") as log:
        for repeat in range(6):
            order = ["ring", "rustcrypto", "rustcrypto11", "aws-lc"]
            if repeat % 2:
                order.reverse()
            for position, backend in enumerate(order):
                raw = subprocess.check_output([str(BIN), backend, "250"], text=True)
                values = [json.loads(line) for line in raw.splitlines()]
                expected = {(size, op) for size in [0, 64, 256, 1280, 1420, 16384] for op in ["open", "seal"]}
                assert len(values) == 12 and {(v["payload_bytes"], v["operation"]) for v in values} == expected
                for value in values:
                    assert value["backend"] == backend and value["verified"]
                    assert value["iterations"] > 0 and value["seconds"] >= 0.25
                    value.update(repeat=repeat, position=position, cpu=cpu)
                    rows.append(value)
                    log.write(json.dumps(value) + "\n")
                log.flush()
    assert len(rows) == 288
    print(f"Retained {len(rows)} verified primitive observations on CPU {cpu}; no download result inferred.")


if __name__ == "__main__":
    main()
