"""Validate coverage against an independently enumerated upstream diff."""

import copy
import unittest

import upstream_coverage as coverage


class Coverage(unittest.TestCase):
    def setUp(self):
        self.previous, self.upstream = "a" * 40, "b" * 40
        self.expected = {"upstream/new.rs", "upstream/setting.rs", "upstream/build.md"}
        self.before = {"src/old.rs", "src/config.rs", "docs/building.md", "Cargo.toml"}
        self.after = (self.before - {"src/old.rs"}) | {"src/new.rs"}
        self.modified = {"src/old.rs", "src/new.rs", "Cargo.toml"}
        self.record = {
            "from": self.previous,
            "to": self.upstream,
            "files": [
                {"path": "upstream/new.rs", "status": "ported", "reason": "Replace the old hook.",
                 "evidence": ["src/old.rs", "src/new.rs"]},
                {"path": "upstream/setting.rs", "status": "already-covered",
                 "reason": "The setting is already handled.", "evidence": ["src/config.rs"]},
                {"path": "upstream/build.md", "status": "not-applicable",
                 "reason": "Our build guide documents the distinct build.", "evidence": ["docs/building.md"]},
            ],
        }

    def validate(self, record):
        return coverage.validate(record, self.previous, self.upstream, self.expected,
                                 self.before, self.after, self.modified)

    def test_accepts_exact_file_inventory_with_tracked_evidence(self):
        self.assertFalse(self.validate(self.record))

    def test_rejects_missing_duplicate_unknown_and_unresolved_files(self):
        for action in ("remove", "duplicate", "unknown", "unresolved"):
            with self.subTest(action=action):
                record = copy.deepcopy(self.record)
                rows = record["files"]
                if action == "remove":
                    rows.pop()
                elif action == "duplicate":
                    rows.append(rows[0].copy())
                elif action == "unknown":
                    rows[0]["path"] = "upstream/unknown.rs"
                else:
                    rows[0]["status"] = "unresolved"
                self.assertTrue(self.validate(record))

    def test_rejects_metadata_nonexistent_or_unchanged_evidence_for_ported_change(self):
        for ref in ("Cargo.toml", coverage.REPORT.as_posix(), "src/config.rs", "src/missing.rs"):
            with self.subTest(ref=ref):
                record = copy.deepcopy(self.record)
                record["files"][0]["evidence"] = [ref]
                self.assertTrue(self.validate(record))

    def test_rejects_a_record_for_a_different_upstream_comparison(self):
        self.assertTrue(self.validate({**self.record, "to": "c" * 40}))
        self.assertTrue(self.validate(None))


if __name__ == "__main__":
    unittest.main()
