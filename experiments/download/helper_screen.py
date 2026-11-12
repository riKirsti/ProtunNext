"""Helper-only screen: byte-identical portable client for every condition."""
import json,os,pathlib,subprocess,sys
ROOT=pathlib.Path(__file__).resolve().parents[2]
OUT=ROOT/'target/erasure-audit'

def main():
    assert os.geteuid()==0
    sys.path.insert(0,str(ROOT/'experiments/audit'))
    import native
    sources=json.loads((OUT/'builds.json').read_text())
    # Main/erase are historical report labels, not different client binaries.
    fixed={name:sources['main'] for name in ['main','erase']}
    bins={name:OUT/'bin/main' for name in ['main','erase']}
    os.environ.update(NEXT_BUILD_MODE='release',NEXT_DOWNLOAD_RATE_BPS='0',NEXT_DOWNLOAD_BLOCKS='32768',NEXT_NETWORK_DELAY_MS='0',NEXT_DOWNLOAD_TUN_MODE='vnet',NEXT_DOWNLOAD_CLIENT_CORES='1',NEXT_DOWNLOAD_PARALLEL='0')
    output=OUT/'results/helper-screen';output.mkdir(parents=True)
    index=[]
    for strategy in ['inplace','reader']:
        os.environ['NEXT_PEER_HELPER_SCREEN']=strategy
        for protocol in ['udp','tcp','stealth']:
            block=protocol+'-v4-http-1'
            os.environ['NEXT_DOWNLOAD_BLOCK']=block
            folder=output/strategy/block
            try:
                native.run(download=True,output_override=folder,bins_override=bins,sources_override=fixed,smoke=protocol=='udp')
                report=json.loads((folder/'analysis.json').read_text())
                index.append(dict(strategy=strategy,block=block,validity=report['blocks'][0]['validity'],errors=report['errors']))
            except Exception as error:index.append(dict(strategy=strategy,block=block,error=repr(error)))
            (output/'index.json').write_text(json.dumps(index,indent=2))
    assert len(index)==6 and all(not r.get('error') and not r.get('errors') for r in index), 'helper screen has invalid data; retained'

if __name__=='__main__':main()
