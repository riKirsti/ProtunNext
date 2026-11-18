"""Validate the scheduling, patch boundaries and paired measurement decision."""

import datetime as dt
import unittest

import maintenance


class Maintenance(unittest.TestCase):
    def test_interval_across_february_and_retry_day(self):
        last = dt.date(2027, 2, 1)
        self.assertTrue(maintenance.due(None, last))
        self.assertFalse(maintenance.due(last, dt.date(2027, 3, 3)))
        self.assertTrue(maintenance.due(last, dt.date(2027, 3, 4)))
        self.assertTrue(maintenance.due(last, dt.date(2027, 3, 5)))

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
        workload = {"sample_quality_passed": True, "metrics": {
            metric: {"decision": "passed", "repeatable_gain": False} for metric in maintenance.download.METRICS
        }}
        tunnels = {"udp/v4/http/1": workload}
        decide = maintenance.measurement_decision
        self.assertEqual(decide(framing, tunnels, True), "no demonstrated download gain")
        workload["metrics"]["cpu_ns_per_payload_byte"]["repeatable_gain"] = True
        self.assertEqual(decide(framing, tunnels, True), "passed")
        workload["sample_quality_passed"] = False
        self.assertEqual(decide(framing, tunnels, True), "measurement inconclusive")

    def test_retryable_results_do_not_consume_cycle(self):
        report = {"stages": {stage: {"status": "no change"} for stage in maintenance.STAGES}}
        self.assertTrue(maintenance.cycle_complete(report, []))
        first = {"stages": {"performance": {"decision": "operational failure"}}}
        self.assertFalse(maintenance.cycle_complete(report, [first]))
        report["stages"]["performance"]["status"] = "no verified agent output"
        self.assertFalse(maintenance.cycle_complete(report, []))

    def test_refresh_is_for_disjoint_documentation_cleanup_only(self):
        files = {"README.md"}
        self.assertTrue(maintenance.can_refresh("cleanup", files, {"src/framing.rs"}))
        for drift in (files, {"Cargo.lock"}, {"vendor/snapshot.rs"}, {".github/workflows/ci.yml"}):
            self.assertFalse(maintenance.can_refresh("cleanup", files, drift))
        self.assertFalse(maintenance.can_refresh("performance", files, {"README.md"}))
        self.assertFalse(maintenance.can_refresh("refactor", {"src/engine.rs"}, {"README.md"}))


if __name__ == "__main__":
    unittest.main()
