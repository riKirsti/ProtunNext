"""Read-only namespace TCP/qdisc diagnostics; never normal speed acceptance."""
import hashlib
import json
import os
import pathlib
import signal
import subprocess
import tempfile
import time

ROOT=pathlib.Path(__file__).resolve().parents[2]
OUT=ROOT/'target/erasure-audit'

def inspect(pid,host_namespace):
    namespace=os.readlink(f'/proc/{pid}/ns/net')
    assert namespace!=host_namespace,'refuse diagnostics in host network namespace'
    commands=[['ss','-tinmH'],['tc','-s','qdisc','show','dev','lo'],
              ['cat','/proc/net/snmp','/proc/net/netstat','/proc/net/snmp6']]
    return dict(namespace=namespace,observations=[dict(command=cmd,**result(subprocess.run(['nsenter','-t',str(pid),'-n',*cmd],capture_output=True,text=True,timeout=5))) for cmd in commands])

def result(p):
    return dict(exit=p.returncode,stdout=p.stdout,stderr=p.stderr)

def main():
    assert os.geteuid()==0
    cpus=sorted(os.sched_getaffinity(0));assert len(cpus)>=4
    host=os.readlink('/proc/self/ns/net')
    binary=OUT/'bin/main';digest=hashlib.sha256(binary.read_bytes()).hexdigest()
    sources=json.loads((OUT/'builds.json').read_text())
    output=OUT/'results/tcp-diagnostics';output.mkdir(parents=True,exist_ok=True)
    (output/'environment.json').write_text(json.dumps(dict(scope='sampled diagnosis only; no acceptance/speed ranking',sources=sources,binary_sha256=digest,cpus=cpus[:4],kernel=os.uname().release),indent=2))
    # Keep monitoring work away from the client/peer CPUs. The fixture pins each
    # role explicitly after fork; the allowed host CPU set is recorded above.
    os.sched_setaffinity(0,{cpus[1]})
    summaries=[]
    for number in range(12):
        folder=output/f'case-{number+1:02}';folder.mkdir()
        ready,go=folder/'ready',folder/'go'
        env=dict(os.environ,NEXT_PROFILE_READY=str(ready),NEXT_PROFILE_GO=str(go),
                 NEXT_TUN_RECEIVE_MODE='vnet',NEXT_DOWNLOAD_RATE_BPS='200000000',NEXT_NETWORK_DELAY_MS='5',
                 NEXT_PEER_CPUS=str(cpus[0]),NEXT_LOAD_CPUS=str(cpus[1]),NEXT_CLIENT_CPUS=str(cpus[2]),NEXT_SERVER_CPUS=str(cpus[3]),NEXT_WARM_SECONDS='1',NEXT_PEER_VECTORED='1')
        setup='set -e; ip link set lo up; tc qdisc add dev lo root netem delay 5ms; echo NEXT_NETEM 5; tc qdisc show dev lo; set +e; "$@"; code=$?; echo NEXT_DIAGNOSTIC_FINAL; tc -s qdisc show dev lo; cat /proc/net/snmp /proc/net/netstat /proc/net/snmp6; exit "$code"'
        command=['unshare','-n','sh','-c',setup,'netem',str(binary),'download-client',str(binary),'tcp','1','2048','v6','https']
        trace=[];peers=[];code=None
        with (folder/'fixture.log').open('w') as log:
            proc=subprocess.Popen(command,env=env,stdout=log,stderr=subprocess.STDOUT,start_new_session=True)
            try:
                deadline=time.monotonic()+60
                while not ready.exists():
                    assert proc.poll() is None and time.monotonic()<deadline,'fixture failed to warm'
                    time.sleep(.05)
                pid=int(ready.read_text())
                assert os.getpgid(pid)==proc.pid
                assert str(pid) in pathlib.Path(f'/proc/{proc.pid}/task/{proc.pid}/children').read_text().split()
                for child in pathlib.Path(f'/proc/{pid}/task/{pid}/children').read_text().split():
                    cmd=pathlib.Path(f'/proc/{child}/cmdline').read_bytes().split(b'\0')
                    if b'download-peer' in cmd:peers.append(int(child))
                assert len(peers)==1,'cannot identify owned peer child'
                assert os.readlink(f'/proc/{peers[0]}/ns/net')!=os.readlink(f'/proc/{pid}/ns/net')
                trace.append(dict(phase='before',client=inspect(pid,host),peer=inspect(peers[0],host)))
                go.touch();deadline=time.monotonic()+180
                while proc.poll() is None:
                    assert time.monotonic()<deadline,'diagnostic sample timeout'
                    try:trace.append(dict(phase='active',monotonic_ns=time.monotonic_ns(),client=inspect(pid,host),peer=inspect(peers[0],host)))
                    except (FileNotFoundError,ProcessLookupError):break
                    time.sleep(.25)
                code=proc.wait(timeout=10)
            finally:
                try:os.killpg(proc.pid,signal.SIGKILL)
                except ProcessLookupError:pass
                proc.wait()
                (folder/'trace.json').write_text(json.dumps(trace,indent=2))
        text=(folder/'fixture.log').read_text()
        assert 'NEXT_DIAGNOSTIC_FINAL' in text,'missing final owned-namespace counters'
        metrics=[json.loads(line[10:]) for line in text.splitlines() if line.startswith('NEXT_JSON ')]
        m=next((m for m in metrics if m.get('role')=='download-client'),None)
        row=dict(case=number+1,exit=code,verified=False)
        if m:
            assert m['binary_sha256']==m['peer']['binary_sha256']==m['load']['binary_sha256']==digest
            assert m['effective_sources_sha256']==sources['main']['effective_sources_sha256']
            assert m['namespace']==m['load']['namespace']==m['peer']['outer_namespace']!=m['peer']['namespace']
            row.update(verified=bool(m['load']['verified'] and m['load']['bytes']==134217728),Mbps=m['load']['bytes']*8/m['load']['seconds']/1e6,metrics=m)
        summaries.append(row)
        (output/'summary.json').write_text(json.dumps(summaries,indent=2))
        assert code==0 and row['verified'],'diagnostic transfer invalid; raw logs retained'

if __name__=='__main__':main()
