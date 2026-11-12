"""Reject fixtures that can manufacture download gains or hide invalid data."""
import copy
import json
import pathlib
import tempfile
import unittest
from unittest import mock

import campaign


class DownloadValidity(unittest.TestCase):
    def setUp(self):
        env = mock.patch.dict(campaign.os.environ, {'NEXT_BUILD_MODE':'release','NEXT_NETWORK_DELAY_MS':'0','NEXT_DOWNLOAD_BLOCKS':'32768'})
        env.start()
        self.addCleanup(env.stop)

    def fixture(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.folder = pathlib.Path(self.temp.name)
        self.expected = campaign.plan()[:8]
        self.env = dict(kernel="6.18.60", sources={n:dict(effective_sources_sha256=n) for n in ["main","erase"]})
        self.rows = []
        for row in self.expected:
            m = dict(role="download-client", effective_sources_sha256=row["build"], kernel=self.env["kernel"],
                     **{k:row[k] for k in ["protocol","flows","blocks_per_flow","family","application"]},
                     seconds=10, client_cpu_seconds=5, client_rss_kib=4096, namespace="net:[1]",
                     before=dict(state="TunnelEstablished"), after=dict(state="TunnelEstablished"),
                     counters=dict(invalid_packets=0,outer_eof=0,outer_io_errors=0,decrypted_packets=100,tun_reads=20),
                     load=dict(effective_sources_sha256="main",bytes=row["bytes"],seconds=10,cpu_seconds=1,namespace="net:[1]",verified=True),
                     peer=dict(effective_sources_sha256="main",seconds=10,cpu_seconds=2,namespace="net:[2]",outer_namespace="net:[1]",handshakes=1))
            self.rows.append(dict(row, exit=0, metrics=[m]))

    def analyze(self):
        for name, obj in [("environment.json",self.env),("plan.json",self.expected)]:
            (self.folder/name).write_text(json.dumps(obj))
        (self.folder/"samples.jsonl").write_text("".join(json.dumps(r)+"\n" for r in self.rows))
        return campaign.analyze(self.folder)

    def test_plan_has_constant_bytes_balanced_order_and_all_protocols(self):
        rows=campaign.plan()
        self.assertEqual(len(rows),96)
        self.assertEqual(len({r["block"] for r in rows}),12)
        for row in rows:
            self.assertEqual(row["bytes"],65536*row["flows"]*row["blocks_per_flow"])
        self.assertIn("download-client",campaign.guest_script())

    def build_fixture(self, mode):
        with mock.patch.dict(campaign.os.environ, {'NEXT_BUILD_MODE':mode}):
            self.fixture()
        self.env['binaries'] = {'main':'1'*64, 'erase':'2'*64}
        self.env['cpu_identity'] = {'sha256':'3'*64}
        for row in self.rows:
            m=row['metrics'][0]
            m['binary_sha256']=self.env['binaries'][row['build']]
            m['peer']['binary_sha256']=m['load']['binary_sha256']=self.env['binaries']['main']
        for name in ['main','erase']:
            self.env['sources'][name]['build_configuration'] = dict(mode='release' if name=='main' else mode,
                binary_sha256=self.env['binaries'][name], rustflags=[])
        candidate=self.env['sources']['erase']['build_configuration']
        candidate['erasure_checked']=True
        candidate.update(gate_version=3,negative_control={'expected_rejection':True})
        if mode=='pgo':
            candidate.update(rustflags=['-Cprofile-use=/fresh/client.profdata'], profile=dict(path='/fresh/client.profdata',sha256='4'*64,instrumentation_absent=True,raw_files=[{'sha256':'5'*64}],
                training={'cases':[dict(name=f'{p}-{n}',flows=2 if n==0 else 4,verified=True) for p in ['udp','tcp','stealth'] for n in range(2)]}))
        else:
            candidate.update(rustflags=['-Ctarget-cpu=native'],cpu_identity=self.env['cpu_identity'].copy())
        return candidate

    def test_pgo_requires_final_binary_consumed_profile_and_separate_verified_training(self):
        for mutation in ['valid','instrumented','reused','missing_profile','unconsumed','evaluation_leak','failed_training','changed_helper']:
            with self.subTest(mutation=mutation):
                cfg=self.build_fixture('pgo')
                if mutation=='instrumented':cfg['rustflags'].append('-Cprofile-generate=/raw')
                elif mutation=='reused':self.env['binaries']['erase']=self.env['binaries']['main'];cfg['binary_sha256']=self.env['binaries']['main']
                elif mutation=='missing_profile':cfg['profile']['raw_files']=[]
                elif mutation=='unconsumed':cfg['rustflags']=[]
                elif mutation=='evaluation_leak':cfg['profile']['training']['cases'][0]['flows']=1
                elif mutation=='failed_training':cfg['profile']['training']['cases'][0]['verified']=False
                elif mutation=='changed_helper':self.rows[0]['metrics'][0]['peer']['binary_sha256']='2'*64
                self.assertEqual(bool(self.analyze()['errors']),mutation!='valid')

    def test_cpu_tuning_rejects_unmatched_cpu_or_absent_tuning(self):
        for mutation in ['valid','different_cpu','missing_flag']:
            with self.subTest(mutation=mutation):
                cfg=self.build_fixture('cpu')
                if mutation=='different_cpu':cfg['cpu_identity']['sha256']='9'*64
                elif mutation=='missing_flag':cfg['rustflags']=[]
                self.assertEqual(bool(self.analyze()['errors']),mutation!='valid')

    def test_combination_requires_both_independent_provenance_contracts(self):
        for mutation in ['valid','missing_cpu','different_cpu','missing_profile','failed_training']:
            with self.subTest(mutation=mutation):
                cfg=self.build_fixture('pgo')
                cfg.update(mode='pgo-cpu',cpu_identity=self.env['cpu_identity'].copy())
                cfg['rustflags'].append('-Ctarget-cpu=native')
                for expected,row in zip(self.expected,self.rows):
                    if row['build']=='erase':expected['build_mode']=row['build_mode']='pgo-cpu'
                if mutation=='missing_cpu':cfg['rustflags'].remove('-Ctarget-cpu=native')
                elif mutation=='different_cpu':cfg['cpu_identity']['sha256']='9'*64
                elif mutation=='missing_profile':cfg['profile']['raw_files']=[]
                elif mutation=='failed_training':cfg['profile']['training']['cases'][0]['verified']=False
                self.assertEqual(bool(self.analyze()['errors']),mutation!='valid')

    def test_single_build_cannot_hide_combined_flags(self):
        cfg=self.build_fixture('pgo');cfg['rustflags'].append('-Ctarget-cpu=native')
        self.assertTrue(self.analyze()['errors'])
        cfg=self.build_fixture('cpu');cfg['rustflags'].append('-Cprofile-use=/unexpected')
        self.assertTrue(self.analyze()['errors'])

    def test_focused_retest_has_explicit_nonempty_plan(self):
        with mock.patch.dict(campaign.os.environ,{"NEXT_DOWNLOAD_BLOCK":"stealth-v6-https-8"}):
            rows=campaign.plan()
            self.assertEqual(len(rows),8)
            self.assertEqual({r["sample"] for r in rows},set(range(89,97)))
        with mock.patch.dict(campaign.os.environ,{"NEXT_DOWNLOAD_BLOCK":"unknown"}):
            with self.assertRaises(AssertionError):campaign.plan()

    def test_network_scenario_requires_actual_setup_and_matching_metric(self):
        with mock.patch.dict(campaign.os.environ,{'NEXT_NETWORK_DELAY_MS':'5'}):self.fixture()
        for row in self.rows:
            row['stdout']='NEXT_NETEM 5\nqdisc netem 8001: root limit 1000 delay 5ms'
            row['metrics'][0]['network_delay_ms']=5
        self.assertFalse(self.analyze()['errors'])
        self.rows[0]['stdout']=''
        self.assertTrue(self.analyze()['errors'])
        self.rows[0]['stdout']='NEXT_NETEM 5\nqdisc netem root delay 5ms'
        self.rows[0]['metrics'][0]['network_delay_ms']=0
        self.assertTrue(self.analyze()['errors'])

    def test_capacity_payload_is_explicit_and_pacing_stays_fixed(self):
        with mock.patch.dict(campaign.os.environ,{'NEXT_DOWNLOAD_BLOCKS':'65536','NEXT_DOWNLOAD_RATE_BPS':'0'}):
            self.assertTrue(all(r['bytes']==4*1024**3 for r in campaign.plan()))
        with mock.patch.dict(campaign.os.environ,{'NEXT_DOWNLOAD_BLOCKS':'65536','NEXT_DOWNLOAD_RATE_BPS':'200000000'}):
            self.assertTrue(all(r['bytes']==128*1024**2 for r in campaign.plan()))

    def helper_fixture(self):
        with mock.patch.dict(campaign.os.environ,{'NEXT_PEER_HELPER_FIXED':'reader'}):self.fixture()
        self.env['helper_cpu_layouts']={'reader':dict(client=[2],wire=[0],reader=[3],server=[1],load=[1],shared_load_server_cpu=True,auxiliary=[0,1,3])}
        for row in self.rows:
            m=row['metrics'][0]
            m.update(helper_mode='reader',client_affinity_cpus=[2],shared_load_server_cpu=True,cpu_ticks_before={f'cpu{n}':[0]*8 for n in [0,1,3]},cpu_ticks_after={f'cpu{n}':[20,0,0,80,0,0,0,0] for n in [0,1,3]})
            m['load']['helper_mode']='reader'
            m['peer'].update(helper_mode='reader',wire_cpu_seconds=2,reader_cpu_seconds=2,server_cpu_seconds=1,cpu_seconds=5,reader_frames=100,inplace_packets=100,reader_cpu_index=3,wire_affinity_cpus=[0],server_affinity_cpus=[1])

    def test_shared_helper_core_and_asynchronous_kernel_work_cannot_hide_a_bottleneck(self):
        self.helper_fixture();self.assertFalse(self.analyze()['errors']);self.assertTrue(self.analyze()['blocks'][0]['capacity_ranking'])
        for row in self.rows:
            row['metrics'][0]['load']['cpu_seconds']=5
            row['metrics'][0]['peer'].update(server_cpu_seconds=5,cpu_seconds=9)
        self.assertTrue(self.analyze()['blocks'][0]['auxiliary_limited'])
        self.helper_fixture()
        self.rows[0]['metrics'][0]['cpu_ticks_after']['cpu3']=[90,0,0,10,0,0,0,0]
        self.assertTrue(self.analyze()['blocks'][0]['auxiliary_limited'])

    def test_helper_claim_requires_actual_strategy_placement_and_accounting(self):
        for mutation in ['wrong_strategy','wrong_cpu','missing_reader','nan_reader','missing_ticks','short_ticks','changed_client']:
            self.helper_fixture();m=self.rows[0]['metrics'][0]
            if mutation=='wrong_strategy':m['peer']['helper_mode']='legacy'
            elif mutation=='wrong_cpu':m['peer']['reader_cpu_index']=2
            elif mutation=='missing_reader':m['peer']['reader_cpu_seconds']=0
            elif mutation=='nan_reader':m['peer']['reader_cpu_seconds']=float('nan')
            elif mutation=='missing_ticks':del m['cpu_ticks_before']
            elif mutation=='short_ticks':m['cpu_ticks_after']['cpu0'].pop()
            elif mutation=='changed_client':m['client_affinity_cpus']=[2,3]
            self.assertTrue(self.analyze()['errors'])

    def test_valid_aa_and_comparison(self):
        self.fixture()
        result=self.analyze()
        self.assertFalse(result["errors"])
        self.assertTrue(result["blocks"][0]["capacity_ranking"])
        self.assertAlmostEqual(result["blocks"][0]["result"]["Mbps"]["change_percent"],0)

    def test_rate_controlled_plan_is_balanced_and_explicit(self):
        with mock.patch.dict(campaign.os.environ,{"NEXT_DOWNLOAD_RATE_BPS":"200000000"}):
            rows=campaign.plan()
            self.assertEqual(len(rows),96)
            for row in rows:
                self.assertEqual(row["bytes"],128*1024*1024)
                self.assertEqual(row["bytes"],65536*row["flows"]*row["blocks_per_flow"])
                self.assertEqual(row["offered_rate_bps"],200000000)
            self.assertIn("export NEXT_DOWNLOAD_RATE_BPS=200000000",campaign.guest_script())
        with mock.patch.dict(campaign.os.environ,{"NEXT_DOWNLOAD_RATE_BPS":"-1"}):
            with self.assertRaises(AssertionError):campaign.plan()

    def test_rate_controlled_samples_cannot_claim_maximum_capacity(self):
        self.fixture()
        rate=200000000
        for expected,row in zip(self.expected,self.rows):
            expected["offered_rate_bps"]=row["offered_rate_bps"]=rate
            m=row["metrics"][0]
            m["offered_rate_bps"]=rate
            seconds=row["bytes"]*8/rate
            m["seconds"]=m["load"]["seconds"]=m["peer"]["seconds"]=seconds
        result=self.analyze()
        self.assertFalse(result["errors"])
        self.assertTrue(result["blocks"][0]["rate_controlled"])
        self.assertTrue(result["blocks"][0]["AA_pass"])
        self.assertTrue(result["blocks"][0]["timing_pass"])
        self.assertFalse(result["blocks"][0]["auxiliary_limited"])
        self.assertFalse(result["blocks"][0]["capacity_ranking"])
        self.rows[0]["metrics"][0]["offered_rate_bps"]=0
        self.assertTrue(self.analyze()["errors"])
        self.rows[0]["metrics"][0]["offered_rate_bps"]=rate
        self.rows[0]["metrics"][0]["load"]["seconds"]*=2
        self.assertTrue(self.analyze()["errors"])

    def test_unplanned_rate_limit_cannot_pass_as_unpaced_capacity(self):
        self.fixture()
        self.rows[0]["metrics"][0]["offered_rate_bps"]=200000000
        self.assertTrue(self.analyze()["errors"])

    def test_tun_comparison_keeps_controls_raw_and_requires_actual_coalescing(self):
        with mock.patch.dict(campaign.os.environ,{"NEXT_DOWNLOAD_TUN_MODE":"compare"}):
            rows=campaign.plan()
            self.assertEqual(len(rows),96)
            for row in rows:
                self.assertEqual(row["tun_mode"],"vnet" if row["build"]=="erase" else "raw")
            self.assertIn("DOWNLOAD_CANDIDATE_VNET_SMOKE",campaign.guest_script())
        self.fixture()
        for expected,row in zip(self.expected,self.rows):
            mode="vnet" if row["build"]=="erase" else "raw"
            expected["tun_mode"]=row["tun_mode"]=row["metrics"][0]["tun_mode"]=mode
            row["metrics"][0]["counters"].update(tun_coalesced_packets=32 if mode=="vnet" else 0,tun_max_write_batch=16,tun_offload_fallbacks=0)
        self.assertFalse(self.analyze()["errors"])
        candidate=next(r for r in self.rows if r["build"]=="erase")
        for key,value in [("tun_coalesced_packets",0),("tun_max_write_batch",1),("tun_max_write_batch",33),("tun_offload_fallbacks",1)]:
            counters=candidate["metrics"][0]["counters"]
            saved=counters[key]
            counters[key]=value
            self.assertTrue(self.analyze()["errors"])
            counters[key]=saved
        candidate["metrics"][0]["tun_mode"]="raw"
        self.assertTrue(self.analyze()["errors"])

    def test_unplanned_vnet_mode_cannot_pass_as_raw_control(self):
        self.fixture()
        self.rows[0]["metrics"][0]["tun_mode"]="vnet"
        self.assertTrue(self.analyze()["errors"])

    def test_parallel_plan_matches_tun_and_cpu_contract_and_screens_shared_helpers(self):
        with mock.patch.dict(campaign.os.environ,{"NEXT_DOWNLOAD_TUN_MODE":"vnet","NEXT_DOWNLOAD_CLIENT_CORES":"2","NEXT_DOWNLOAD_PARALLEL":"1"}):
            self.fixture()
            self.assertTrue(all(r["tun_mode"]=="vnet" and r["client_cores"]==2 for r in campaign.plan()))
            script=campaign.guest_script()
            self.assertIn("NEXT_CLIENT_CPUS=2,3 NEXT_SERVER_CPUS=0",script)
            self.assertIn("export NEXT_TUN_RECEIVE_MODE=vnet",script)
        self.env["cpu_layout"]={"client":[2,3]}
        for row in self.rows:
            m=row["metrics"][0]
            m.update(tun_mode="vnet",client_affinity_cpus=[2,3],shared_peer_server_cpu=True)
            m["counters"].update(tun_coalesced_packets=32,tun_max_write_batch=16,tun_offload_fallbacks=0)
            if row["build"]=="erase":
                m["counters"].update(parallel_batches=1,parallel_helper_packets=8,parallel_owner_packets=8,parallel_max_batch=16,parallel_owner_cpu=2,parallel_helper_cpu=3)
            m["peer"].update(cpu_seconds=9,wire_cpu_seconds=4.5,server_cpu_seconds=4.5)
        result=self.analyze()
        self.assertFalse(result["errors"])
        self.assertTrue(result["blocks"][0]["auxiliary_limited"])
        self.assertFalse(result["blocks"][0]["capacity_ranking"])
        candidate=next(r for r in self.rows if r["build"]=="erase")["metrics"][0]
        candidate["client_affinity_cpus"]=[0,1]
        self.assertTrue(self.analyze()["errors"])
        candidate["client_affinity_cpus"]=[2,3]
        candidate["counters"]["parallel_helper_cpu"]=2
        self.assertTrue(self.analyze()["errors"])
        candidate["counters"]["parallel_helper_cpu"]=3
        for key,bad in [("parallel_batches",0),("parallel_helper_packets",0),("parallel_owner_packets",0),("parallel_max_batch",4),("parallel_max_batch",33)]:
            saved=candidate["counters"][key];candidate["counters"][key]=bad
            self.assertTrue(self.analyze()["errors"])
            candidate["counters"][key]=saved
        with mock.patch.dict(campaign.os.environ,{"NEXT_DOWNLOAD_CLIENT_CORES":"1","NEXT_DOWNLOAD_PARALLEL":"1"}):
            with self.assertRaises(AssertionError):campaign.plan()

    def test_wrong_identity_bypassed_tunnel_payload_failure_and_outer_errors(self):
        self.fixture()
        for name, value in [("effective_sources_sha256","wrong"),("namespace","net:[2]"),
                            ("load.verified",False),("load.bytes",1),("counters.outer_eof",1),
                            ("peer.effective_sources_sha256","erase"),("kernel","7.0.0")]:
            with self.subTest(name=name):
                saved=copy.deepcopy(self.rows)
                target=self.rows[0]["metrics"][0]
                parts=name.split('.')
                for part in parts[:-1]: target=target[part]
                target[parts[-1]]=value
                self.assertTrue(self.analyze()["errors"])
                self.rows=saved

    def test_failed_and_missing_samples_not_silently_excluded(self):
        self.fixture()
        self.rows[0]["exit"]=124
        self.assertTrue(self.analyze()["errors"])
        self.rows.pop()
        self.assertEqual(self.analyze()["blocks"][0]["validity"],"invalid/incomplete")

    def test_noisy_controls_cannot_rank(self):
        self.fixture()
        self.rows[1]["metrics"][0]["client_cpu_seconds"]*=1.5
        result=self.analyze()
        self.assertFalse(result["errors"])
        self.assertFalse(result["blocks"][0]["AA_pass"])
        self.assertFalse(result["blocks"][0]["capacity_ranking"])

    def test_auxiliary_saturation_disqualifies_capacity_not_correctness(self):
        self.fixture()
        self.rows[0]["metrics"][0]["peer"]["cpu_seconds"]=9
        result=self.analyze()
        self.assertFalse(result["errors"])
        self.assertTrue(result["blocks"][0]["auxiliary_limited"])
        self.assertFalse(result["blocks"][0]["capacity_ranking"])

    def test_short_or_misaligned_intervals_are_inconclusive(self):
        self.fixture()
        for seconds in [1,11]:
            with self.subTest(seconds=seconds):
                self.rows[0]["metrics"][0]["load"]["seconds"]=seconds
                result=self.analyze()
                self.assertFalse(result["errors"])
                self.assertFalse(result["blocks"][0]["timing_pass"])
                self.assertFalse(result["blocks"][0]["capacity_ranking"])

    def test_nonfinite_auxiliary_metrics_cannot_hide_saturation(self):
        self.fixture()
        for value in [float("nan"),float("inf"),-1,0]:
            self.rows[0]["metrics"][0]["peer"]["cpu_seconds"]=value
            self.assertTrue(self.analyze()["errors"])

    def test_separate_peer_roles_use_individual_core_budgets(self):
        self.fixture()
        for row in self.rows:
            row["metrics"][0]["peer"].update(cpu_seconds=10,wire_cpu_seconds=5,server_cpu_seconds=5)
        self.assertTrue(self.analyze()["blocks"][0]["capacity_ranking"])
        self.rows[0]["metrics"][0]["peer"]["wire_cpu_seconds"]=9
        self.assertFalse(self.analyze()["blocks"][0]["capacity_ranking"])
        self.rows[0]["metrics"][0]["peer"]["wire_cpu_seconds"]=float("nan")
        self.assertTrue(self.analyze()["errors"])


if __name__ == "__main__":
    unittest.main()
