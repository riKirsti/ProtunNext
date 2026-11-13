import tempfile
import unittest
from pathlib import Path

from check_repository import link_findings, privacy_findings


class RepositoryPrivacy(unittest.TestCase):
    def test_realistic_local_paths_and_contact_addresses_are_rejected(self):
        for text in [
            "Q:" + "\\Users\\" + "developer\\project",
            "/Users/" + "developer/project",
            "/home/" + "developer/project",
            "developer" + "@" + "mail.invalid",
        ]:
            with self.subTest(text=text):
                self.assertTrue(privacy_findings(text))

    def test_synthetic_contact_and_portable_paths_are_allowed(self):
        self.assertFalse(privacy_findings("test@example.com /path/to/config $HOME/cache"))

    def test_links_distinguish_local_files_and_external_urls(self):
        with tempfile.TemporaryDirectory() as folder:
            root = Path(folder).resolve()
            page = root / "README.md"
            (root / "guide.md").write_text("# Guide", encoding="utf-8")
            self.assertFalse(link_findings(page, "[guide](guide.md#guide) [web](https://example.com)", root))
            self.assertTrue(link_findings(page, "[missing](missing.md)", root))
            self.assertTrue(link_findings(page, "[outside](../outside.md)", root))


if __name__ == "__main__":
    unittest.main()
