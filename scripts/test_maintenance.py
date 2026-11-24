"""Validate the scheduling, patch boundaries and paired measurement decision."""

import datetime as dt
import os
import unittest
from copy import deepcopy
from pathlib import Path
from unittest.mock import patch

import maintenance


class Maintenance(unittest.TestCase):
    def test_calendar_cycle_across_february_and_year_boundary(self):
        for month in (2, 3):
            day = dt.date(2027, month, 25)
            self.assertEqual(maintenance.cycle_date(day, True), day)
        delayed = dt.date(2027, 1, 1)
        self.assertEqual(maintenance.cycle_date(delayed, True), dt.date(2026, 12, 25))
        self.assertEqual(maintenance.cycle_date(delayed, False), delayed)
        workflows = maintenance.ROOT / ".github" / "workflows"
        for filename, cron in (("maintenance.yml", "'0 2 25 * *'"),
                               ("upstream.yml", "'0 1 * * 0'")):
            lines = workflows.joinpath(filename).read_text(encoding="utf-8").splitlines()
            self.assertEqual([line.strip() for line in lines if "- cron:" in line],
                             [f"- cron: {cron}"])

    def test_stages_leave_version_and_workflow_ownership_with_upstream_monitor(self):
        for stage in maintenance.STAGES:
            for path in ("Cargo.toml", "Cargo.lock", "tests/fixtures/compatibility.json",
                         ".github/workflows/release.yml", "src/requirements.json"):
                for change in ("M", "A", "D"):
                    self.assertFalse(maintenance.permitted(stage, path, change))
        self.assertTrue(maintenance.permitted("performance", "src/framing.rs", "M"))
        self.assertFalse(maintenance.permitted("performance", "src/linux.rs", "M"))
        self.assertTrue(maintenance.permitted("refactor", "src/linux.rs", "M"))
        self.assertFalse(maintenance.permitted("cleanup", "docs/building.md", "D"))
        self.assertTrue(maintenance.permitted("cleanup", "src/obsolete.rs", "D"))
        self.assertTrue(maintenance.permitted("cleanup", "README.md", "M"))
        self.assertTrue(maintenance.permitted_path("README.md"))

    def test_measurement_spread_and_fixed_regression_margin(self):
        baseline = {metric: 100 for metric in maintenance.METRICS}
        control = [(baseline, baseline)] * 4
        faster = {**baseline, "framing_payload_mbps": 110}
        costlier = {**faster, "peak_rss_bytes": 118}
        noisy = {**baseline, "framing_payload_mbps": 110}
        compare = maintenance.compare_samples
        directions = maintenance.METRICS
        fast = compare(control, [(baseline, faster)] * 7 + [(baseline, baseline)], directions)
        self.assertTrue(fast["framing_payload_mbps"]["repeatable_gain"])
        regression = compare(control, [(baseline, costlier)] * 8, directions)
        self.assertEqual(regression["peak_rss_bytes"]["decision"], "regression")
        invalid = compare([(baseline, noisy)] * 4, [(baseline, faster)] * 8, directions)
        self.assertEqual(invalid["framing_payload_mbps"]["decision"], "measurement inconclusive")
        with self.assertRaises(ValueError):
            compare(control, [(baseline, baseline)], directions)

    def test_qualification_requires_the_declared_gain_and_shared_regression_evidence(self):
        stable = {"decision": "passed", "repeatable_gain": False}
        framing = {"metrics": {metric: stable.copy() for metric in maintenance.METRICS}}
        downloads = {
            f"{protocol}/{family}/{app}/{flows}": {
                "sample_quality_passed": True, "throughput_headroom": True,
                "metrics": {metric: stable.copy() for metric in maintenance.download.METRICS},
            }
            for protocol in maintenance.download.TRANSPORTS
            for family, app, flows in maintenance.WORKLOADS
        }
        hypothesis = {"platform": "linux-arm64", "workload": "udp/v4/http/1",
                      "metric": "download_mbps"}
        decide = maintenance.qualification_decision
        self.assertEqual(decide(framing, downloads, "linux-x64", hypothesis), "passed")
        framing["metrics"]["framing_payload_mbps"]["repeatable_gain"] = True
        sample = downloads[hypothesis["workload"]]
        sample["metrics"]["cpu_ns_per_payload_byte"]["repeatable_gain"] = True
        self.assertEqual(decide(framing, downloads, "linux-arm64", hypothesis),
                         "no demonstrated download gain")
        sample["metrics"]["download_mbps"]["repeatable_gain"] = True
        sample["throughput_headroom"] = False
        self.assertEqual(decide(framing, downloads, "linux-arm64", hypothesis),
                         "no demonstrated download gain")
        sample["throughput_headroom"] = True
        self.assertEqual(decide(framing, downloads, "linux-arm64", hypothesis), "passed")

        unaffected_gain = deepcopy(downloads)
        unaffected_gain[hypothesis["workload"]]["metrics"]["download_mbps"]["decision"] = "regression"
        self.assertEqual(decide(framing, unaffected_gain, "linux-x64", hypothesis), "regression")
        sample["sample_quality_passed"] = False
        self.assertEqual(decide(framing, downloads, "linux-arm64", hypothesis),
                         "measurement inconclusive")
        del downloads[hypothesis["workload"]]
        self.assertEqual(decide(framing, downloads, "linux-arm64", hypothesis),
                         "measurement inconclusive")

    def test_linux_rss_floor_clears_launch_spread_and_still_detects_growth(self):
        mib = 2**20
        baseline = {"peak_rss_bytes": 10 * mib}
        launch_spread = {"peak_rss_bytes": 11.5 * mib}
        higher_usage = {"peak_rss_bytes": 13 * mib}
        control = [(baseline, launch_spread)] + [(baseline, baseline)] * 3
        directions = {"peak_rss_bytes": -1}
        stable = maintenance.compare_samples(control, [(baseline, launch_spread)] * 8, directions,
                                             rss_floor=maintenance.DOWNLOAD_RSS_FLOOR)
        regression = maintenance.compare_samples(control, [(baseline, higher_usage)] * 8, directions,
                                                 rss_floor=maintenance.DOWNLOAD_RSS_FLOOR)
        self.assertEqual(stable["peak_rss_bytes"]["decision"], "passed")
        self.assertEqual(regression["peak_rss_bytes"]["decision"], "regression")

    def test_retryable_results_do_not_consume_cycle(self):
        report = {"stages": {stage: {"status": "no change"} for stage in maintenance.STAGES}}
        self.assertTrue(maintenance.cycle_complete(report, []))
        first = {"stages": {"performance": {"decision": "operational failure"}}}
        self.assertFalse(maintenance.cycle_complete(report, [first]))
        report["stages"]["performance"]["status"] = "no verified agent output"
        self.assertFalse(maintenance.cycle_complete(report, []))

    def test_retry_reuses_the_original_patch_identity(self):
        report = {"stages": {"cleanup": {"sha256": "patch"}}}
        first = {"commit": "baseline", "platform": "linux-x64", "architecture": "x86_64",
                 "stages": {"cleanup": {"sha256": "patch", "decision": "operational failure"}}}
        retry = deepcopy(first)
        retry["stages"]["cleanup"]["decision"] = "passed"
        with patch.object(maintenance, "read_result", return_value=retry):
            candidate = deepcopy(first)
            maintenance.merge_retries(report, [candidate])
            self.assertTrue(candidate["stages"]["cleanup"]["retried"])
            retry["stages"]["cleanup"]["sha256"] = "different patch"
            with self.assertRaisesRegex(ValueError, "retry results do not match"):
                maintenance.merge_retries(report, [first])

    def test_download_metrics_reject_wrong_fixed_peer_or_bypassed_tunnel(self):
        record = {"role": "download-client", "effective_sources_sha256": "candidate",
                  "protocol": "stealth", "family": "v4", "application": "http",
                  "flows": 1, "blocks_per_flow": 32768, "seconds": 8, "client_cpu_seconds": 7,
                  "client_rss_kib": 8000, "namespace": "client",
                  "before": {"state": "TunnelEstablished"}, "after": {"state": "TunnelEstablished"},
                  "counters": {"invalid_packets": 0, "outer_io_errors": 0, "outer_eof": 0,
                               "decrypted_packets": 10, "tun_reads": 10},
                  "peer": {"effective_sources_sha256": "baseline", "outer_namespace": "client",
                           "namespace": "peer", "handshakes": 1, "tls": "Some(TLSv1_3)",
                           "seconds": 8, "wire_cpu_seconds": 6, "server_cpu_seconds": 1},
                  "load": {"effective_sources_sha256": "baseline", "namespace": "client",
                           "verified": True, "bytes": 2**31, "seconds": 8, "cpu_seconds": 2}}
        args = ("candidate", "baseline", "stealth", "v4", "http", 1, 32768)
        metrics = maintenance.download.metrics
        self.assertEqual(metrics(record, *args)["verified_payload_bytes"], 2**31)
        bad = deepcopy(record)
        bad["peer"]["effective_sources_sha256"] = "candidate"
        with self.assertRaisesRegex(ValueError, "identity"):
            metrics(bad, *args)
        record["peer"]["namespace"] = "client"
        with self.assertRaisesRegex(ValueError, "routing"):
            metrics(record, *args)

    def test_qualified_stage_does_not_depend_on_other_stages(self):
        results = [
            {"platform": runner["platform"], "stages": {
                "performance": {"decision": "measurement inconclusive"},
                "refactor": {"decision": "passed"},
            }} for runner in maintenance.RUNNERS
        ]
        proposal = {"sha256": "fixture"}
        self.assertTrue(maintenance.stage_qualified("refactor", proposal, results))
        self.assertFalse(maintenance.stage_qualified("performance", proposal, results))
        self.assertFalse(maintenance.stage_qualified("refactor", proposal, results[:-1]))

    def test_screen_distinguishes_gain_noise_and_wrong_workload(self):
        baseline = {metric: 100 for metric in maintenance.METRICS}
        steady = [(baseline, baseline)] * 4
        faster = {**baseline, "framing_payload_mbps": 110}
        normal = maintenance.compare_samples(steady, [(baseline, faster)] * 8, maintenance.METRICS)
        noisy = maintenance.compare_samples([(baseline, faster)] * 4,
                                            [(baseline, faster)] * 8, maintenance.METRICS)
        decide = maintenance.screen_decision
        self.assertEqual(decide("framing", {"metrics": normal}, "framing_payload_mbps"), "passed")
        self.assertEqual(decide("framing", {"metrics": normal}, "cpu_ns_per_payload_byte"),
                         "no demonstrated framing gain")
        self.assertEqual(decide("framing", {"metrics": noisy}, "framing_payload_mbps"),
                         "measurement inconclusive")
        self.assertTrue(maintenance.needs_measurement({"metrics": noisy}))
        self.assertFalse(maintenance.needs_measurement({"metrics": normal}))
        notes = "Platform: macos\nWorkload: framing\nMetric: framing_payload_mbps\nHypothesis: Remove a division."
        self.assertEqual(maintenance.performance_hypothesis(notes, {"src/framing.rs"})["workload"],
                         "framing")
        with self.assertRaisesRegex(maintenance.CheckFailed, "does not match"):
            maintenance.performance_hypothesis(notes, {"src/stream.rs"})
        self.assertEqual(maintenance.performance_hypothesis(notes, {"src/framing.rs"})["platform"],
                         "macos")
        download_notes = ("Platform: linux-arm64\nWorkload: udp/v4/http/1\n"
                          "Metric: download_mbps\nHypothesis: Reduce packet copying.")
        self.assertEqual(maintenance.performance_hypothesis(download_notes, {"src/packet.rs"})["platform"],
                         "linux-arm64")
        with self.assertRaisesRegex(maintenance.CheckFailed, "does not match"):
            maintenance.performance_hypothesis(download_notes.replace("linux-arm64", "windows"),
                                               {"src/packet.rs"})


    def test_preflight_formats_once_and_rejects_new_paths(self):
        with (patch.object(maintenance.hosted, "apply_patch"),
              patch.object(maintenance, "changed", return_value={"src/framing.rs"}),
              patch.object(maintenance.hosted, "git", return_value=b"formatted"),
              patch.object(maintenance, "run_check") as check):
            self.assertEqual(maintenance.preflight_patch("refactor", b"raw", Path("."), {}),
                             b"formatted")
            self.assertEqual([call.args[0] for call in check.call_args_list],
                             ["formatting", "privacy and links", "vendor", "compatibility"])
        with (patch.object(maintenance.hosted, "apply_patch"),
              patch.object(maintenance, "changed", side_effect=[
                  {"src/framing.rs"}, {"src/framing.rs", "src/packet.rs"}]),
              patch.object(maintenance.hosted, "git"), patch.object(maintenance, "run_check")):
            with self.assertRaisesRegex(maintenance.CheckFailed, "outside the original patch"):
                maintenance.preflight_patch("refactor", b"raw", Path("."), {})

    def test_verified_maintenance_proposal_stays_a_draft(self):
        from unittest.mock import Mock
        head = "a" * 40
        branch = "maintenance/review/2026-11-25/refactor-bbbbbbbbbbbb"
        pr = {"draft": True, "head": {"sha": head, "ref": branch}}
        finish = Mock()
        finish.main_sha.return_value = "base"
        finish.api.return_value = pr
        finish.fetch.return_value = head
        finish.git.return_value = "base"
        with (patch.object(maintenance, "load_script", return_value=finish),
              patch.dict(os.environ, {"MAINTENANCE_DRAFTS": '{"42":"' + head + '"}'})):
            maintenance.verify_drafts()
        finish.check.assert_called_once_with(branch, head)
        self.assertEqual(finish.verification_status.call_args.args[:2], (head, "success"))
        self.assertTrue(pr["draft"])
        finish.merge.assert_not_called()


if __name__ == "__main__":
    unittest.main()
