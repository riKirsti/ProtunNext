"""Separate, instrumented diagnostic: attach perf only after the fixture warms.

Never use profiled throughput as a normal-release performance comparison.
Requires perf and CAP_PERFMON/root, on an explicitly recorded native host kernel.
"""
import hashlib
import json
import os
import pathlib
import signal
import subprocess
import tempfile
import time

ROOT=pathlib.Path(__file__).resolve().parents[2]


def main():
    output=ROOT/"target/erasure-audit/results/profiles"
    output.mkdir(parents=True,exist_ok=True)
    binary=ROOT/"target/erasure-audit/bin/erase"
    peer=ROOT/"target/erasure-audit/bin/main"
    cpus=sorted(os.sched_getaffinity(0))
    assert len(cpus)>=4
    environment=dict(kernel=os.uname().release,architecture=os.uname().machine,
                     scope="native host diagnostic; instrumented frame-pointer build; not throughput acceptance",
                     perf=subprocess.check_output(["perf","version"],text=True).strip(),
                     compiler=(ROOT/"target/erasure-audit/toolchain.txt").read_text(),
                     event="cpu-clock", frequency_hz=499, call_graph="frame pointers", client_cpus=cpus[2:4],
                     builds=json.loads((ROOT/"target/erasure-audit/builds.json").read_text()),
                     binaries={p.name:hashlib.sha256(p.read_bytes()).hexdigest() for p in [binary,peer]})
    (output/"environment.json").write_text(json.dumps(environment,indent=2))
    for protocol in ["udp","tcp","stealth"]:
        with tempfile.TemporaryDirectory(prefix="profile-",dir=output) as folder:
            stage=pathlib.Path(folder)
            ready,go=stage/"ready",stage/"go"
            env=dict(os.environ,NEXT_PROFILE_READY=str(ready),NEXT_PROFILE_GO=str(go),
                     NEXT_PEER_CPUS=str(cpus[0]),NEXT_LOAD_CPUS=str(cpus[1]),NEXT_CLIENT_CPUS=",".join(map(str,cpus[2:4])))
            profiler=None
            with (output/f"{protocol}-fixture.log").open("w") as log, (output/f"{protocol}-perf.log").open("w") as perf_log:
                process=subprocess.Popen(["unshare","-n",str(binary),"download-client",str(peer),protocol,"1","16384","v4","http"],env=env,stdout=log,stderr=subprocess.STDOUT,start_new_session=True)
                try:
                    deadline=time.monotonic()+45
                    while not ready.exists():
                        assert process.poll() is None and time.monotonic()<deadline, "fixture did not warm"
                        time.sleep(.05)
                    pid=int(ready.read_text())
                    profiler=subprocess.Popen(["perf","record","-e","cpu-clock","-F","499","-g","--call-graph","fp","--inherit","-p",str(pid),"-o",str(output/f"{protocol}.data")],stdout=perf_log,stderr=subprocess.STDOUT)
                    time.sleep(.3)
                    assert profiler.poll() is None, "perf attachment failed; see retained log"
                    go.touch()
                    assert process.wait(timeout=180)==0, "profile fixture failed"
                finally:
                    if profiler is not None and profiler.poll() is None:
                        profiler.send_signal(signal.SIGINT)
                        profiler.wait(timeout=15)
                    try: os.killpg(process.pid,signal.SIGKILL)
                    except ProcessLookupError: pass
                    process.wait()
            with (output/f"{protocol}-report.txt").open("w") as report:
                subprocess.run(["perf","report","--stdio","--no-children","--sort","comm,dso,symbol","-i",str(output/f"{protocol}.data")],stdout=report,check=True)
            with (output/f"{protocol}-header.txt").open("w") as header:
                subprocess.run(["perf","report","--stdio","--header-only","-i",str(output/f"{protocol}.data")],stdout=header,check=True)


if __name__=="__main__":
    main()
