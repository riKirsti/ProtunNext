"""Run the same bounded plan natively; record the real host kernel explicitly."""
import argparse
import json
import os
import pathlib
import signal
import subprocess
import sys

from analyze import publish
from lts import collect, sha
from run import plan

ROOT = pathlib.Path(__file__).resolve().parents[2]
OUT = ROOT / "target/erasure-audit/results/native"


def run(download=False):
    if download:
        sys.path.insert(0, str(ROOT / "experiments"))
        from download import campaign
    output = OUT / "download" if download else OUT
    expected = campaign.plan() if download else plan()
    publisher = campaign.publish if download else publish
    output.mkdir(parents=True, exist_ok=True)
    cpus = sorted(os.sched_getaffinity(0))
    assert len(cpus) >= 4
    bins = {n: ROOT / "target/erasure-audit/bin" / n for n in ["main", "erase"]}
    sources = json.loads((ROOT / "target/erasure-audit/builds.json").read_text())
    for name, binary in bins.items():
        assert json.loads(subprocess.check_output([str(binary), "identity"], text=True))["effective_sources_sha256"] == sources[name]["effective_sources_sha256"]
    env = dict(kernel=os.uname().release, machine=os.uname().machine, sources=sources,
               binaries={n: sha(p) for n, p in bins.items()}, host_cpus=cpus,
               compiler=(ROOT / "target/erasure-audit/toolchain.txt").read_text(),
               scope="native diagnostic: actual host kernel; not supported-LTS kernel acceptance")
    (output / "environment.json").write_text(json.dumps(env, indent=2))
    (output / "hardware.txt").write_text(subprocess.check_output(["lscpu"], text=True))
    process_env = dict(os.environ, NEXT_PEER_CPUS=str(cpus[0]), NEXT_LOAD_CPUS=str(cpus[1]),
                       NEXT_CLIENT_CPUS=",".join(map(str, cpus[2:4])), NEXT_WARM_SECONDS="1", NEXT_PEER_VECTORED="1")
    if download:
        process_env.update(NEXT_CLIENT_CPUS=str(cpus[2]), NEXT_SERVER_CPUS=str(cpus[3]))
    with (output / "guest.log").open("w") as log:
        if download:
            smokes = [("main", "raw", args) for args in campaign.smoke_arguments(str(bins["main"]))]
            if os.environ.get("NEXT_DOWNLOAD_TUN_MODE", "raw") == "compare":
                smokes += [("erase", "vnet", args) for args in campaign.smoke_arguments(str(bins["main"]))[:6]]
            for build, tun_mode, arguments in smokes:
                log.write(f"DOWNLOAD_SMOKE {build} {tun_mode} " + " ".join(arguments[2:]) + "\n")
                log.flush()
                smoke_env = dict(process_env, NEXT_TUN_RECEIVE_MODE=tun_mode, NEXT_DOWNLOAD_START_DELAY_MS="1250" if arguments[2] == "stealth" else "0")
                process = subprocess.Popen(["unshare", "-n", str(bins[build]), *arguments], env=smoke_env,
                                           stdout=log, stderr=subprocess.STDOUT, start_new_session=True)
                try:
                    code = process.wait(timeout=30)
                finally:
                    try:
                        os.killpg(process.pid, signal.SIGKILL)
                    except ProcessLookupError:
                        pass
                    process.wait()
                assert code == 0, "download smoke failed; see retained guest.log"
        for row in expected:
            if download:
                process_env["NEXT_TUN_RECEIVE_MODE"] = row.get("tun_mode", "raw")
            log.write("ERASURE_CASE " + json.dumps(row) + "\n")
            log.flush()
            process_env["NEXT_LOAD_SHAPE"] = row.get("shape", "download")
            process_env.pop("NEXT_RATE_PPS", None)
            if row.get("rate_pps"):
                process_env["NEXT_RATE_PPS"] = str(row["rate_pps"])
            arguments = campaign.arguments(row, str(bins["main"])) if download else ["client", str(bins["main"]), row["protocol"],
                         str(row["size"]), str(row["window"]), str(row["seconds"]), str(row["flows"])]
            command = ["unshare", "-n", str(bins[row["build"]]), *arguments]
            proc = subprocess.Popen(command, env=process_env, stdout=log, stderr=subprocess.STDOUT, start_new_session=True)
            try:
                code = proc.wait(timeout=180 if download else 40)
            except subprocess.TimeoutExpired:
                code = 124
            finally:
                try:
                    os.killpg(proc.pid, signal.SIGKILL)
                except ProcessLookupError:
                    pass
                proc.wait()
            log.write(f"ERASURE_SAMPLE_EXIT {code}\n")
    collect((output / "guest.log").read_text(), output, expected)
    report = publisher(output)
    assert not report["errors"], report["errors"]


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--download", action="store_true")
    run(parser.parse_args().download)
