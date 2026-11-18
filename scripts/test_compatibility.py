"""Guard compatibility metadata and contract integrity; Cargo executes consumers."""
import copy
import json
import unittest
from unittest.mock import patch

from compatibility import ROOT, validate, validate_checkout, validate_versions, verify_tag


class CompatibilityIndex(unittest.TestCase):
    def setUp(self):
        self.index = json.loads((ROOT / "tests/fixtures/compatibility.json").read_text())
        self.ledger = json.loads((ROOT / "src/requirements.json").read_text())

    def test_all_requirements_have_pinned_coverage_mappings(self):
        validate(self.index, self.ledger)
        validate_checkout()

    def test_replaced_or_unpinned_contract_is_rejected(self):
        original = (ROOT / self.index["contracts"]).read_text(encoding="utf-8")
        for text in ("", original + "\n// contract edited\n"):
            with self.subTest(contract=text[:20]), self.assertRaises(ValueError):
                validate(self.index, self.ledger, text)
        for checksum in (None, "0" * 64):
            index = copy.deepcopy(self.index)
            if checksum is None:
                index.pop("contracts_sha256")
            else:
                index["contracts_sha256"] = checksum
            with self.subTest(checksum=checksum), self.assertRaises(ValueError):
                validate(index, self.ledger)

    def test_host_line_endings_do_not_change_contract_identity(self):
        original = (ROOT / self.index["contracts"]).read_text(encoding="utf-8")
        validate(self.index, self.ledger, original)
        validate(self.index, self.ledger, original.replace("\n", "\r\n"))

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
        self.ledger[0]["source"] = self.ledger[0]["source"].replace(self.index["upstream"]["commit"], "master")
        with self.assertRaises(ValueError):
            validate(self.index, self.ledger)

    def test_future_adoption_requires_matching_package_and_lock_versions(self):
        upstream = {**self.index["upstream"], "tag": "v99.1.2", "commit": "b" * 40}
        package = {"name": "protun-next", "version": "99.1.2"}
        validate_versions(upstream, package, [package])
        with self.assertRaisesRegex(ValueError, "must agree"):
            validate_versions(upstream, package, [{**package, "version": "99.1.1"}])
        with self.assertRaisesRegex(ValueError, "must agree"):
            validate_versions({**upstream, "tag": "v99.1.1"}, package, [package])

    def test_published_annotated_tag_is_checked_at_its_peeled_commit(self):
        upstream = self.index["upstream"]
        ref = "refs/tags/" + upstream["tag"]
        object_id = "f" * 40
        with patch("compatibility.subprocess.check_output", return_value=(
            f"{object_id}\t{ref}\n{upstream['commit']}\t{ref}^{{}}\n"
        )):
            verify_tag(upstream)
        with patch("compatibility.subprocess.check_output", return_value=f"{object_id}\t{ref}\n"):
            with self.assertRaisesRegex(ValueError, "published upstream tag"):
                verify_tag(upstream)

    def test_missing_check_is_rejected(self):
        self.index["groups"][0]["checks"] = ["tests/missing.rs"]
        with self.assertRaises(ValueError):
            validate(self.index, self.ledger)


if __name__ == "__main__":
    unittest.main()
