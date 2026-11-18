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

    def test_measurements_require_repeatable_gain_and_pass_all_guardrails(self):
        baseline = {metric: 100 for metric in maintenance.METRICS}
        control = [(baseline, baseline)] * 4
        faster = {**baseline, "framing_payload_mbps": 110}
        costlier = {**faster, "peak_rss_bytes": 110}
        noisy = {**baseline, "framing_payload_mbps": 110}
        self.assertTrue(maintenance.compare_samples(control, [(baseline, faster)] * 8, True)[0])
        self.assertFalse(maintenance.compare_samples(control, [(baseline, costlier)] * 8, True)[0])
        self.assertFalse(maintenance.compare_samples([(baseline, noisy)] * 4,
                                                      [(baseline, faster)] * 8, True)[0])
        self.assertFalse(maintenance.compare_samples(control, [(baseline, faster)] * 7 +
                                                      [(baseline, baseline)], True)[0])
        self.assertTrue(maintenance.compare_samples(control, [(baseline, baseline)] * 8, False)[0])


if __name__ == "__main__":
    unittest.main()
