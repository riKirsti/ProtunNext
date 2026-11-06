"""Kernel TCP download plan and conservative validity analysis; reuse audit runners."""
import collections
import json
import math
import pathlib
import shlex
import statistics

BASELINE = "f7bf03f5b4dc3981eaadedbf3a7841c5ed5ee5c1"


def plan():
    rows = []
    for protocol in ["udp", "tcp", "stealth"]:
        for family, application, flows in [("v4", "http", 1), ("v4", "http", 8),
                                            ("v6", "https", 1), ("v6", "https", 8)]:
            block = f"{protocol}-{family}-{application}-{flows}"
            order = ["main", "erase", "erase", "main"] if len(rows) // 8 % 2 == 0 else ["erase", "main", "main", "erase"]
            for role, builds in [("aa", ["main"] * 4), ("compare", order)]:
                for slot, build in enumerate(builds):
                    rows.append(dict(sample=len(rows)+1, block=block, protocol=protocol, family=family,
                                     application=application, flows=flows, blocks_per_flow=32768//flows,
                                     bytes=2*1024*1024*1024, role=role, slot=slot, build=build))
    return rows


def arguments(row, peer):
    return ["download-client", peer, row["protocol"], str(row["flows"]),
            str(row["blocks_per_flow"]), row["family"], row["application"]]


def smoke_arguments(peer):
    rows = [["download-client",peer,protocol,str(flows),"16",family,application]
            for protocol in ["udp","tcp","stealth"]
            for family,application,flows in [("v4","http",1),("v6","https",8)]]
    # Reproduce rare packet rejection before paying for the full matrix.
    rows += [["download-client",peer,"stealth","1","4096","v4","http"] for _ in range(4)]
    return rows


def guest_script():
    lines = ["#!/bin/sh", "export PATH=/bin", "mount -t proc proc /proc",
             "mount -t sysfs sysfs /sys", "mount -t devtmpfs devtmpfs /dev",
             'echo "ERASURE_KERNEL $(uname -r)"',
             'echo "ERASURE_CLOCK $(cat /sys/devices/system/clocksource/clocksource0/current_clocksource)"',
             'echo "ERASURE_CPUS $(cat /sys/devices/system/cpu/online)"',
             "sha256sum /bin/main /bin/erase", "result=0",
             "export NEXT_PEER_CPUS=0 NEXT_LOAD_CPUS=1 NEXT_CLIENT_CPUS=2 NEXT_SERVER_CPUS=3"]
    for args in smoke_arguments("/bin/main"):
        command=["setsid","timeout","-s","KILL","30","unshare","-n","/bin/main",*args]
        prefix = "NEXT_DOWNLOAD_START_DELAY_MS=1250 " if args[2] == "stealth" else ""
        lines += ["echo " + shlex.quote("DOWNLOAD_SMOKE " + " ".join(args[2:])),
                  prefix+shlex.join(command)+" &", "sample_pid=$!", 'wait "$sample_pid"', "code=$?",
                  'kill -KILL -- "-$sample_pid" 2>/dev/null',
                  'if [ "$code" != 0 ]; then echo "ERASURE_EXIT 1"; poweroff -f; fi']
    for row in plan():
        command = ["setsid", "timeout", "-s", "KILL", "180", "unshare", "-n", "/bin/"+row["build"], *arguments(row, "/bin/main")]
        lines += ["echo " + shlex.quote("ERASURE_CASE " + json.dumps(row, separators=(",", ":"))),
                  shlex.join(command)+" &", "sample_pid=$!", 'wait "$sample_pid"', "code=$?",
                  'kill -KILL -- "-$sample_pid" 2>/dev/null', 'echo "ERASURE_SAMPLE_EXIT $code"',
                  '[ "$code" = 0 ] || result=1']
    lines += ['echo "ERASURE_EXIT $result"', "poweroff -f"]
    return "\n".join(lines)+"\n"


def geometric(values):
    assert values and all(math.isfinite(v) and v > 0 for v in values)
    return math.exp(statistics.mean(map(math.log, values)))


def metrics(row, env):
    assert row["exit"] == 0 and len(row["metrics"]) == 1, "process failed or incomplete metrics"
    m = row["metrics"][0]
    assert m["role"] == "download-client" and m["kernel"] == env["kernel"]
    assert m["effective_sources_sha256"] == env["sources"][row["build"]]["effective_sources_sha256"]
    for name in ["peer", "load"]:
        assert m[name]["effective_sources_sha256"] == env["sources"]["main"]["effective_sources_sha256"], "auxiliary changed"
    for key in ["protocol", "flows", "blocks_per_flow", "family", "application"]:
        assert m[key] == row[key], key
    load, peer = m["load"], m["peer"]
    for auxiliary in [load,peer]:
        assert all(math.isfinite(auxiliary[k]) and auxiliary[k]>0 for k in ["cpu_seconds","seconds"]), "invalid auxiliary accounting"
    for key in ["wire_cpu_seconds","server_cpu_seconds"]:
        if key in peer:
            assert math.isfinite(peer[key]) and peer[key]>=0, "invalid peer thread accounting"
    assert load["verified"] is True and load["bytes"] == row["bytes"] == 65536*row["blocks_per_flow"]*row["flows"]
    assert m["namespace"] == load["namespace"] == peer["outer_namespace"] != peer["namespace"], "kernel TCP bypasses tunnel"
    assert peer["handshakes"] >= 1
    assert m["after"]["state"] == m["before"]["state"] == "TunnelEstablished"
    for key in ["invalid_packets", "outer_io_errors", "outer_eof"]:
        assert m["counters"][key] == 0, key
    assert m["counters"]["decrypted_packets"] > 0 and m["counters"]["tun_reads"] > 0
    if row["protocol"] == "stealth":
        tls = json.loads(peer["tls"])
        assert tls["vectored"] is True and tls["version"] == "Some(TLSv1_3)"
    seconds, cpu = m["seconds"], m["client_cpu_seconds"]
    assert all(math.isfinite(v) and v > 0 for v in [seconds, cpu, load["seconds"], peer["seconds"], m["client_rss_kib"]])
    skew = abs(seconds/load["seconds"]-1)
    aux = max(load["cpu_seconds"]/load["seconds"],
              peer.get("wire_cpu_seconds",peer["cpu_seconds"])/peer["seconds"],
              peer.get("server_cpu_seconds",0)/peer["seconds"])
    return dict(Mbps=load["bytes"]*8/load["seconds"]/1e6, CPU_ns_per_byte=cpu*1e9/load["bytes"],
                CPU_core_percent=cpu/seconds*100, peak_RSS_MiB=m["client_rss_kib"]/1024,
                short=load["seconds"]<2, interval_skew=skew, auxiliary_core_fraction=aux,
                auxiliary_limited=aux>=.85)


def comparison(a, b):
    values = {}
    for key in ["Mbps", "CPU_ns_per_byte", "CPU_core_percent", "peak_RSS_MiB"]:
        x, y = geometric([m[key] for m in a]), geometric([m[key] for m in b])
        values[key] = dict(baseline=x, candidate=y, change_percent=100*(y/x-1))
    return values


def analyze(folder):
    folder = pathlib.Path(folder)
    env = json.loads((folder/"environment.json").read_text())
    expected = json.loads((folder/"plan.json").read_text())
    rows = [json.loads(line) for line in (folder/"samples.jsonl").read_text().splitlines()]
    planned = {r["sample"]:r for r in expected}
    checked, errors = {}, []
    for row in rows:
        try:
            sample = row["sample"]
            assert sample in planned and sample not in checked, "duplicate/unplanned sample"
            assert all(row[k] == value for k, value in planned[sample].items()), "plan mismatch"
            checked[sample] = metrics(row, env)
        except (AssertionError, KeyError, ValueError, ZeroDivisionError, TypeError) as error:
            errors.append(dict(sample=row.get("sample"), error=str(error) or type(error).__name__))
    if set(checked) != set(planned):
        errors.append(dict(error="missing/invalid planned samples", samples=sorted(set(planned)-set(checked))))
    groups = collections.defaultdict(list)
    for row in expected:
        groups[row["block"]].append(row)
    blocks = []
    for name, group in groups.items():
        block = dict(block=name, capacity_ranking=False)
        if any(r["sample"] not in checked for r in group):
            block["validity"] = "invalid/incomplete"
        else:
            def selected(role, slots=None, build=None):
                return [checked[r["sample"]] for r in group if r["role"] == role and
                        (slots is None or r["slot"] in slots) and (build is None or r["build"] == build)]
            control = comparison(selected("aa",[0,3]), selected("aa",[1,2]))
            result = comparison(selected("compare",build="main"),selected("compare",build="erase"))
            aa = all(abs(control[k]["change_percent"])<=5 for k in ["Mbps","CPU_ns_per_byte"])
            timing = all(not checked[r["sample"]]["short"] and checked[r["sample"]]["interval_skew"]<=.02 for r in group)
            auxiliary = any(checked[r["sample"]]["auxiliary_limited"] for r in group)
            block.update(control=control,result=result,AA_pass=aa,timing_pass=timing,auxiliary_limited=auxiliary,
                         validity="usable" if aa and timing else "inconclusive-control/timing",
                         capacity_ranking=aa and timing and not auxiliary)
        blocks.append(block)
    same_runtime = env["sources"]["main"].get("runtime_sources_sha256") == env["sources"]["erase"].get("runtime_sources_sha256")
    return dict(environment=env, same_runtime_sources=same_runtime, observed_samples=len(rows), expected_samples=len(expected), errors=errors, blocks=blocks)


def publish(folder):
    folder=pathlib.Path(folder)
    report=analyze(folder)
    (folder/"analysis.json").write_text(json.dumps(report,indent=2))
    lines=["# Kernel TCP download comparison", "", f"Samples: {report['observed_samples']}/{report['expected_samples']}; invalid-data errors: {len(report['errors'])}.",
           "", "CPU is client process cost (100% = one core); peak RSS excludes kernel and auxiliary memory.",
           "A/A screen: 5% for throughput and CPU/byte; minimum transfer 2s; interval skew <=2%.",
           "Peer/load >=0.85 core disqualifies capacity ranking. Noise and failures are retained.",
           "This loopback fixture is not NIC, RTT/loss, thermal, mobile or Proton WAN acceptance.", "",
           "| Workload | Validity | Download Mbps base / candidate | CPU ns/B base / candidate | CPU core % base / candidate | Peak RSS MiB base / candidate | Capacity ranking |",
           "|---|---|---:|---:|---:|---:|---|"]
    if report["same_runtime_sources"]:
        lines[2:2] = ["Production source fingerprints match: baseline characterization only; no optimization gain is claimed.", ""]
    for b in report["blocks"]:
        values=b.get("result")
        cells=[f"{values[k]['baseline']:.2f} / {values[k]['candidate']:.2f}" if values else "n/a" for k in ["Mbps","CPU_ns_per_byte","CPU_core_percent","peak_RSS_MiB"]]
        lines.append(f"| {b['block']} | {b['validity']} | {' | '.join(cells)} | {b['capacity_ranking']} |")
    (folder/"summary.md").write_text("\n".join(lines)+"\n")
    return report
