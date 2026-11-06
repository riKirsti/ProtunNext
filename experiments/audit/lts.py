"""Run the erasure-only plan in an actual supported kernel; analyze every run."""
import argparse
import hashlib
import json
import os
import pathlib
import shutil
import socket
import subprocess
import sys
import time

from analyze import publish
from run import guest_script, plan

ROOT = pathlib.Path(__file__).resolve().parents[2]


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def collect(log, output, expected=None):
    rows, row = [], None
    for line in log.splitlines():
        if line.startswith("ERASURE_CASE "):
            if row is not None:
                rows.append(row)
            row = json.loads(line[len("ERASURE_CASE "):])
            row.update(exit=None, stdout="", metrics=[])
        elif row is not None:
            row["stdout"] += line + "\n"
            if line.startswith("NEXT_JSON "):
                row["metrics"].append(json.loads(line[len("NEXT_JSON "):]))
            elif line.startswith("ERASURE_SAMPLE_EXIT "):
                row["exit"] = int(line.split()[-1])
    if row is not None:
        rows.append(row)
    (output / "plan.json").write_text(json.dumps(plan() if expected is None else expected, indent=2), encoding="utf-8")
    (output / "samples.jsonl").write_text("".join(json.dumps(r) + "\n" for r in rows), encoding="utf-8")


def run(version, download=False):
    if download:
        sys.path.insert(0, str(ROOT / "experiments"))
        from download import campaign
    expected = campaign.plan() if download else plan()
    script = campaign.guest_script() if download else guest_script()
    publisher = campaign.publish if download else publish
    output = ROOT / "target/erasure-audit/results" / ("download/" + version if download else version)
    output.mkdir(parents=True, exist_ok=True)
    stage = ROOT / "target/erasure-audit/guest" / version
    guest = stage / "root"
    for folder in ["bin", "proc", "sys", "dev", "tmp"]:
        (guest / folder).mkdir(parents=True, exist_ok=True)
    image = ROOT / f"target/lts-kernels/{version}/bzImage"
    assert image.is_file(), "restore or build the supported functional-test kernel first"
    bins = {n: ROOT / "target/erasure-audit/bin" / n for n in ["main", "erase"]}
    digests = {n: sha(p) for n, p in bins.items()}
    sources = json.loads((ROOT / "target/erasure-audit/builds.json").read_text())
    for name, binary in bins.items():
        identity = json.loads(subprocess.check_output([str(binary), "identity"], text=True))
        assert identity["effective_sources_sha256"] == sources[name]["effective_sources_sha256"]
        shutil.copyfile(binary, guest / "bin" / name)
        os.chmod(guest / "bin" / name, 0o755)
    shutil.copyfile("/usr/bin/busybox", guest / "bin/busybox")
    os.chmod(guest / "bin/busybox", 0o755)
    for app in ["sh", "mount", "ip", "poweroff", "uname", "cat", "unshare", "sha256sum", "setsid", "timeout", "kill"]:
        path = guest / "bin" / app
        if not path.exists():
            path.symlink_to("busybox")
    (guest / "init").write_text(script, encoding="utf-8")
    os.chmod(guest / "init", 0o755)
    subprocess.run("find . -print0 | cpio --null -o --format=newc | gzip -1 >../initramfs.gz", shell=True, cwd=guest, check=True)
    cpus = sorted(os.sched_getaffinity(0))
    assert len(cpus) >= 4 and os.access("/dev/kvm", os.R_OK | os.W_OK), "four CPUs and usable KVM required"
    environment = dict(kernel=version, machine="x86_64", sources=sources, binaries=digests,
                       kernel_image_sha256=sha(image), kernel_builder_sha256=sha(ROOT / "scripts/test-linux-lts.sh"),
                       harness_sha256={f.name: sha(f) for f in pathlib.Path(__file__).parent.glob("*.py")},
                       compiler=(ROOT / "target/erasure-audit/toolchain.txt").read_text(),
                       host_cpus=cpus, workload="download" if download else "echo",
                       warm_seconds=None if download else 1, measured_seconds=None if download else 5,
                       scope="actual-kernel KVM; pinned logical CPUs, not physical-core or interrupt isolation")
    if download:
        environment["target_class"] = os.environ.get("NEXT_DOWNLOAD_TARGET_CLASS")
    (output / "environment.json").write_text(json.dumps(environment, indent=2), encoding="utf-8")
    (output / "hardware.txt").write_text(subprocess.check_output(["lscpu"], text=True), encoding="utf-8")
    (output / "host-before.txt").write_text(pathlib.Path("/proc/stat").read_text())
    qmp = stage / "qmp.sock"
    assert not qmp.exists(), "stale VM socket; do not reuse a running stage"
    command = ["qemu-system-x86_64", "-accel", "kvm", "-cpu", "host", "-m", "1536", "-smp", "4", "-nographic", "-no-reboot", "-S",
               "-qmp", f"unix:{qmp},server=on,wait=off", "-object", "rng-random,filename=/dev/urandom,id=rng0", "-device", "virtio-rng-pci,rng=rng0",
               "-nic", "none", "-kernel", str(image), "-initrd", str(stage / "initramfs.gz"), "-append", "console=ttyS0 rdinit=/init panic=-1 quiet"]
    error = None
    with (output / "guest.log").open("w", encoding="utf-8") as log:
        proc = subprocess.Popen(command, stdout=log, stderr=subprocess.STDOUT)
        sock = socket.socket(socket.AF_UNIX)
        try:
            deadline = time.monotonic() + 10
            while True:
                assert proc.poll() is None, "QEMU startup failed; see guest.log"
                assert time.monotonic() < deadline, "QMP startup deadline"
                try:
                    sock.connect(str(qmp))
                    break
                except (FileNotFoundError, ConnectionRefusedError):
                    time.sleep(.05)
            wire = sock.makefile("rwb", buffering=0)
            json.loads(wire.readline())

            def call(name):
                wire.write((json.dumps(dict(execute=name)) + "\n").encode())
                while True:
                    reply = json.loads(wire.readline())
                    if "return" in reply:
                        return reply["return"]
                    assert "error" not in reply, reply

            call("qmp_capabilities")
            pins = []
            for cpu in call("query-cpus-fast"):
                host_cpu = cpus[cpu["cpu-index"]]
                os.sched_setaffinity(cpu["thread-id"], {host_cpu})
                pins.append(dict(vcpu=cpu["cpu-index"], thread=cpu["thread-id"], host_cpu=host_cpu))
            assert len(pins) == 4
            (output / "pins.json").write_text(json.dumps(pins, indent=2), encoding="utf-8")
            call("cont")
            assert proc.wait(timeout=1200) == 0, "VM execution failed"
        except BaseException as exc:
            error = str(exc)
        finally:
            if proc.poll() is None:
                proc.kill()
                proc.wait()
            sock.close()
            qmp.unlink(missing_ok=True)
    (output / "host-after.txt").write_text(pathlib.Path("/proc/stat").read_text())
    text = (output / "guest.log").read_text()
    collect(text, output, expected)
    report = publisher(output)  # Publish partial/failing measurements too.
    assert error is None, error
    assert f"ERASURE_KERNEL {version}" in text and "ERASURE_CPUS 0-3" in text
    clock = next(l.split("ERASURE_CLOCK ", 1)[1] for l in text.splitlines() if l.startswith("ERASURE_CLOCK "))
    assert "jiffies" not in clock and clock.strip()
    assert all(f"{digest}  /bin/{name}" in text for name, digest in digests.items()), "guest executable hash mismatch"
    assert "ERASURE_EXIT 0" in text and not report["errors"], "invalid data; see automatic analysis"
    print(json.dumps(dict(kernel=version, samples=report["observed_samples"],
                          usable=sum(b["validity"] == "usable" for b in report["blocks"]))))


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("kernel", choices=["6.12.115", "6.18.60"])
    parser.add_argument("--download", action="store_true")
    args = parser.parse_args()
    run(args.kernel, args.download)
