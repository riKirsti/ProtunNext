"""Stage 7 same-host factorial comparisons; no portable default promotion."""
import json
import os
import pathlib
import shutil
import subprocess
import sys

from common import sha, cpu_identity

ROOT=pathlib.Path(__file__).resolve().parents[2]
OUT=ROOT/'target/erasure-audit'
MODES=('pgo','cpu','pgo-cpu')

def main():
    scenario=sys.argv[1]
    assert scenario in {'paced','capacity','delay'}
    assert os.environ['BASELINE']=='83582248f3a8e153c6ee75d2846ef7c7064b6323'
    if len(sys.argv)==3:
        assert sys.argv[2]=='--evaluate'
        evaluate(scenario)
        return
    target=subprocess.check_output(['rustc','-vV'],text=True).split('host: ')[1].splitlines()[0]
    variants={}; baseline_hash=None
    for i,mode in enumerate(MODES):
        env=dict(os.environ,NEXT_BUILD_MODE=mode,NEXT_BUILD_REUSE='1' if i else '0')
        subprocess.run([sys.executable,str(ROOT/'experiments/build/tune.py'),target],env=env,check=True)
        sources=json.loads((OUT/'builds.json').read_text())
        if baseline_hash is None:baseline_hash=sha(OUT/'bin/main')
        assert sha(OUT/'bin/main')==baseline_hash,'baseline binary changed between factors'
        folder=OUT/'matrix'/mode;folder.mkdir(parents=True)
        for name in ['main','erase','optimized-erasure-codegen']:
            shutil.copy2(OUT/'bin'/name,folder/name)
        for p in [*OUT.glob('*.log'),*OUT.glob('*disassembly.txt'),OUT/'toolchain.txt']:
            shutil.copy2(p,folder/p.name)
        (folder/'builds.json').write_text(json.dumps(sources,indent=2))
        variants[mode]=dict(folder=folder,sources=sources)
    subprocess.run(['sudo','env','PATH='+os.environ['PATH'],'BASELINE='+os.environ['BASELINE'],
                    sys.executable,str(pathlib.Path(__file__).resolve()),scenario,'--evaluate'],check=True,cwd=ROOT)

def evaluate(scenario):
    assert os.geteuid()==0
    variants={mode:dict(folder=OUT/'matrix'/mode,sources=json.loads((OUT/'matrix'/mode/'builds.json').read_text())) for mode in MODES}
    baseline_hash=sha(variants['pgo']['folder']/'main')
    assert all(sha(v['folder']/'main')==baseline_hash for v in variants.values()), 'baseline factor changed'
    sys.path.insert(0,str(ROOT/'experiments/audit'))
    import native
    sys.path.insert(0,str(ROOT/'experiments/download'))
    import campaign
    os.environ.update(NEXT_DOWNLOAD_RATE_BPS='0' if scenario=='capacity' else '200000000',
                      NEXT_DOWNLOAD_BLOCKS='65536',NEXT_NETWORK_DELAY_MS='5' if scenario=='delay' else '0',
                      NEXT_DOWNLOAD_TUN_MODE='vnet',NEXT_DOWNLOAD_CLIENT_CORES='1',NEXT_DOWNLOAD_PARALLEL='0')
    blocks=list(dict.fromkeys(r['block'] for r in campaign.plan()))
    schedule=[]
    # Rotate which factor is first for each workload, keeping its own A/A and
    # ABBA/BAAB comparisons. Helpers and baseline binary remain identical.
    for number,block in enumerate(blocks):
        order=MODES[number%3:]+MODES[:number%3]
        for mode in order:schedule.append(dict(block=block,mode=mode))
    result=OUT/'results'/'acceptance'/scenario;result.mkdir(parents=True)
    index=dict(scenario=scenario,cpu_identity=cpu_identity(),baseline_binary_sha256=baseline_hash,
               schedule=schedule,completed=[],failures=[])
    for row in schedule:
        mode,block=row['mode'],row['block'];variant=variants[mode]
        os.environ.update(NEXT_BUILD_MODE=mode,NEXT_DOWNLOAD_BLOCK=block)
        folder=result/mode/block
        try:
            native.run(download=True,output_override=folder,
                       bins_override={n:variant['folder']/n for n in ['main','erase']},
                       sources_override=variant['sources'],smoke=block==blocks[0],
                       compiler_override=(variant['folder']/'toolchain.txt').read_text())
            analysis=json.loads((folder/'analysis.json').read_text())
            assert not analysis['errors']
            index['completed'].append(dict(row,validity=analysis['blocks'][0]['validity']))
        except Exception as error:
            index['failures'].append(dict(row,error=type(error).__name__+': '+str(error)))
        (result/'index.json').write_text(json.dumps(index,indent=2))
    assert len(index['completed'])==len(schedule) and not index['failures'], 'incomplete/invalid comparisons retained in index'
    print('Completed same-host independent and combined builds; inspect controls/headroom before selecting a winner.')

if __name__=='__main__':main()
