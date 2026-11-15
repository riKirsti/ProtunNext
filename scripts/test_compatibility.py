"""Guard the pinned coverage index; executable contracts are compiled by Cargo."""
import copy
import json
from pathlib import Path
import unittest

ROOT = Path(__file__).resolve().parent.parent
PIN = "a498c38ae6ffb0e68e7190b2831ef985d73fa6da"


def validate(index, ledger):
    if index["upstream"] != {
        "repository": "https://github.com/ProtonVPN/protun", "tag": "v3.0.5", "commit": PIN
    }:
        raise ValueError("upstream baseline changed")
    actual = [item["id"] for item in ledger]
    mapped = [key for group in index["groups"] for key in group["requirements"]]
    if len(set(actual)) != len(actual) or len(set(mapped)) != len(mapped) or set(actual) != set(mapped):
        raise ValueError("missing, duplicate or unknown capability mapping")
    for item in ledger:
        if item["source"].startswith("https://github.com/ProtonVPN/protun/") and f"/blob/{PIN}/" not in item["source"]:
            raise ValueError("unpinned upstream requirement")
    paths = [index["contracts"]]
    for group in index["groups"]:
        if not group["implementation"] or not group["checks"]:
            raise ValueError("capability lacks implementation/check mapping")
        paths.extend(group["implementation"] + group["checks"])
    for path in paths:
        target = (ROOT / path).resolve()
        if not target.is_relative_to(ROOT) or not target.is_file():
            raise ValueError("missing or outside-repository mapping")


class CompatibilityIndex(unittest.TestCase):
    def setUp(self):
        self.index = json.loads((ROOT / "tests/fixtures/compatibility.json").read_text())
        self.ledger = json.loads((ROOT / "src/requirements.json").read_text())

    def test_all_requirements_have_pinned_coverage_mappings(self):
        validate(self.index, self.ledger)

    def test_missing_duplicate_and_unknown_capabilities_are_rejected(self):
        for change in ("missing", "duplicate", "unknown"):
            index = copy.deepcopy(self.index)
            keys = index["groups"][0]["requirements"]
            if change == "missing":
                keys.pop()
            else:
                keys.append(keys[0] if change == "duplicate" else "unknown")
            with self.subTest(change=change), self.assertRaises(ValueError):
                validate(index, self.ledger)

    def test_default_branch_cannot_replace_the_release_baseline(self):
        self.index["upstream"]["commit"] = "master"
        with self.assertRaises(ValueError):
            validate(self.index, self.ledger)
        self.setUp()
        self.ledger[0]["source"] = self.ledger[0]["source"].replace(PIN, "master")
        with self.assertRaises(ValueError):
            validate(self.index, self.ledger)

    def test_missing_check_is_rejected(self):
        self.index["groups"][0]["checks"] = ["tests/missing.rs"]
        with self.assertRaises(ValueError):
            validate(self.index, self.ledger)


if __name__ == "__main__":
    unittest.main()
