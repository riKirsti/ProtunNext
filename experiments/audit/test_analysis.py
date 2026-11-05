"""Fault injection: execution success must not masquerade as useful evidence."""
import copy
import json
import pathlib
import tempfile
import unittest

from analyze import analyze
from check_codegen import check
from run import plan


class ValidityTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.folder = pathlib.Path(self.temp.name)
        self.planned = plan()[:8]  # One complete A/A + ABBA block.
        self.env = dict(kernel="6.18.60", binaries=dict(main="a", erase="b"),
                        sources={n: dict(effective_sources_sha256=n) for n in ["main", "erase"]})
        self.rows = []
        for p in self.planned:
            seconds, size, packets = 5.01, p["size"], 100000
            load = dict(effective_sources_sha256="main", shape=p["shape"], bytes=packets * size,
                        packets=packets, seconds=seconds, Mbps=packets * size * 8 / seconds / 1e6,
                        cpu_seconds=1.)
            m = dict(effective_sources_sha256=p["build"], kernel="6.18.60", protocol=p["protocol"],
                     size=size, window=p["window"], flows=p["flows"], load=load, seconds=seconds,
                     client_cpu_seconds=2., client_cpu_ns_per_byte=2e9 / load["bytes"], client_rss_kib=4096,
                     state="TunnelEstablished", peer=dict(effective_sources_sha256="main", timed=True,
                     seconds=seconds, cpu_seconds=1.), counters=dict(invalid_packets=0, outer_io_errors=0,
                     outer_eof=0, io=dict(gso_datagrams=32)))
            self.rows.append(dict(p, exit=0, metrics=[m]))

    def result(self):
        for name, value in [("environment", self.env), ("plan", self.planned)]:
            (self.folder / (name + ".json")).write_text(json.dumps(value))
        (self.folder / "samples.jsonl").write_text("".join(json.dumps(r) + "\n" for r in self.rows))
        return analyze(self.folder)

    def test_unchanged_is_usable_but_has_no_gain(self):
        r = self.result()
        self.assertFalse(r["errors"])
        self.assertEqual(r["blocks"][0]["validity"], "usable")
        self.assertAlmostEqual(r["blocks"][0]["result"]["CPU_ns_per_byte"]["change_percent"], 0)

    def test_source_identity_and_math_tampering_are_invalid(self):
        self.rows[0]["metrics"][0]["peer"]["effective_sources_sha256"] = "wrong"
        self.rows[1]["metrics"][0]["load"]["Mbps"] *= 2
        self.assertTrue(self.result()["errors"])

    def test_missing_and_duplicate_are_invalid(self):
        self.rows[-1] = copy.deepcopy(self.rows[0])
        self.assertTrue(self.result()["errors"])

    def test_noisy_control_is_inconclusive_despite_successful_execution(self):
        for row in self.rows:
            if row["role"] == "aa-b":
                m = row["metrics"][0]
                m["client_cpu_seconds"] *= 1.1
                m["client_cpu_ns_per_byte"] *= 1.1
        r = self.result()
        self.assertFalse(r["errors"])
        self.assertEqual(r["blocks"][0]["validity"], "inconclusive-control/load")

    def test_auxiliary_saturation_disqualifies_capacity_claim(self):
        self.rows[0]["metrics"][0]["peer"]["cpu_seconds"] = 5.
        b = self.result()["blocks"][0]
        self.assertEqual(b["validity"], "usable")
        self.assertFalse(b["capacity_ranking"])

    def test_missed_fixed_rate_is_inconclusive(self):
        for p, r in zip(self.planned, self.rows):
            p["rate_pps"] = r["rate_pps"] = 100000
        b = self.result()["blocks"][0]
        self.assertEqual(b["validity"], "inconclusive-control/load")


class CodegenTests(unittest.TestCase):
    def test_an_overwritten_zero_argument_cannot_pass(self):
        source = "".join(f"0000000000001000 <{name}>:\n 1000: xor %esi,%esi\n 1001: mov %ebx,%esi\n 1002: call 2000 <memset@plt>\n" for name in ["audit_drop_packet", "audit_recycle_packet"])
        with self.assertRaises(AssertionError):
            check(source)

    def test_indirect_callee_saved_memset_and_relocation(self):
        source = """0000000000001000 <audit_drop_packet>:
 1000: mov 0x1234(%rip),%r15 # 4000 <memset@GLIBC_2.2.5>
 1001: xor %esi,%esi
 1002: call *%r15
0000000000002000 <audit_recycle_packet>:
 2000: call *0x1234(%rip) # 5000 <_DYNAMIC+0x490>
0000000000003000 <production_recycle>:
 3000: xor %esi,%esi
 3001: call 4000 <memset@plt>
0000000000005000 R_X86_64_RELATIVE *ABS*+0x0000000000003000
"""
        self.assertEqual(len(check(source)), 2)

    def test_initial_nonzero_fill_cannot_pass_as_erasure(self):
        source = "".join(f"0000000000001000 <{name}>:\n 1000: mov %ebx,%esi\n 1002: call 2000 <memset@plt>\n" for name in ["audit_drop_packet", "audit_recycle_packet"])
        with self.assertRaises(AssertionError):
            check(source)

    def test_recognized_x64_and_arm64_zero_arguments(self):
        for instruction, call in [("xor %esi,%esi", "call"), ("mov w1, #0x0", "bl")]:
            source = "".join(f"0000000000001000 <{name}>:\n 1000: {instruction}\n 1002: {call} 2000 <memset@plt>\n" for name in ["audit_drop_packet", "audit_recycle_packet"])
            self.assertEqual(len(check(source)), 2)


if __name__ == "__main__":
    unittest.main()
