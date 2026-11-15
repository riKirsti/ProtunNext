"""Measure the existing framing benchmark, not VPN download performance.

Whole-process CPU and lifetime peak RSS include startup/teardown. Reports exclude
hostnames, usernames, executable paths, environment dumps and raw stdout.
"""
import argparse
import ctypes
import hashlib
import json
import math
import os
from pathlib import Path
import platform
import statistics
import subprocess
import tempfile
import time

ROOT = Path(__file__).resolve().parent.parent


def measure(executable):
    with tempfile.TemporaryFile() as output:
        started = time.perf_counter()
        process = subprocess.Popen([str(executable)], stdout=output, stderr=subprocess.DEVNULL)
        if os.name == "nt":
            from ctypes import wintypes as w

            class Memory(ctypes.Structure):
                _fields_ = [("cb", w.DWORD), ("faults", w.DWORD)] + [
                    (name, ctypes.c_size_t) for name in (
                        "peak_rss", "rss", "peak_paged", "paged", "peak_nonpaged",
                        "nonpaged", "pagefile", "peak_pagefile")]

            kernel = ctypes.WinDLL("kernel32", use_last_error=True)
            psapi = ctypes.WinDLL("psapi", use_last_error=True)
            kernel.OpenProcess.argtypes = [w.DWORD, w.BOOL, w.DWORD]
            kernel.OpenProcess.restype = w.HANDLE
            kernel.GetProcessTimes.argtypes = [w.HANDLE] + [ctypes.POINTER(w.FILETIME)] * 4
            kernel.CloseHandle.argtypes = [w.HANDLE]
            psapi.GetProcessMemoryInfo.argtypes = [w.HANDLE, ctypes.POINTER(Memory), w.DWORD]
            handle = kernel.OpenProcess(0x0400 | 0x0010, False, process.pid)
            if not handle:
                process.wait()
                raise ctypes.WinError(ctypes.get_last_error())
            try:
                process.wait(timeout=30)
                stamps = [w.FILETIME() for _ in range(4)]
                memory = Memory()
                memory.cb = ctypes.sizeof(memory)
                if not kernel.GetProcessTimes(handle, *(ctypes.byref(s) for s in stamps)):
                    raise ctypes.WinError(ctypes.get_last_error())
                if not psapi.GetProcessMemoryInfo(handle, ctypes.byref(memory), memory.cb):
                    raise ctypes.WinError(ctypes.get_last_error())
                cpu = sum((s.dwHighDateTime << 32) | s.dwLowDateTime for s in stamps[2:]) / 1e7
                rss = memory.peak_rss
            finally:
                kernel.CloseHandle(handle)
        else:
            # Per-child counters, not cumulative resource.getrusage(CHILDREN).
            _, status, usage = os.wait4(process.pid, 0)
            process.returncode = os.waitstatus_to_exitcode(status)
            cpu = usage.ru_utime + usage.ru_stime
            rss = usage.ru_maxrss * (1 if platform.system() == "Darwin" else 1024)
        elapsed = time.perf_counter() - started
        if process.returncode:
            raise RuntimeError("benchmark failed; no successful sample recorded")
        output.seek(0)
        record = json.load(output)
    return metrics(record, cpu, rss, elapsed)


def metrics(record, cpu, rss, elapsed):
    if record.get("benchmark") != "actual-frame-roundtrip":
        raise ValueError("unexpected benchmark scope")
    for name in ("bytes", "packets", "seconds"):
        value = record.get(name)
        if not isinstance(value, (int, float)) or isinstance(value, bool) or not math.isfinite(value) or value <= 0:
            raise ValueError("invalid benchmark counters")
    if record["bytes"] != 1280 or int(record["packets"]) != record["packets"]:
        raise ValueError("unexpected payload or fractional packet count")
    if not all(math.isfinite(x) for x in (cpu, rss, elapsed)) or cpu <= 0 or rss <= 0 or elapsed <= 0:
        raise ValueError("invalid process measurements")
    if record["seconds"] > elapsed * 1.05:
        raise ValueError("benchmark duration exceeds process lifetime")
    total = record["bytes"] * record["packets"]
    return {"packets": record["packets"], "loop_seconds": record["seconds"],
            "process_wall_seconds": elapsed, "process_cpu_seconds": cpu,
            "cpu_percent_one_core": cpu / elapsed * 100, "peak_rss_bytes": rss,
            "cpu_ns_per_payload_byte": cpu * 1e9 / total,
            "framing_payload_mbps": total * 8 / record["seconds"] / 1e6}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("executable", type=Path, nargs="?")
    parser.add_argument("--build", action="store_true", help="build release library and link the framing driver")
    parser.add_argument("--output", required=True, type=Path)
    parser.add_argument("--samples", type=int, default=7)
    args = parser.parse_args()
    if not 3 <= args.samples <= 30:
        parser.error("use 3..30 repeated samples")
    if bool(args.executable) == args.build:
        parser.error("supply an executable or --build, not both")
    if args.build:
        built = subprocess.run([os.environ.get("CARGO", "cargo"), "build", "--locked",
                                "--release", "--lib", "--message-format=json"],
                               cwd=ROOT, check=True, stdout=subprocess.PIPE, text=True)
        native_paths = sorted({path for line in built.stdout.splitlines()
                               for item in [json.loads(line)] if item.get("reason") == "build-script-executed"
                               for path in item.get("linked_paths", [])})
        target = Path(os.environ.get("CARGO_TARGET_DIR", ROOT / "target")).resolve()
        destination = target / "baseline" / ("packet-path.exe" if os.name == "nt" else "packet-path")
        destination.parent.mkdir(parents=True, exist_ok=True)
        # Link the unchanged harness to the ordinary release rlib. Cargo's bench
        # profile mixes panic strategies for this rlib/cdylib/staticlib package.
        command = [os.environ.get("RUSTC", "rustc"), "--edition=2024", "--crate-name", "packet_path",
                        str(ROOT / "benches/packet_path.rs"), "--extern",
                        f"protun_next={target / 'release/libprotun_next.rlib'}",
                        "-L", f"dependency={target / 'release/deps'}",
                        "-C", "opt-level=3", "-C", "lto=thin", "-C", "codegen-units=1",
                   "-C", "panic=abort", "-o", str(destination)]
        for path in native_paths:
            command.extend(["-L", path])
        subprocess.run(command, cwd=ROOT, check=True)
        args.executable = destination
    binary = args.executable.resolve(strict=True)
    measure(binary)  # Separate-process warmup, excluded from saved samples.
    samples = [measure(binary) for _ in range(args.samples)]
    summary = {}
    for name in ("cpu_percent_one_core", "peak_rss_bytes", "cpu_ns_per_payload_byte", "framing_payload_mbps"):
        values = [sample[name] for sample in samples]
        summary[name] = {"mean": statistics.mean(values), "median": statistics.median(values),
                         "stdev": statistics.stdev(values), "min": min(values), "max": max(values)}
    result = {
        "schema_version": 1, "upstream": "Protun v3.0.5",
        "upstream_commit": "a498c38ae6ffb0e68e7190b2831ef985d73fa6da",
        "commit": subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=ROOT, text=True).strip(),
        "tracked_changes": bool(subprocess.check_output(["git", "status", "--porcelain", "--untracked-files=no"], cwd=ROOT)),
        "system": platform.system(), "architecture": platform.machine(),
        "binary_sha256": hashlib.sha256(binary.read_bytes()).hexdigest(), "binary_bytes": binary.stat().st_size,
        "lockfile_sha256": hashlib.sha256((ROOT / "Cargo.lock").read_bytes()).hexdigest(),
        "compiler": subprocess.check_output(["rustc", "-vV"], text=True).strip(),
        "build_contract": ("default-feature cargo build --locked --release --lib; rustc framing harness "
                           "linked to that rlib with opt-level=3, thin LTO, codegen-units=1, panic=abort"
                           if args.build else "caller-supplied executable; build configuration not attested"),
        "scope": "host framing and buffer recycling; no encryption, sockets, TUN or VPN download",
        "cpu_scope": "all process threads; 100 percent is one core; includes startup and teardown",
        "memory_scope": "OS lifetime peak working set/RSS; excludes kernel and other processes",
        "comparison_policy": "observational baseline; host variability is not a universal VPN regression threshold",
        "samples": samples, "summary": summary,
    }
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(result, indent=2) + "\n", encoding="utf-8")
    print(json.dumps({"scope": result["scope"], "summary": summary}))


if __name__ == "__main__":
    main()
