"""Reject fixtures that can manufacture download gains or hide invalid data."""
import copy
import json
import pathlib
import tempfile
import unittest

import campaign


class DownloadValidity(unittest.TestCase):
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

    def test_valid_aa_and_comparison(self):
        self.fixture()
        result=self.analyze()
        self.assertFalse(result["errors"])
        self.assertTrue(result["blocks"][0]["capacity_ranking"])
        self.assertAlmostEqual(result["blocks"][0]["result"]["Mbps"]["change_percent"],0)

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


if __name__ == "__main__":
    unittest.main()
