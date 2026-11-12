"""Kernel TCP download plan and conservative validity analysis; reuse audit runners."""
import collections
import json
import math
import os
import pathlib
import shlex
import statistics

BASELINE = "f7bf03f5b4dc3981eaadedbf3a7841c5ed5ee5c1"


def plan():
    rows = []
    rate = int(os.environ.get("NEXT_DOWNLOAD_RATE_BPS", "0"))
    tun_mode = os.environ.get("NEXT_DOWNLOAD_TUN_MODE", "raw")
    assert tun_mode in {"raw", "compare", "vnet"}, "invalid TUN comparison mode"
    client_cores = int(os.environ.get("NEXT_DOWNLOAD_CLIENT_CORES", "1"))
    parallel = os.environ.get("NEXT_DOWNLOAD_PARALLEL", "0") == "1"
    build_mode = os.environ.get("NEXT_BUILD_MODE", "release")
    assert build_mode in {"release", "pgo", "cpu", "pgo-cpu"}, "unknown build experiment"
    assert client_cores in {1, 2}, "invalid client CPU allowance"
    assert not parallel or client_cores == 2, "parallel comparison needs two allowed client CPUs"
    assert rate >= 0, "invalid offered download rate"
    total_blocks = 2048 if rate else int(os.environ.get("NEXT_DOWNLOAD_BLOCKS", "32768"))
    assert total_blocks > 0 and total_blocks % 8 == 0, "invalid verified payload size"
    network_delay = int(os.environ.get("NEXT_NETWORK_DELAY_MS", "0"))
    assert network_delay in {0, 5}, "unsupported disposable network scenario"
    for protocol in ["udp", "tcp", "stealth"]:
        for family, application, flows in [("v4", "http", 1), ("v4", "http", 8),
                                            ("v6", "https", 1), ("v6", "https", 8)]:
            block = f"{protocol}-{family}-{application}-{flows}"
            order = ["main", "erase", "erase", "main"] if len(rows) // 8 % 2 == 0 else ["erase", "main", "main", "erase"]
            for role, builds in [("aa", ["main"] * 4), ("compare", order)]:
                for slot, build in enumerate(builds):
                    rows.append(dict(sample=len(rows)+1, block=block, protocol=protocol, family=family,
                                     application=application, flows=flows, blocks_per_flow=total_blocks//flows,
                                     bytes=total_blocks*65536, role=role, slot=slot, build=build))
                    if rate:
                        assert rate >= flows, "offered rate cannot be split across flows"
                        rows[-1]["offered_rate_bps"] = rate
                    if network_delay:
                        rows[-1]["network_delay_ms"] = network_delay
                    if tun_mode == "compare":
                        rows[-1]["tun_mode"] = "vnet" if build == "erase" else "raw"
                    elif tun_mode == "vnet":
                        rows[-1]["tun_mode"] = "vnet"
                    if client_cores == 2:
                        rows[-1].update(client_cores=2, shared_peer_server_cpu=True)
                    if parallel:
                        rows[-1]["parallel_receive"] = build == "erase"
                        if build == "erase":
                            rows[-1]["thread_placement"] = "split"
                    if build_mode != "release":
                        rows[-1]["build_mode"] = build_mode if build == "erase" else "release"
                        rows[-1]["build_gate_version"] = 3
    selected=os.environ.get("NEXT_DOWNLOAD_BLOCK","")
    if selected:
        assert selected in {r["block"] for r in rows}, "unknown download workload block"
        rows=[r for r in rows if r["block"]==selected]
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
    assert int(os.environ.get("NEXT_NETWORK_DELAY_MS", "0")) == 0, "network delay scenario requires native disposable namespaces"
    lines = ["#!/bin/sh", "export PATH=/bin", "mount -t proc proc /proc",
             "mount -t sysfs sysfs /sys", "mount -t devtmpfs devtmpfs /dev",
             'echo "ERASURE_KERNEL $(uname -r)"',
             'echo "ERASURE_CLOCK $(cat /sys/devices/system/clocksource/clocksource0/current_clocksource)"',
             'echo "ERASURE_CPUS $(cat /sys/devices/system/cpu/online)"',
             "sha256sum /bin/main /bin/erase", "result=0",
             "export NEXT_PEER_CPUS=0 NEXT_LOAD_CPUS=1 NEXT_CLIENT_CPUS=2 NEXT_SERVER_CPUS=3"]
    lines.append("export NEXT_DOWNLOAD_RATE_BPS=" + str(int(os.environ.get("NEXT_DOWNLOAD_RATE_BPS", "0"))))
    vnet = os.environ.get("NEXT_DOWNLOAD_TUN_MODE") == "vnet"
    lines.append("export NEXT_TUN_RECEIVE_MODE=" + ("vnet" if vnet else "raw"))
    if int(os.environ.get("NEXT_DOWNLOAD_CLIENT_CORES", "1")) == 2:
        lines.append("export NEXT_CLIENT_CPUS=2,3 NEXT_SERVER_CPUS=0 NEXT_SHARED_PEER_SERVER_CPU=1")
    for args in smoke_arguments("/bin/main"):
        command=["setsid","timeout","-s","KILL","30","unshare","-n","/bin/main",*args]
        prefix = "NEXT_DOWNLOAD_START_DELAY_MS=1250 " if args[2] == "stealth" else ""
        lines += ["echo " + shlex.quote("DOWNLOAD_SMOKE " + " ".join(args[2:])),
                  prefix+shlex.join(command)+" &", "sample_pid=$!", 'wait "$sample_pid"', "code=$?",
                  'kill -KILL -- "-$sample_pid" 2>/dev/null',
                  'if [ "$code" != 0 ]; then echo "ERASURE_EXIT 1"; poweroff -f; fi']
    if os.environ.get("NEXT_DOWNLOAD_TUN_MODE", "raw") in {"compare", "vnet"}:
        for args in smoke_arguments("/bin/main")[:6]:
            command = ["setsid", "timeout", "-s", "KILL", "30", "unshare", "-n", "/bin/erase", *args]
            lines += ["echo DOWNLOAD_CANDIDATE_VNET_SMOKE",
                      "NEXT_TUN_RECEIVE_MODE=vnet " + shlex.join(command) + " &",
                      "sample_pid=$!", 'wait "$sample_pid"', "code=$?",
                      'kill -KILL -- "-$sample_pid" 2>/dev/null',
                      'if [ "$code" != 0 ]; then echo "ERASURE_EXIT 1"; poweroff -f; fi']
    for row in plan():
        command = ["setsid", "timeout", "-s", "KILL", "180", "unshare", "-n", "/bin/"+row["build"], *arguments(row, "/bin/main")]
        lines += ["echo " + shlex.quote("ERASURE_CASE " + json.dumps(row, separators=(",", ":"))),
                  "NEXT_TUN_RECEIVE_MODE=" + row.get("tun_mode", "raw") + " " + shlex.join(command)+" &", "sample_pid=$!", 'wait "$sample_pid"', "code=$?",
                  'kill -KILL -- "-$sample_pid" 2>/dev/null', 'echo "ERASURE_SAMPLE_EXIT $code"',
                  '[ "$code" = 0 ] || result=1']
    lines += ['echo "ERASURE_EXIT $result"', "poweroff -f"]
    return "\n".join(lines)+"\n"


def geometric(values):
    assert values and all(math.isfinite(v) and v > 0 for v in values)
    return math.exp(statistics.mean(map(math.log, values)))


def metrics(row, env):
    if row.get("network_delay_ms"):
        assert "NEXT_NETEM 5" in row.get("stdout", "") and "netem" in row["stdout"] and "delay 5ms" in row["stdout"], "network impairment not configured"
    assert row["exit"] == 0 and len(row["metrics"]) == 1, "process failed or incomplete metrics"
    m = row["metrics"][0]
    if "build_mode" in row:
        cfg = env["sources"][row["build"]]["build_configuration"]
        assert cfg["mode"] == row["build_mode"], "build configuration mismatch"
        assert ("-Ctarget-cpu=native" in cfg["rustflags"]) == (cfg["mode"] in {"cpu", "pgo-cpu"}), "undeclared/missing CPU tuning"
        assert any(f.startswith("-Cprofile-use=") for f in cfg["rustflags"]) == (cfg["mode"] in {"pgo", "pgo-cpu"}), "undeclared/missing PGO tuning"
        assert cfg["binary_sha256"] == env["binaries"][row["build"]], "built binary identity mismatch"
        assert m["binary_sha256"] == env["binaries"][row["build"]], "running client binary mismatch"
        assert m["peer"]["binary_sha256"] == m["load"]["binary_sha256"] == env["binaries"]["main"], "auxiliary binary changed"
        assert not any("profile-generate" in flag for flag in cfg["rustflags"]), "instrumented timing is not a speed result"
        if cfg["mode"] != "release":
            assert env["binaries"]["main"] != env["binaries"]["erase"], "build experiment reused baseline binary"
            assert cfg["erasure_checked"] is True, "optimized erasure not checked"
            if row.get("build_gate_version", 1) >= 2:
                assert cfg["gate_version"] == row["build_gate_version"] and cfg["negative_control"]["expected_rejection"] is True, "optimized negative control not checked"
        if cfg["mode"] in {"pgo", "pgo-cpu"}:
            profile = cfg["profile"]
            assert len(profile["sha256"]) == 64 and profile["raw_files"], "missing training profile identity"
            assert profile["instrumentation_absent"] is True, "final instrumentation not checked"
            assert "-Cprofile-use="+profile["path"] in cfg["rustflags"], "profile not consumed"
            cases = profile["training"]["cases"]
            assert len(cases) == 6 and all(c["verified"] and c["flows"] in {2,4} for c in cases), "invalid/overlapping PGO training"
            assert {c["name"].split('-')[0] for c in cases} == {"udp","tcp","stealth"}, "incomplete training transports"
        if cfg["mode"] in {"cpu", "pgo-cpu"}:
            assert cfg["cpu_identity"]["sha256"] == env["cpu_identity"]["sha256"], "CPU-specific binary evaluated on different CPU"
            assert "-Ctarget-cpu=native" in cfg["rustflags"], "CPU tuning was not applied"
    if "client_cores" in row:
        assert len(set(m["client_affinity_cpus"])) == row["client_cores"], "client CPU allowance mismatch"
        assert m["client_affinity_cpus"] == env["cpu_layout"]["client"], "client CPU pin identity mismatch"
        assert m["shared_peer_server_cpu"] == row["shared_peer_server_cpu"], "auxiliary CPU layout mismatch"
    if row.get("parallel_receive"):
        assert m["counters"].get("parallel_batches", 0) > 0, "parallel receive was not exercised"
        assert m["counters"].get("parallel_helper_packets", 0) > 0
        assert m["counters"].get("parallel_owner_packets", 0) > 0
        assert 8 <= m["counters"].get("parallel_max_batch", 0) <= 32
        if row.get("thread_placement") == "split":
            assert {m["counters"]["parallel_owner_cpu"],m["counters"]["parallel_helper_cpu"]} == set(m["client_affinity_cpus"]), "receive threads do not occupy the two allowed CPUs"
    elif "parallel_receive" in row:
        assert m["counters"].get("parallel_batches", 0) == 0, "baseline unexpectedly used parallel receive"
    assert m.get("tun_mode", "raw") == row.get("tun_mode", "raw"), "TUN mode identity mismatch"
    if row.get("tun_mode") == "vnet":
        assert m["counters"].get("tun_coalesced_packets", 0) > 0, "coalescing was not exercised"
        assert 1 < m["counters"].get("tun_max_write_batch", 0) <= 32, "invalid coalescing batch"
        assert m["counters"].get("tun_offload_fallbacks") == 0, "offload fallback invalidates the coalescing comparison"
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
    assert m.get("offered_rate_bps", 0) == row.get("offered_rate_bps", 0), "offered rate identity mismatch"
    assert m.get("network_delay_ms", 0) == row.get("network_delay_ms", 0), "network scenario identity mismatch"
    if "offered_rate_bps" in row:
        rate = row["offered_rate_bps"]
        assert isinstance(rate, int) and not isinstance(rate, bool) and rate > 0
        assert abs(load["bytes"]*8/load["seconds"]/rate-1) <= .05, "offered rate was not sustained"
    skew = abs(seconds/load["seconds"]-1)
    aux = max(load["cpu_seconds"]/load["seconds"],
              peer.get("wire_cpu_seconds",peer["cpu_seconds"])/peer["seconds"],
              peer.get("server_cpu_seconds",0)/peer["seconds"])
    if row.get("shared_peer_server_cpu"):
        aux = max(aux, peer["cpu_seconds"]/peer["seconds"])
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
        paced = any(r.get("offered_rate_bps", 0) for r in group)
        block = dict(block=name, capacity_ranking=False, rate_controlled=bool(paced))
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
                         capacity_ranking=aa and timing and not auxiliary and not paced)
        blocks.append(block)
    baseline_runtime = env["sources"]["main"].get("runtime_sources_sha256")
    same_runtime = bool(baseline_runtime) and baseline_runtime == env["sources"]["erase"].get("runtime_sources_sha256")
    return dict(environment=env, same_runtime_sources=same_runtime, observed_samples=len(rows), expected_samples=len(expected), errors=errors, blocks=blocks)


def publish(folder):
    folder=pathlib.Path(folder)
    report=analyze(folder)
    (folder/"analysis.json").write_text(json.dumps(report,indent=2))
    lines=["# Kernel TCP download comparison", "", f"Samples: {report['observed_samples']}/{report['expected_samples']}; invalid-data errors: {len(report['errors'])}.",
           "", "CPU is client process cost (100% = one core); peak RSS excludes kernel and auxiliary memory.",
           "A/A screen: 5% for throughput and CPU/byte; minimum transfer 2s; interval skew <=2%.",
           "Peer/load >=0.85 core disqualifies capacity ranking. Noise and failures are retained.",
           "Rate-controlled samples measure resource cost at equal offered load and cannot rank maximum capacity.",
           "This loopback fixture is not NIC, RTT/loss, thermal, mobile or Proton WAN acceptance.", "",
           "| Workload | Validity | Download Mbps base / candidate | CPU ns/B base / candidate | CPU core % base / candidate | Peak RSS MiB base / candidate | Capacity ranking |",
           "|---|---|---:|---:|---:|---:|---|"]
    if report["same_runtime_sources"]:
        tuned = any(r.get("build_mode") in {"pgo","cpu","pgo-cpu"} for r in json.loads((folder/"plan.json").read_text()))
        message = "Runtime source fingerprints match; this is a recorded compiler-build comparison, not a runtime-source optimization." if tuned else "Production source fingerprints match: baseline characterization only; no optimization gain is claimed."
        lines[2:2] = [message, ""]
    for b in report["blocks"]:
        values=b.get("result")
        cells=[f"{values[k]['baseline']:.2f} / {values[k]['candidate']:.2f}" if values else "n/a" for k in ["Mbps","CPU_ns_per_byte","CPU_core_percent","peak_RSS_MiB"]]
        lines.append(f"| {b['block']} | {b['validity']} | {' | '.join(cells)} | {b['capacity_ranking']} |")
    (folder/"summary.md").write_text("\n".join(lines)+"\n")
    return report
