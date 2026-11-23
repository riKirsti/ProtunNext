"""Validate the scheduling, patch boundaries and paired measurement decision."""

import datetime as dt
import unittest
from copy import deepcopy
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

    def test_framing_cannot_supply_the_linux_download_benefit(self):
        framing = {"metrics": {"framing_payload_mbps": {"decision": "passed", "repeatable_gain": True}}}
        workload = {"sample_quality_passed": True, "throughput_headroom": False, "metrics": {
            metric: {"decision": "passed", "repeatable_gain": False} for metric in maintenance.download.METRICS
        }}
        tunnels = {"udp/v4/http/1": workload}
        decide = maintenance.measurement_decision
        self.assertEqual(decide(framing, tunnels, True), "no demonstrated download gain")
        workload["metrics"]["download_mbps"]["repeatable_gain"] = True
        self.assertEqual(decide(framing, tunnels, True), "no demonstrated download gain")
        workload["metrics"]["cpu_ns_per_payload_byte"]["repeatable_gain"] = True
        self.assertEqual(decide(framing, tunnels, True), "passed")
        workload["sample_quality_passed"] = False
        self.assertEqual(decide(framing, tunnels, True), "measurement inconclusive")

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

    def test_refresh_is_for_disjoint_documentation_cleanup_only(self):
        files = {"README.md"}
        self.assertTrue(maintenance.can_refresh("cleanup", files, {"src/framing.rs"}))
        for drift in (files, {"Cargo.lock"}, {"vendor/snapshot.rs"}, {".github/workflows/ci.yml"}):
            self.assertFalse(maintenance.can_refresh("cleanup", files, drift))
        self.assertFalse(maintenance.can_refresh("performance", files, {"README.md"}))
        self.assertFalse(maintenance.can_refresh("refactor", {"src/engine.rs"}, {"README.md"}))


if __name__ == "__main__":
    unittest.main()
