import importlib.util
from pathlib import Path
import unittest

spec = importlib.util.spec_from_file_location("baseline", Path(__file__).with_name("measure-baseline.py"))
baseline = importlib.util.module_from_spec(spec)
spec.loader.exec_module(baseline)


class MeasurementValidation(unittest.TestCase):
    def test_process_metrics_use_reported_payload_and_one_core_cpu(self):
        row = baseline.metrics({"benchmark": "actual-frame-roundtrip", "bytes": 1280,
                                "packets": 1000, "seconds": 1}, 0.5, 4096, 1.25)
        self.assertEqual(row["cpu_percent_one_core"], 40)
        self.assertEqual(row["peak_rss_bytes"], 4096)
        self.assertEqual(row["framing_payload_mbps"], 10.24)
        self.assertEqual(row["cpu_ns_per_payload_byte"], 390.625)

    def test_invalid_scope_counters_and_process_measurements_are_rejected(self):
        good = {"benchmark": "actual-frame-roundtrip", "bytes": 1280, "packets": 1000, "seconds": 1}
        for field, value in [("benchmark", "VPN"), ("bytes", 0), ("bytes", 1500),
                             ("packets", 1.5), ("packets", True), ("seconds", float("nan")),
                             ("seconds", float("inf")), ("seconds", -1), ("seconds", 3)]:
            with self.subTest(field=field, value=value), self.assertRaises(ValueError):
                baseline.metrics({**good, field: value}, 1, 4096, 1.25)
        for cpu, rss, wall in [(0, 4096, 1.25), (1, -1, 1.25), (1, 4096, 0)]:
            with self.assertRaises(ValueError):
                baseline.metrics(good, cpu, rss, wall)


if __name__ == "__main__":
    unittest.main()
