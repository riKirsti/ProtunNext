"""Train only the instrumented client; helper binaries remain frozen baseline."""
import json
import os
import pathlib
import signal
import subprocess
import sys

from common import cpu_identity, sha


def main():
    client, baseline, folder, client_source, baseline_source = sys.argv[1:]
    folder = pathlib.Path(folder)
    cpus = sorted(os.sched_getaffinity(0))
    assert len(cpus) >= 4
    env = dict(os.environ, NEXT_CLIENT_CPUS=str(cpus[2]), NEXT_PEER_CPUS=str(cpus[0]),
               NEXT_LOAD_CPUS=str(cpus[1]), NEXT_SERVER_CPUS=str(cpus[3]),
               NEXT_SHARED_PEER_SERVER_CPU='0', NEXT_TUN_RECEIVE_MODE='vnet',
               NEXT_DOWNLOAD_RATE_BPS='0', NEXT_PEER_VECTORED='1', NEXT_WARM_SECONDS='1',
               NEXT_NETWORK_DELAY_MS='0',
               LLVM_PROFILE_FILE=str(folder/'raw'/'%m-%p.profraw'))
    # Evaluation uses one/eight flows; training deliberately uses two/four.
    cases = []
    for protocol in ['udp', 'tcp', 'stealth']:
        for family, application, flows in [('v4', 'http', 2), ('v6', 'https', 4)]:
            name = f'{protocol}-{family}-{application}-{flows}'
            command = ['unshare', '-n', client, 'download-client', baseline,
                       protocol, str(flows), '128', family, application]
            case_env = dict(env, NEXT_DOWNLOAD_START_DELAY_MS='1250' if protocol == 'stealth' else '0')
            log = folder/(name+'.log')
            with log.open('w') as stream:
                proc = subprocess.Popen(command, env=case_env, stdout=stream,
                                        stderr=subprocess.STDOUT, start_new_session=True)
                try:
                    code = proc.wait(timeout=90)
                finally:
                    try:
                        os.killpg(proc.pid, signal.SIGKILL)
                    except ProcessLookupError:
                        pass
                    proc.wait()
            assert code == 0, f'training failed: {name}; retained log'
            rows = [json.loads(line[10:]) for line in log.read_text().splitlines() if line.startswith('NEXT_JSON ')]
            m = next(row for row in rows if row['role'] == 'download-client')
            assert m['effective_sources_sha256'] == client_source
            assert m['binary_sha256'] == sha(client)
            assert m['peer']['binary_sha256'] == m['load']['binary_sha256'] == sha(baseline)
            assert m['peer']['effective_sources_sha256'] == m['load']['effective_sources_sha256'] == baseline_source
            assert m['load']['verified'] and m['load']['bytes'] == flows*128*65536
            assert m['namespace'] == m['load']['namespace'] == m['peer']['outer_namespace'] != m['peer']['namespace']
            assert m['after']['state'] == m['before']['state'] == 'TunnelEstablished'
            assert all(m['counters'][k] == 0 for k in ['invalid_packets', 'outer_eof', 'outer_io_errors'])
            assert m['tun_mode'] == 'vnet' and m['counters']['tun_coalesced_packets'] > 0
            cases.append(dict(name=name, flows=flows, bytes=m['load']['bytes'], verified=True,
                              client_source=client_source, helper_source=baseline_source))
    report = dict(scope='instrumented training only; no performance estimate', kernel=os.uname().release,
                  machine=os.uname().machine, cpu_identity=cpu_identity(), cpu_layout=cpus[:4], cases=cases)
    (folder/'training.json').write_text(json.dumps(report, indent=2))


if __name__ == '__main__':
    main()
