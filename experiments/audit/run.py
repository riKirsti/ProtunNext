"""Focused erasure-only plan, shared by actual-LTS guests and validation."""
import json


def plan():
    rows = []
    for pi, protocol in enumerate(["udp", "tcp", "stealth"]):
        for ci, (shape, rate) in enumerate([("burst", 0), ("sliding", 0), ("burst", 10000)]):
            block = f"{protocol}-{ci}"
            for slot in range(4):
                rows.append(dict(protocol=protocol, shape=shape, rate_pps=rate, block=block,
                                 role="aa-a" if slot in [0, 3] else "aa-b", slot=slot, build="erase"))
            order = ["main", "erase", "erase", "main"] if (pi + ci) % 2 == 0 else ["erase", "main", "main", "erase"]
            for slot, build in enumerate(order):
                rows.append(dict(protocol=protocol, shape=shape, rate_pps=rate, block=block,
                                 role="compare", slot=slot, build=build))
    for index, row in enumerate(rows, 1):
        row.update(sample=index, size=1200, window=32, flows=1, resources="two",
                   peer_vectored=True, seconds=5, warm_seconds=1)
    return rows


def guest_script():
    lines = ["#!/bin/sh", "export PATH=/bin", "mount -t proc proc /proc",
             "mount -t sysfs sysfs /sys", "mount -t devtmpfs devtmpfs /dev",
             'echo "ERASURE_KERNEL $(uname -r)"',
             'echo "ERASURE_CLOCK $(cat /sys/devices/system/clocksource/clocksource0/current_clocksource)"',
             'echo "ERASURE_CPUS $(cat /sys/devices/system/cpu/online)"',
             "sha256sum /bin/main /bin/erase", "result=0",
             "export NEXT_PEER_CPUS=0 NEXT_LOAD_CPUS=1 NEXT_CLIENT_CPUS=2,3",
             "export NEXT_WARM_SECONDS=1 NEXT_PEER_VECTORED=1"]
    for row in plan():
        lines += ["echo 'ERASURE_CASE " + json.dumps(row, separators=(",", ":")) + "'",
                  f"export NEXT_LOAD_SHAPE={row['shape']}"]
        lines.append(f"export NEXT_RATE_PPS={row['rate_pps']}" if row["rate_pps"] else "unset NEXT_RATE_PPS")
        lines += [f"setsid timeout -s KILL 40 unshare -n /bin/{row['build']} client /bin/erase {row['protocol']} 1200 32 5 1 &",
                  'sample_pid=$!', 'wait "$sample_pid"', 'code=$?',
                  'kill -KILL -- "-$sample_pid" 2>/dev/null',
                  'echo "ERASURE_SAMPLE_EXIT $code"', '[ "$code" = 0 ] || result=1']
    lines += ['echo "ERASURE_EXIT $result"', "poweroff -f"]
    return "\n".join(lines) + "\n"


if __name__ == "__main__":
    print(json.dumps(plan(), indent=2))
