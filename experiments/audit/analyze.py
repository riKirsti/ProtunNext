"""Validate measurements; noisy data is inconclusive, not a correctness failure."""
import argparse
import collections
import json
import math
import pathlib
import statistics


def geometric(values):
    return math.exp(statistics.mean(math.log(v) for v in values))


def metrics(row, environment):
    assert row["exit"] == 0 and len(row["metrics"]) == 1
    m = row["metrics"][0]
    assert m["effective_sources_sha256"] == environment["sources"][row["build"]]["effective_sources_sha256"]
    for aux in ["peer", "load"]:
        assert m[aux]["effective_sources_sha256"] == environment["sources"]["main"]["effective_sources_sha256"]
    assert m["kernel"] == environment["kernel"]
    assert (m["protocol"], m["size"], m["window"], m["flows"], m["load"]["shape"]) == (
        row["protocol"], row["size"], row["window"], row["flows"], row["shape"])
    load = m["load"]
    assert load["bytes"] == load["packets"] * row["size"] > 0
    assert math.isclose(load["Mbps"], load["bytes"] * 8 / load["seconds"] / 1e6, rel_tol=1e-10)
    assert math.isclose(m["client_cpu_ns_per_byte"], m["client_cpu_seconds"] * 1e9 / load["bytes"], rel_tol=1e-10)
    assert load["seconds"] >= row["seconds"] and m["client_rss_kib"] > 0
    assert m["state"] == "TunnelEstablished" and m["peer"]["timed"]
    assert all(m["counters"][k] == 0 for k in ["invalid_packets", "outer_io_errors", "outer_eof"])
    if row["protocol"] == "udp":
        assert m["counters"]["io"]["gso_datagrams"] > 0
    if row["protocol"] == "stealth":
        tls = json.loads(m["peer"]["tls"])
        assert tls["version"] == "Some(TLSv1_3)" and tls["suite"] == "Some(TLS13_AES_256_GCM_SHA384)"
        assert tls["vectored"] is True
    return dict(Mbps=load["Mbps"], CPU_ns_per_byte=m["client_cpu_ns_per_byte"],
                CPU_core_percent=m["client_cpu_seconds"] / m["seconds"] * 100,
                peak_RSS_MiB=m["client_rss_kib"] / 1024,
                auxiliary_limited=max(m[a]["cpu_seconds"] / m[a]["seconds"] for a in ["peer", "load"]) >= .85,
                rate_achieved=not row["rate_pps"] or abs(load["packets"] / load["seconds"] / row["rate_pps"] - 1) <= .05)


def compare(base, candidate):
    values = {}
    for key in ["Mbps", "CPU_ns_per_byte", "CPU_core_percent", "peak_RSS_MiB"]:
        a = geometric([m[key] for m in base])
        b = geometric([m[key] for m in candidate])
        values[key] = dict(baseline=a, candidate=b, change_percent=(b / a - 1) * 100)
    return values


def analyze(folder):
    folder = pathlib.Path(folder)
    env = json.loads((folder / "environment.json").read_text())
    expected = json.loads((folder / "plan.json").read_text())
    rows = [json.loads(l) for l in (folder / "samples.jsonl").read_text().splitlines()]
    errors, checked = [], {}
    assert set(env["binaries"]) == {"main", "erase"} and len(set(env["binaries"].values())) == 2
    planned = {r["sample"]: r for r in expected}
    for row in rows:
        try:
            sample = row["sample"]
            assert sample not in checked and sample in planned, "duplicate/unplanned sample"
            assert all(row[k] == v for k, v in planned[sample].items()), "plan mismatch"
            checked[sample] = metrics(row, env)
        except (AssertionError, KeyError, ValueError, ZeroDivisionError) as e:
            errors.append(dict(sample=row.get("sample"), error=str(e) or type(e).__name__))
    if set(checked) != set(planned):
        errors.append(dict(error="missing or invalid planned samples", samples=sorted(set(planned) - set(checked))))
    groups = collections.defaultdict(list)
    for row in expected:
        groups[row["block"]].append(row)
    blocks = []
    for name, group in groups.items():
        block = dict(block=name, protocol=group[0]["protocol"], shape=group[0]["shape"], rate_pps=group[0]["rate_pps"])
        if any(r["sample"] not in checked for r in group):
            block["validity"] = "invalid/incomplete"
        else:
            aa = compare([checked[r["sample"]] for r in group if r["role"] == "aa-a"],
                         [checked[r["sample"]] for r in group if r["role"] == "aa-b"])
            result = compare([checked[r["sample"]] for r in group if r["role"] == "compare" and r["build"] == "main"],
                             [checked[r["sample"]] for r in group if r["role"] == "compare" and r["build"] == "erase"])
            aa_pass = all(abs(aa[k]["change_percent"]) <= 5 for k in ["Mbps", "CPU_ns_per_byte"])
            aux = any(checked[r["sample"]]["auxiliary_limited"] for r in group)
            rate = all(checked[r["sample"]]["rate_achieved"] for r in group)
            block.update(control=aa, result=result, AA_pass=aa_pass, auxiliary_limited=aux, rate_achieved=rate,
                         validity="usable" if aa_pass and rate else "inconclusive-control/load",
                         capacity_ranking=aa_pass and rate and not aux and not block["rate_pps"])
        blocks.append(block)
    return dict(environment=env, observed_samples=len(rows), expected_samples=len(expected), errors=errors, blocks=blocks)


def publish(folder):
    folder = pathlib.Path(folder)
    report = analyze(folder)
    (folder / "analysis.json").write_text(json.dumps(report, indent=2), encoding="utf-8")
    lines = ["# Erasure comparison validity", "", f"Samples: {report['observed_samples']}/{report['expected_samples']}; invalid-data errors: {len(report['errors'])}.",
             "", "CPU/byte is client process cost; RSS is lifetime client peak and excludes kernel/peer/generator memory.",
             "A/A tolerance is 5% for CPU/byte and throughput. Noisy cells are inconclusive; they are not retried until passing.", "",
             "| Workload | Validity | Throughput delta | CPU/byte delta | Peak RSS delta | Capacity ranking |",
             "|---|---|---:|---:|---:|---|"]
    for b in report["blocks"]:
        v = b.get("result")
        delta = [f"{v[k]['change_percent']:+.2f}%" if v else "n/a" for k in ["Mbps", "CPU_ns_per_byte", "peak_RSS_MiB"]]
        lines.append(f"| {b['block']} | {b['validity']} | {' | '.join(delta)} | {b.get('capacity_ranking', False)} |")
    (folder / "summary.md").write_text("\n".join(lines) + "\n", encoding="utf-8")
    return report


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("folder")
    args = parser.parse_args()
    report = publish(args.folder)
    print(json.dumps(dict(samples=report["observed_samples"], errors=report["errors"],
                          usable=sum(b["validity"] == "usable" for b in report["blocks"]))))
    raise SystemExit(1 if report["errors"] else 0)
