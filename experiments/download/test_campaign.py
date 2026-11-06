"""Reject fixtures that can manufacture download gains or hide invalid data."""
import copy
import json
import pathlib
import tempfile
import unittest
from unittest import mock

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

    def test_focused_retest_has_explicit_nonempty_plan(self):
        with mock.patch.dict(campaign.os.environ,{"NEXT_DOWNLOAD_BLOCK":"stealth-v6-https-8"}):
            rows=campaign.plan()
            self.assertEqual(len(rows),8)
            self.assertEqual({r["sample"] for r in rows},set(range(89,97)))
        with mock.patch.dict(campaign.os.environ,{"NEXT_DOWNLOAD_BLOCK":"unknown"}):
            with self.assertRaises(AssertionError):campaign.plan()

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

    def budget_fixture(self, target="travel"):
        self.fixture()
        self.env["target_class"] = target
        for row in self.rows:
            row["metrics"][0].update(userspace_memory_scope="per_tunnel_peak",
                                     tunnel_count=1,
                                     userspace_memory_method="single-tunnel process VmHWM; includes setup and fixture overhead",
                                     userspace_peak_MiB_per_tunnel=4)

    def kernel_fixture(self):
        # Synthetic analyzer input only, never reported as a measured benchmark.
        socket = dict.fromkeys(campaign.KERNEL_SOCKET_FIELDS, 0)
        socket.update(rcvbuf_limit_bytes=8192, sndbuf_limit_bytes=8192)
        report = dict(method="SO_MEMINFO", scope="client_inet_sockets", coverage="boundary_snapshots",
                      before=dict(sockets=[socket], errors=[]), after=dict(sockets=[socket], errors=[]))
        for row in self.rows:
            row["metrics"][0]["kernel_buffers"] = copy.deepcopy(report)

    def candidate_metrics(self):
        return [r["metrics"][0] for r in self.rows if r["role"] == "compare" and r["build"] == "erase"]

    def test_cpu_budget_is_per_byte_with_inclusive_25_percent_boundary(self):
        self.budget_fixture()
        for m in self.candidate_metrics():
            m.update(client_cpu_seconds=6.25, seconds=5)
            m["load"]["seconds"] = 5
            m["peer"]["seconds"] = 5
        budget = self.analyze()["blocks"][0]["resource_budget"]
        self.assertAlmostEqual(budget["cpu_per_byte"]["ratio"], 1.25)
        self.assertEqual(budget["cpu_per_byte"]["status"], "pass")
        for m in self.candidate_metrics():
            m["client_cpu_seconds"] = 6.251
        block = self.analyze()["blocks"][0]
        self.assertEqual(block["resource_budget"]["cpu_per_byte"]["status"], "fail")
        self.assertEqual(block["resource_budget"]["status"], "fail")
        self.assertFalse(block["optimization_eligible"])

    def test_userspace_increment_caps_are_per_target_and_inclusive(self):
        for target, cap in [("travel", 8), ("desktop_server", 32)]:
            with self.subTest(target=target):
                self.budget_fixture(target)
                for m in self.candidate_metrics():
                    m["userspace_peak_MiB_per_tunnel"] = 4 + cap
                    m["client_rss_kib"] = (4 + cap) * 1024
                budget = self.analyze()["blocks"][0]["resource_budget"]
                self.assertEqual(budget["userspace_memory"]["status"], "pass")
                self.assertEqual(budget["userspace_memory"]["additional_MiB_per_tunnel"], cap)
                self.candidate_metrics()[0]["userspace_peak_MiB_per_tunnel"] += .001
                self.candidate_metrics()[0]["client_rss_kib"] += .001 * 1024
                budget = self.analyze()["blocks"][0]["resource_budget"]
                self.assertEqual(budget["userspace_memory"]["status"], "fail")
                self.assertEqual(budget["status"], "fail")

    def test_total_process_rss_does_not_prove_per_tunnel_budget(self):
        self.fixture()
        self.env["target_class"] = "travel"
        block = self.analyze()["blocks"][0]
        self.assertTrue(block["capacity_ranking"])
        self.assertEqual(block["resource_budget"]["cpu_per_byte"]["status"], "pass")
        self.assertEqual(block["resource_budget"]["userspace_memory"]["status"], "unverified")
        self.assertFalse(block["optimization_eligible"])

    def test_unknown_target_is_not_assumed_desktop(self):
        self.budget_fixture("unknown")
        memory = self.analyze()["blocks"][0]["resource_budget"]["userspace_memory"]
        self.assertIsNone(memory["maximum_additional_MiB_per_tunnel"])
        self.assertEqual(memory["status"], "unverified")

    def test_missing_kernel_collection_cannot_be_treated_as_zero_or_pass(self):
        self.budget_fixture()
        budget = self.analyze()["blocks"][0]["resource_budget"]
        self.assertEqual(budget["userspace_memory"]["status"], "pass")
        self.assertEqual(budget["kernel_buffers"]["baseline"], [None, None])
        self.assertEqual(budget["kernel_buffers"]["status"], "unverified")
        self.assertEqual(budget["status"], "unverified")

    def test_budget_pass_requires_scoped_kernel_data_and_real_improvement_is_separate(self):
        self.budget_fixture()
        self.kernel_fixture()
        self.env["sources"]["main"]["runtime_sources_sha256"] = "base"
        self.env["sources"]["erase"]["runtime_sources_sha256"] = "different"
        result = self.analyze()["blocks"][0]
        self.assertEqual(result["resource_budget"]["status"], "pass")
        self.assertFalse(result["optimization_eligible"])
        for m in self.candidate_metrics():
            m["seconds"] = m["load"]["seconds"] = m["peer"]["seconds"] = 5
        self.assertTrue(self.analyze()["blocks"][0]["optimization_eligible"])
        for m in self.candidate_metrics():
            m["client_cpu_seconds"] = 7
        self.assertFalse(self.analyze()["blocks"][0]["optimization_eligible"])

    def test_incomplete_or_invalid_kernel_data_never_passes(self):
        self.budget_fixture()
        self.kernel_fixture()
        for field, value in [("rmem_alloc_bytes", float("nan")), ("drops_packets", -1),
                             ("sndbuf_limit_bytes", True)]:
            with self.subTest(field=field):
                self.kernel_fixture()
                self.candidate_metrics()[0]["kernel_buffers"]["after"]["sockets"][0][field] = value
                self.assertEqual(self.analyze()["blocks"][0]["resource_budget"]["status"], "unverified")
        self.kernel_fixture()
        self.candidate_metrics()[0]["kernel_buffers"]["before"]["errors"] = ["unsupported"]
        self.assertEqual(self.analyze()["blocks"][0]["resource_budget"]["status"], "unverified")

    def test_invalid_or_unscoped_per_tunnel_memory_is_rejected(self):
        self.budget_fixture()
        for value in [float("nan"), float("inf"), -1, True]:
            with self.subTest(value=value):
                self.candidate_metrics()[0]["userspace_peak_MiB_per_tunnel"] = value
                self.assertTrue(self.analyze()["errors"])
        self.candidate_metrics()[0].update(userspace_peak_MiB_per_tunnel=4,
                                           userspace_memory_scope="whole_process")
        self.assertTrue(self.analyze()["errors"])

    def test_same_runtime_is_never_a_promotable_optimization(self):
        self.budget_fixture()
        for source in self.env["sources"].values():
            source["runtime_sources_sha256"] = "identical"
        result = self.analyze()
        self.assertTrue(result["same_runtime_sources"])
        self.assertFalse(result["blocks"][0]["optimization_eligible"])

    def test_published_summary_distinguishes_capacity_and_resource_budget(self):
        self.fixture()
        self.analyze()
        campaign.publish(self.folder)
        text = (self.folder / "summary.md").read_text()
        self.assertIn("Budget status", text)
        self.assertIn("Additional userspace MiB/tunnel", text)
        self.assertIn("Kernel buffers", text)
        self.assertIn("unverified", text)


if __name__ == "__main__":
    unittest.main()
