"""Collect hosted benchmark evidence from trusted, fixed input."""

import hashlib
import json
from pathlib import Path
import sys
import tempfile

import maintenance


def seed():
    head = maintenance.hosted.git("rev-parse", "HEAD").decode().strip()
    guidance = maintenance.hosted.git("show", f"{head}:AGENTS.md")
    out = maintenance.data_dir("maintenance-proposals")
    out.mkdir()
    with tempfile.TemporaryDirectory(dir=out.parent) as directory:
        with maintenance.checkout(Path(directory), "fixture") as work:
            path = work / "src/linux_batch.rs"
            with path.open("a", encoding="utf-8") as source:
                source.write("\n// Fixed benchmark replay control.\n")
            patch = maintenance.hosted.git("diff", "--", "src/linux_batch.rs")
    (out / "performance.patch").write_bytes(patch)
    report = {"commit": head, "guidance_sha256": hashlib.sha256(guidance).hexdigest(),
              "stages": {"performance": {
                  "notes": ("Platform: linux-arm64\nWorkload: udp/v4/http/1\nMetric: download_mbps\n"
                            "Hypothesis: An unchanged runtime should show no repeatable gain.\n"),
                  "sha256": hashlib.sha256(patch).hexdigest(), "status": "candidate"}}}
    (out / "report.json").write_text(json.dumps(report), encoding="utf-8")


def check_replay():
    path = maintenance.data_dir("maintenance-proposals") / "report.json"
    result = json.loads(path.read_text(encoding="utf-8"))["stages"]["performance"]
    if (result.get("screen", {}).get("workload") != "udp/v4/http/1"
            or "sha256" in result
            or result["status"] not in {"screen: no demonstrated download gain", "screen: measurement inconclusive"}):
        raise ValueError(f"Fixed Linux replay did not reach a held download result: {result['status']}")
    print(result["status"])


def calibrate():
    binary, lock_sha = maintenance.framing.build_executable(
        maintenance.ROOT, maintenance.data_dir("macos-calibration-target"))
    evidence = {}
    for seconds in (1, 3):
        def measure(path):
            return maintenance.framing.measure(path, seconds=seconds)

        controls, pairs = maintenance.paired_samples(measure, binary, binary)
        evidence[str(seconds)] = maintenance.compare_samples(controls, pairs, maintenance.METRICS)
    head = maintenance.hosted.git("rev-parse", "HEAD").decode().strip()
    report = {"commit": head, "binary_sha256": hashlib.sha256(binary.read_bytes()).hexdigest(),
              "consumer_lock_sha256": lock_sha, "same_binary_at_seconds": evidence}
    output = maintenance.data_dir("macos-benchmark-calibration.json")
    output.write_text(json.dumps(report, indent=2), encoding="utf-8")
    print(json.dumps(report, indent=2))


if __name__ == "__main__":
    {"seed": seed, "check-replay": check_replay, "calibrate": calibrate}[sys.argv[1]]()
